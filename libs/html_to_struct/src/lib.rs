use html5ever::tendril::TendrilSink;
use html5ever::{
    interface::TreeSink, parse_document, tree_builder::TreeBuilderOpts, ExpandedName, ParseOpts,
};

extern crate markup5ever;
extern crate tendril;

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::{HashSet, VecDeque};
use std::default::Default;
use std::fmt::{self, Debug};
use std::io;
use std::mem;
use std::rc::{Rc, Weak};

use tendril::StrTendril;

use markup5ever::interface::tree_builder;
use markup5ever::interface::tree_builder::{ElementFlags, NodeOrText, QuirksMode};
use markup5ever::serialize::TraversalScope;
use markup5ever::serialize::TraversalScope::{ChildrenOnly, IncludeNode};
use markup5ever::serialize::{Serialize, Serializer};
use markup5ever::Attribute;
use markup5ever::QualName;

/// The different kinds of nodes in the DOM.
#[derive(Debug)]
pub enum NodeData {
    /// The `Document` itself - the root node of a HTML document.
    Document,

    /// A `DOCTYPE` with name, public id, and system id. See
    /// [document type declaration on wikipedia][dtd wiki].
    ///
    /// [dtd wiki]: https://en.wikipedia.org/wiki/Document_type_declaration
    Doctype {
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    },

    /// A text node.
    Text { contents: RefCell<StrTendril> },

    /// A comment.
    Comment { contents: StrTendril },

    /// An element with attributes.
    Element {
        name: QualName,
        attrs: RefCell<Vec<Attribute>>,

        /// For HTML \<template\> elements, the [template contents].
        ///
        /// [template contents]: https://html.spec.whatwg.org/multipage/#template-contents
        template_contents: RefCell<Option<Handle>>,

        /// Whether the node is a [HTML integration point].
        ///
        /// [HTML integration point]: https://html.spec.whatwg.org/multipage/#html-integration-point
        mathml_annotation_xml_integration_point: bool,
    },

    /// A Processing instruction.
    ProcessingInstruction {
        target: StrTendril,
        contents: StrTendril,
    },
}

/// A DOM node.
pub struct Node {
    /// Parent node.
    pub parent: Cell<Option<WeakHandle>>,
    /// Child nodes of this node.
    pub children: RefCell<Vec<Handle>>,
    /// Represents this node's data.
    pub data: NodeData,
}

impl Node {
    /// Create a new node from its contents
    pub fn new(data: NodeData) -> Rc<Self> {
        Rc::new(Node {
            data,
            parent: Cell::new(None),
            children: RefCell::new(Vec::new()),
        })
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let mut nodes = mem::take(&mut *self.children.borrow_mut());
        while let Some(node) = nodes.pop() {
            let children = mem::take(&mut *node.children.borrow_mut());
            nodes.extend(children.into_iter());
            if let NodeData::Element {
                ref template_contents,
                ..
            } = node.data
            {
                if let Some(template_contents) = template_contents.borrow_mut().take() {
                    nodes.push(template_contents);
                }
            }
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Node")
            .field("data", &self.data)
            .field("children", &self.children)
            .finish()
    }
}

/// Reference to a DOM node.
pub type Handle = Rc<Node>;

/// Weak reference to a DOM node, used for parent pointers.
pub type WeakHandle = Weak<Node>;

/// Append a parentless node to another nodes' children
fn append(new_parent: &Handle, child: Handle) {
    let previous_parent = child.parent.replace(Some(Rc::downgrade(new_parent)));
    // Invariant: child cannot have existing parent
    assert!(previous_parent.is_none());
    new_parent.children.borrow_mut().push(child);
}

/// If the node has a parent, get it and this node's position in its children
fn get_parent_and_index(target: &Handle) -> Option<(Handle, usize)> {
    if let Some(weak) = target.parent.take() {
        let parent = weak.upgrade().expect("dangling weak pointer");
        target.parent.set(Some(weak));
        let i = match parent
            .children
            .borrow()
            .iter()
            .enumerate()
            .find(|&(_, child)| Rc::ptr_eq(child, target))
        {
            Some((i, _)) => i,
            None => panic!("have parent but couldn't find in parent's children!"),
        };
        Some((parent, i))
    } else {
        None
    }
}

fn append_to_existing_text(prev: &Handle, text: &str) -> bool {
    match prev.data {
        NodeData::Text { ref contents } => {
            contents.borrow_mut().push_slice(text);
            true
        }
        _ => false,
    }
}

fn remove_from_parent(target: &Handle) {
    if let Some((parent, i)) = get_parent_and_index(target) {
        parent.children.borrow_mut().remove(i);
        target.parent.set(None);
    }
}

/// The DOM itself; the result of parsing.

#[derive(Debug)]
pub struct RcDom {
    /// The `Document` itself.
    pub document: Handle,

    /// Errors that occurred during parsing.
    pub errors: RefCell<Vec<Cow<'static, str>>>,

    /// The document's quirks mode.
    pub quirks_mode: Cell<QuirksMode>,
}

impl TreeSink for RcDom {
    type Output = Self;
    fn finish(self) -> Self {
        self
    }

    type Handle = Handle;

    type ElemName<'a>
        = ExpandedName<'a>
    where
        Self: 'a;

    fn parse_error(&self, msg: Cow<'static, str>) {
        self.errors.borrow_mut().push(msg);
    }

    fn get_document(&self) -> Handle {
        self.document.clone()
    }

    fn get_template_contents(&self, target: &Handle) -> Handle {
        if let NodeData::Element {
            ref template_contents,
            ..
        } = target.data
        {
            template_contents
                .borrow()
                .as_ref()
                .expect("not a template element!")
                .clone()
        } else {
            panic!("not a template element!")
        }
    }

    fn set_quirks_mode(&self, mode: QuirksMode) {
        self.quirks_mode.set(mode);
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        Rc::ptr_eq(x, y)
    }

    fn elem_name<'a>(&self, target: &'a Handle) -> ExpandedName<'a> {
        match target.data {
            NodeData::Element { ref name, .. } => name.expanded(),
            _ => panic!("not an element!"),
        }
    }

    fn create_element(&self, name: QualName, attrs: Vec<Attribute>, flags: ElementFlags) -> Handle {
        Node::new(NodeData::Element {
            name,
            attrs: RefCell::new(attrs),
            template_contents: RefCell::new(if flags.template {
                Some(Node::new(NodeData::Document))
            } else {
                None
            }),
            mathml_annotation_xml_integration_point: flags.mathml_annotation_xml_integration_point,
        })
    }

    fn create_comment(&self, text: StrTendril) -> Handle {
        Node::new(NodeData::Comment { contents: text })
    }

    fn create_pi(&self, target: StrTendril, data: StrTendril) -> Handle {
        Node::new(NodeData::ProcessingInstruction {
            target,
            contents: data,
        })
    }

    fn append(&self, parent: &Handle, child: NodeOrText<Handle>) {
        // Append to an existing Text node if we have one.
        if let NodeOrText::AppendText(text) = &child {
            if let Some(h) = parent.children.borrow().last() {
                if append_to_existing_text(h, text) {
                    return;
                }
            }
        }

        append(
            parent,
            match child {
                NodeOrText::AppendText(text) => Node::new(NodeData::Text {
                    contents: RefCell::new(text),
                }),
                NodeOrText::AppendNode(node) => node,
            },
        );
    }

    fn append_before_sibling(&self, sibling: &Handle, child: NodeOrText<Handle>) {
        let (parent, i) = get_parent_and_index(sibling)
            .expect("append_before_sibling called on node without parent");

        let child = match (child, i) {
            // No previous node.
            (NodeOrText::AppendText(text), 0) => Node::new(NodeData::Text {
                contents: RefCell::new(text),
            }),

            // Look for a text node before the insertion point.
            (NodeOrText::AppendText(text), i) => {
                let children = parent.children.borrow();
                let prev = &children[i - 1];
                if append_to_existing_text(prev, &text) {
                    return;
                }
                Node::new(NodeData::Text {
                    contents: RefCell::new(text),
                })
            }

            // The tree builder promises we won't have a text node after
            // the insertion point.

            // Any other kind of node.
            (NodeOrText::AppendNode(node), _) => node,
        };

        remove_from_parent(&child);

        child.parent.set(Some(Rc::downgrade(&parent)));
        parent.children.borrow_mut().insert(i, child);
    }

    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        let parent = element.parent.take();
        let has_parent = parent.is_some();
        element.parent.set(parent);

        if has_parent {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        append(
            &self.document,
            Node::new(NodeData::Doctype {
                name,
                public_id,
                system_id,
            }),
        );
    }

    fn add_attrs_if_missing(&self, target: &Handle, attrs: Vec<Attribute>) {
        let mut existing = if let NodeData::Element { ref attrs, .. } = target.data {
            attrs.borrow_mut()
        } else {
            panic!("not an element")
        };

        let existing_names = existing
            .iter()
            .map(|e| e.name.clone())
            .collect::<HashSet<_>>();
        existing.extend(
            attrs
                .into_iter()
                .filter(|attr| !existing_names.contains(&attr.name)),
        );
    }

    fn remove_from_parent(&self, target: &Handle) {
        remove_from_parent(target);
    }

    fn reparent_children(&self, node: &Handle, new_parent: &Handle) {
        let mut children = node.children.borrow_mut();
        let mut new_children = new_parent.children.borrow_mut();
        for child in children.iter() {
            let previous_parent = child.parent.replace(Some(Rc::downgrade(new_parent)));
            assert!(Rc::ptr_eq(
                node,
                &previous_parent.unwrap().upgrade().expect("dangling weak")
            ))
        }
        new_children.extend(mem::take(&mut *children));
    }

    fn is_mathml_annotation_xml_integration_point(&self, target: &Handle) -> bool {
        if let NodeData::Element {
            mathml_annotation_xml_integration_point,
            ..
        } = target.data
        {
            mathml_annotation_xml_integration_point
        } else {
            panic!("not an element!")
        }
    }
}

impl Default for RcDom {
    fn default() -> RcDom {
        RcDom {
            document: Node::new(NodeData::Document),
            errors: Default::default(),
            quirks_mode: Cell::new(tree_builder::NoQuirks),
        }
    }
}

enum SerializeOp {
    Open(Handle),
    Close(QualName),
}

pub struct SerializableHandle(Handle);

impl From<Handle> for SerializableHandle {
    fn from(h: Handle) -> SerializableHandle {
        SerializableHandle(h)
    }
}

impl Serialize for SerializableHandle {
    fn serialize<S>(&self, serializer: &mut S, traversal_scope: TraversalScope) -> io::Result<()>
    where
        S: Serializer,
    {
        let mut ops = VecDeque::new();
        match traversal_scope {
            IncludeNode => ops.push_back(SerializeOp::Open(self.0.clone())),
            ChildrenOnly(_) => ops.extend(
                self.0
                    .children
                    .borrow()
                    .iter()
                    .map(|h| SerializeOp::Open(h.clone())),
            ),
        }

        while let Some(op) = ops.pop_front() {
            match op {
                SerializeOp::Open(handle) => match handle.data {
                    NodeData::Element {
                        ref name,
                        ref attrs,
                        ..
                    } => {
                        serializer.start_elem(
                            name.clone(),
                            attrs.borrow().iter().map(|at| (&at.name, &at.value[..])),
                        )?;

                        ops.reserve(1 + handle.children.borrow().len());
                        ops.push_front(SerializeOp::Close(name.clone()));

                        for child in handle.children.borrow().iter().rev() {
                            ops.push_front(SerializeOp::Open(child.clone()));
                        }
                    }

                    NodeData::Doctype { ref name, .. } => serializer.write_doctype(name)?,

                    NodeData::Text { ref contents } => {
                        serializer.write_text(&contents.borrow().to_string())?
                    }

                    NodeData::Comment { ref contents } => serializer.write_comment(contents)?,

                    NodeData::ProcessingInstruction {
                        ref target,
                        ref contents,
                    } => serializer.write_processing_instruction(target, contents)?,

                    NodeData::Document => panic!("Can't serialize Document node itself"),
                },

                SerializeOp::Close(name) => {
                    serializer.end_elem(name)?;
                }
            }
        }

        Ok(())
    }
}

pub fn process(string: &str) -> SpellDescriptionStruct {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ignore_missing_rules: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut string.as_bytes())
        .unwrap();

    process_node(&dom.document).unwrap()
}

#[derive(Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum SpellDescriptionStruct {
    Body(Vec<SpellDescriptionStruct>),
    Paragraph(Vec<SpellDescriptionStruct>),
    Caption(Vec<SpellDescriptionStruct>),
    Italics(String),
    Bold(String),
    Sup(Vec<SpellDescriptionStruct>),
    Br,
    Listing(Vec<SpellDescriptionStruct>),
    Line(Vec<SpellDescriptionStruct>),
    Table(Vec<SpellDescriptionStruct>),
    Tbody(Vec<SpellDescriptionStruct>),
    Thead(Vec<SpellDescriptionStruct>),
    Tfoot(Vec<SpellDescriptionStruct>),
    Row(Vec<SpellDescriptionStruct>),
    Header(Vec<SpellDescriptionStruct>),
    Cell(Vec<SpellDescriptionStruct>),
    Text(String),
}

impl Debug for SpellDescriptionStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Body(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Body")
                .field(arg0)
                .finish(),
            Self::Paragraph(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Paragraph")
                .field(arg0)
                .finish(),
            Self::Caption(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Caption")
                .field(arg0)
                .finish(),
            Self::Italics(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Italics")
                .field(arg0)
                .finish(),
            Self::Bold(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Bold")
                .field(arg0)
                .finish(),
            Self::Sup(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Sup")
                .field(arg0)
                .finish(),
            Self::Br => write!(f, "SpellDescriptionStruct::Br"),
            Self::Listing(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Listing")
                .field(arg0)
                .finish(),
            Self::Line(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Line")
                .field(arg0)
                .finish(),
            Self::Table(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Table")
                .field(arg0)
                .finish(),
            Self::Tbody(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Tbody")
                .field(arg0)
                .finish(),
            Self::Thead(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Thead")
                .field(arg0)
                .finish(),
            Self::Tfoot(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Tfoot")
                .field(arg0)
                .finish(),
            Self::Row(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Row")
                .field(arg0)
                .finish(),
            Self::Header(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Header")
                .field(arg0)
                .finish(),
            Self::Cell(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Cell")
                .field(arg0)
                .finish(),
            Self::Text(arg0) => f
                .debug_tuple("SpellDescriptionStruct::Text")
                .field(arg0)
                .finish(),
        }
    }
}

fn process_node(n: &Rc<Node>) -> Option<SpellDescriptionStruct> {
    match &n.data {
        NodeData::Document => process_node(n.children.take().first().unwrap()),
        NodeData::Doctype {
            name: _,
            public_id: _,
            system_id: _,
        } => {
            panic!("Doctype")
        }
        NodeData::Text { contents } => {
            let s = contents.take().to_string();
            if !s.trim().is_empty() {
                Some(SpellDescriptionStruct::Text(s))
            } else {
                None
            }
        }
        NodeData::Comment { contents: _ } => {
            panic!("Comment")
        }
        NodeData::Element {
            name,
            attrs: _,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => Some(match name.local.to_string().as_str() {
            "body" => SpellDescriptionStruct::Body(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "p" => SpellDescriptionStruct::Paragraph(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "i" => {
                if let NodeData::Text { contents: s } = &n.children.take().first().unwrap().data {
                    SpellDescriptionStruct::Italics(s.take().to_string())
                } else {
                    SpellDescriptionStruct::Italics("".to_string())
                }
            }
            "b" => {
                if let NodeData::Text { contents: s } = &n.children.take().first().unwrap().data {
                    SpellDescriptionStruct::Bold(s.take().to_string())
                } else {
                    SpellDescriptionStruct::Bold("".to_string())
                }
            }
            "ul" => SpellDescriptionStruct::Listing(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "li" => SpellDescriptionStruct::Line(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "html" => process_node(&n.children.take().get(1).unwrap()).unwrap(),
            "table" => SpellDescriptionStruct::Table(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "tbody" => SpellDescriptionStruct::Tbody(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "thead" => SpellDescriptionStruct::Thead(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "tfoot" => SpellDescriptionStruct::Tfoot(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "tr" => SpellDescriptionStruct::Row(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "th" => SpellDescriptionStruct::Header(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "td" => SpellDescriptionStruct::Cell(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "caption" => SpellDescriptionStruct::Caption(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "sup" => SpellDescriptionStruct::Sup(
                n.children
                    .take()
                    .iter()
                    .filter_map(|e| process_node(e))
                    .collect(),
            ),
            "br" => SpellDescriptionStruct::Br,
            "head" | _ => {
                panic!("What? {:?} {:?}", name.local.to_string(), n.children)
            }
        }),
        NodeData::ProcessingInstruction {
            target: _,
            contents: _,
        } => {
            panic!("ProcessingInstruction")
        }
    }
}
