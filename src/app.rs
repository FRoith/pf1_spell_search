use egui::RichText;
use egui_extras::{Column, TableBuilder, TableRow};
use filter_repr::FilterRepr;

use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::{
    filter_row,
    filters::{
        Domain, Level, Save, SpellComponent, SpellDescriptor, SpellRange, SpellResistance,
        SpellSource, Spellschool, Subschool,
    },
    spell::{ClassType, Spell, SpellMeta, BONUS_INFO},
    util::{html2egui, toggle},
};

fn _vec_from_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(",").map(|x| x.to_string()).collect())
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub enum RowOrder {
    None,
    Ascending,
    Descending,
}

impl RowOrder {
    fn n(&self) -> Self {
        match self {
            Self::None => Self::Ascending,
            Self::Ascending => Self::Descending,
            Self::Descending => Self::None,
        }
    }

    fn p(&self) -> Self {
        match self {
            Self::None => Self::Descending,
            Self::Ascending => Self::None,
            Self::Descending => Self::Ascending,
        }
    }

    fn compare(
        &self,
        col: &ColType,
        a: &Spell,
        b: &Spell,
        c: &String,
        d: &String,
    ) -> std::cmp::Ordering {
        match (self, col) {
            (Self::Ascending, ColType::Name(_)) => a.name.cmp(&b.name),
            (Self::Ascending, ColType::School(_)) => a.school.cmp(&b.school),
            (Self::Ascending, ColType::Level(_)) => c.cmp(d),
            (Self::Ascending, ColType::Subschools(_)) => a.subschool.cmp(&b.subschool),
            (Self::Ascending, ColType::Domain(_)) => a.domain.cmp(&b.domain),
            (Self::Ascending, ColType::Descriptors(_)) => a.descriptors.cmp(&b.descriptors),
            (Self::Ascending, ColType::Components(_)) => a.components.cmp(&b.components),
            (Self::Ascending, ColType::Range(_)) => a.range.cmp(&b.range),
            (Self::Ascending, ColType::Area(_)) => a.area.cmp(&b.area),
            (Self::Ascending, ColType::Effect(_)) => a.effect.cmp(&b.effect),
            (Self::Ascending, ColType::Targets(_)) => a.targets.cmp(&b.targets),
            (Self::Ascending, ColType::Duration(_)) => a.duration.cmp(&b.duration),
            (Self::Ascending, ColType::SavingThrow(_)) => a.saving_throw.cmp(&b.duration),
            (Self::Ascending, ColType::SpellResistance(_)) => {
                a.spell_resistance.cmp(&b.spell_resistance)
            }
            (Self::Ascending, ColType::Description(_)) => a.description.cmp(&b.description),
            (Self::Ascending, ColType::Source(_)) => a.source.cmp(&b.source),
            (Self::Descending, ColType::Name(_)) => b.name.cmp(&a.name),
            (Self::Descending, ColType::School(_)) => b.school.cmp(&a.school),
            (Self::Descending, ColType::Level(_)) => d.cmp(c),
            (Self::Descending, ColType::Subschools(_)) => b.subschool.cmp(&a.subschool),
            (Self::Descending, ColType::Domain(_)) => b.domain.cmp(&a.domain),
            (Self::Descending, ColType::Descriptors(_)) => b.descriptors.cmp(&a.descriptors),
            (Self::Descending, ColType::Components(_)) => b.components.cmp(&a.components),
            (Self::Descending, ColType::Range(_)) => b.range.cmp(&a.range),
            (Self::Descending, ColType::Area(_)) => b.area.cmp(&a.area),
            (Self::Descending, ColType::Effect(_)) => b.effect.cmp(&a.effect),
            (Self::Descending, ColType::Targets(_)) => b.targets.cmp(&a.targets),
            (Self::Descending, ColType::Duration(_)) => b.duration.cmp(&a.duration),
            (Self::Descending, ColType::SavingThrow(_)) => b.saving_throw.cmp(&a.saving_throw),
            (Self::Descending, ColType::SpellResistance(_)) => {
                b.spell_resistance.cmp(&a.spell_resistance)
            }
            (Self::Descending, ColType::Description(_)) => b.description.cmp(&a.description),
            (Self::Descending, ColType::Source(_)) => b.source.cmp(&a.source),
            (_, _) => std::cmp::Ordering::Equal,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub enum ColType {
    None,
    Name(bool),
    School(bool),
    Level(bool),
    Subschools(bool),
    Domain(bool),
    Descriptors(bool),
    Components(bool),
    Range(bool),
    Area(bool),
    Effect(bool),
    Targets(bool),
    Duration(bool),
    SavingThrow(bool),
    SpellResistance(bool),
    Description(bool),
    Source(bool),
}

impl ColType {
    fn title(&self) -> String {
        match self {
            Self::None => "",
            Self::Name(_) => "Name",
            Self::School(_) => "Spellschool",
            Self::Level(_) => "Level",
            Self::Subschools(_) => "Subschools",
            Self::Domain(_) => "Domain",
            Self::Descriptors(_) => "Descriptor",
            Self::Components(_) => "Components",
            Self::Range(_) => "Range",
            Self::Area(_) => "Area",
            Self::Effect(_) => "Effect",
            Self::Targets(_) => "Targets",
            Self::Duration(_) => "Duration",
            Self::SavingThrow(_) => "Saving Throw",
            Self::SpellResistance(_) => "Spell Resistance",
            Self::Description(_) => "Description",
            Self::Source(_) => "Source",
        }
        .to_string()
    }

    fn get_all() -> Vec<(Self, RowOrder)> {
        [
            (Self::Name(true), RowOrder::None),
            (Self::School(true), RowOrder::None),
            (Self::Level(true), RowOrder::None),
            (Self::Subschools(false), RowOrder::None),
            (Self::Domain(false), RowOrder::None),
            (Self::Descriptors(false), RowOrder::None),
            (Self::Components(false), RowOrder::None),
            (Self::Range(false), RowOrder::None),
            (Self::Area(false), RowOrder::None),
            (Self::Effect(false), RowOrder::None),
            (Self::Targets(false), RowOrder::None),
            (Self::Duration(false), RowOrder::None),
            (Self::SavingThrow(false), RowOrder::None),
            (Self::SpellResistance(false), RowOrder::None),
            (Self::Description(true), RowOrder::None),
            (Self::Source(false), RowOrder::None),
        ]
        .into()
    }

    fn get_bool(&self) -> bool {
        *match self {
            Self::None => &false,
            Self::Name(b) => b,
            Self::School(b) => b,
            Self::Level(b) => b,
            Self::Subschools(b) => b,
            Self::Domain(b) => b,
            Self::Descriptors(b) => b,
            Self::Components(b) => b,
            Self::Range(b) => b,
            Self::Area(b) => b,
            Self::Effect(b) => b,
            Self::Targets(b) => b,
            Self::Duration(b) => b,
            Self::SavingThrow(b) => b,
            Self::SpellResistance(b) => b,
            Self::Description(b) => b,
            Self::Source(b) => b,
        }
    }

    fn get_bool_mut(&mut self) -> &mut bool {
        match self {
            Self::None => panic!("should never happen"),
            Self::Name(ref mut b) => b,
            Self::School(ref mut b) => b,
            Self::Level(ref mut b) => b,
            Self::Subschools(ref mut b) => b,
            Self::Domain(ref mut b) => b,
            Self::Descriptors(ref mut b) => b,
            Self::Components(ref mut b) => b,
            Self::Range(ref mut b) => b,
            Self::Area(ref mut b) => b,
            Self::Effect(ref mut b) => b,
            Self::Targets(ref mut b) => b,
            Self::Duration(ref mut b) => b,
            Self::SavingThrow(ref mut b) => b,
            Self::SpellResistance(ref mut b) => b,
            Self::Description(ref mut b) => b,
            Self::Source(ref mut b) => b,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SpellSearchApp {
    spell_table: SpellTable,
    #[serde(skip)]
    filter_window_active: bool,
    #[serde(skip)]
    source_window_active: bool,
}

impl Default for SpellSearchApp {
    fn default() -> Self {
        Self {
            spell_table: SpellTable::new(),
            filter_window_active: false,
            source_window_active: false,
        }
    }
}

impl SpellSearchApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SpellSearchApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                ui.menu_button("Columns", |ui| {
                    for (col, _) in &mut self.spell_table.shown_columns {
                        let title = col.title();
                        ui.checkbox(col.get_bool_mut(), title);
                    }
                });

                if ui.button("Filters").clicked() {
                    self.filter_window_active = !self.filter_window_active;
                };

                if ui.button("Sources").clicked() {
                    self.source_window_active = !self.source_window_active;
                };

                if self.filter_window_active {
                    self.spell_table
                        .filter_window
                        .filter_ui(ctx, &mut self.filter_window_active);
                }

                if self.source_window_active {
                    self.spell_table
                        .source_window
                        .filter_ui(ctx, &mut self.source_window_active);
                }

                if self.spell_table.spell_window_active
                    && self.spell_table.selected_spell_window.is_some()
                {
                    let r = self.spell_table.spell_window.spell_ui(
                        ctx,
                        &mut self.spell_table.spell_window_active,
                        &mut self.spell_table.selected_spell_window.as_mut().unwrap(),
                    );
                    match r {
                        Some((new_spell, true)) => {
                            if let Some(old_spell) = &self.spell_table.selected_spell_window {
                                if old_spell.id == new_spell.id {
                                    self.spell_table.selected_spell_window = None;
                                } else {
                                    self.spell_table.selected_spell_window = Some(new_spell);
                                }
                            } else {
                                self.spell_table.selected_spell_window = Some(new_spell);
                            }
                        }
                        Some((new_spell, false)) => {
                            if let Some(old_spell) = &self.spell_table.selected_spell {
                                if old_spell.id == new_spell.id {
                                    self.spell_table.selected_spell = None;
                                } else {
                                    self.spell_table.selected_spell = Some(new_spell);
                                }
                            } else {
                                self.spell_table.selected_spell = Some(new_spell);
                            }
                        }
                        None => {}
                    }
                }

                egui::widgets::global_theme_preference_buttons(ui);

                ui.hyperlink_to(" Github", "https://github.com/FRoith/pf1_spell_search");
            });
        });

        if self.spell_table.shown_value.is_none() {
            self.spell_table.filter_window.keywords = self
                .spell_table
                .filter_window
                .description
                .to_lowercase()
                .split(",")
                .map(|x| Regex::new(&format!("\\b{}\\b", x)))
                .collect();
            self.spell_table.filter_window.filters_changed = true;
        }

        self.spell_table.selected_ui(ctx);

        self.spell_table.table_ui(ctx);

        if self.spell_table.filter_window.description
            != self.spell_table.filter_window.prev_description
        {
            self.spell_table.filter_window.keywords = self
                .spell_table
                .filter_window
                .description
                .to_lowercase()
                .split(",")
                .map(|x| Regex::new(&format!("\\b{}\\b", x)))
                .collect();
            self.spell_table.filter_window.filters_changed = true;
        }
        self.spell_table.filter_window.prev_description =
            self.spell_table.filter_window.description.clone();

        if self.spell_table.update_filters() {
            ctx.request_repaint();
        }
        //if undo || redo {
        //    panic!("{:?}", (undo, redo))
        //}
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SpellTable {
    #[serde(skip, default)]
    // This how you opt-out of serialization of a field
    shown_value: Option<Vec<(&'static Spell, String)>>,

    shown_columns: Vec<(ColType, RowOrder)>,

    filter_string: String,
    selected_spell: Option<Spell>,
    selected_spell_window: Option<Spell>,
    spell_window_active: bool,
    filter_window: FilterWindow,
    source_window: SourceWindow,
    spell_window: SpellWindow,
}

impl SpellTable {
    fn new() -> Self {
        let shown_columns: Vec<(ColType, RowOrder)> = ColType::get_all();
        Self {
            shown_value: None,
            shown_columns,
            filter_string: String::new(),
            selected_spell: None,
            selected_spell_window: None,
            spell_window_active: false,
            filter_window: FilterWindow::new(),
            source_window: SourceWindow::new(),
            spell_window: SpellWindow::new(),
        }
    }

    fn table_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and BottomPanel's
            let s: &mut egui::Style = ui.style_mut();
            s.wrap_mode = Some(egui::TextWrapMode::Extend);

            TableBuilder::new(ui)
                .auto_shrink(false)
                .sense(egui::Sense::click())
                .striped(true)
                .columns(
                    Column::auto().resizable(true),
                    self.shown_columns
                        .iter()
                        .filter(|(x, _)| x.get_bool())
                        .count()
                        - 1,
                )
                .column(Column::remainder())
                .header(20.0, |mut header: TableRow<'_, '_>| {
                    self.render_header(&mut header);
                })
                .body(|body| {
                    self.render_body(body, ctx);
                });
        });
    }

    fn render_header(&mut self, header: &mut egui_extras::TableRow<'_, '_>) {
        let mut clicked = ColType::None;
        for (col, order) in &mut self.shown_columns {
            if col.get_bool() {
                header.col(|ui| {
                    let resp = ui
                        .horizontal(|ui| {
                            let btn = match order {
                                RowOrder::None => egui::Button::new("⏵"),
                                RowOrder::Ascending => {
                                    egui::Button::new("⏷").fill(ui.visuals().selection.bg_fill)
                                }
                                RowOrder::Descending => {
                                    egui::Button::new("⏶").fill(ui.visuals().selection.bg_fill)
                                }
                            };
                            let resp = ui.add(btn);
                            ui.add(
                                egui::Label::new(RichText::new(col.title()).heading())
                                    .selectable(false),
                            );

                            if clicked != ColType::None {
                                *order = RowOrder::None;
                            }
                            if resp.clicked() {
                                *order = order.n();
                                clicked = col.clone();
                            }
                            if resp.secondary_clicked() {
                                *order = order.p();
                                clicked = col.clone();
                            }
                        })
                        .response;
                    match col {
                        ColType::None => {}
                        ColType::Name(_) => {
                            resp.context_menu(|ui| {
                                if ui
                                    .add(
                                        egui::text_edit::TextEdit::singleline(
                                            &mut self.filter_window.name,
                                        )
                                        .hint_text("case insensitive search"),
                                    )
                                    .changed()
                                {
                                    self.filter_window.filters_changed = true;
                                };
                            });
                        }
                        ColType::School(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(
                                    ui,
                                    self.filter_window,
                                    school,
                                    school_or,
                                    "Spellschool"
                                );
                            });
                        }
                        ColType::Level(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(ui, self.filter_window, level, level_or, "Spell Level");
                            });
                        }
                        ColType::Subschools(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(
                                    ui,
                                    self.filter_window,
                                    subschool,
                                    subschool_or,
                                    "Subschool"
                                );
                            });
                        }
                        ColType::Domain(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(ui, self.filter_window, domain, domain_or, "Domain");
                            });
                        }
                        ColType::Descriptors(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(
                                    ui,
                                    self.filter_window,
                                    descriptor,
                                    descriptor_or,
                                    "Descriptor"
                                );
                            });
                        }
                        ColType::Components(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(
                                    ui,
                                    self.filter_window,
                                    components,
                                    components_or,
                                    "Components"
                                );
                            });
                        }
                        ColType::Range(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(ui, self.filter_window, range, range_or, "Range");
                            });
                        }
                        ColType::Area(_) => {}
                        ColType::Effect(_) => {}
                        ColType::Targets(_) => {}
                        ColType::Duration(_) => {}
                        ColType::SavingThrow(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(ui, self.filter_window, save, save_or, "Save");
                            });
                        }
                        ColType::SpellResistance(_) => {
                            resp.context_menu(|ui| {
                                filter_row!(
                                    ui,
                                    self.filter_window,
                                    spell_res,
                                    spell_res_or,
                                    "Spell Resistance"
                                );
                            });
                        }
                        ColType::Description(_) => {
                            resp.context_menu(|ui| {
                                ui.add(
                                    egui::text_edit::TextEdit::singleline(
                                        &mut self.filter_window.description,
                                    )
                                    .hint_text("comma,seperated,keywords"),
                                );
                            });
                        }
                        ColType::Source(_) => {}
                    };
                });
            } else if clicked != ColType::None {
                *order = RowOrder::None;
            }
        }
        if clicked != ColType::None {
            for (col, order) in &mut self.shown_columns {
                if clicked == *col {
                    break;
                }
                *order = RowOrder::None;
            }
        }
    }

    fn render_body(&mut self, body: egui_extras::TableBody<'_>, ctx: &egui::Context) {
        if let Some(stuff) = &mut self.shown_value {
            for (col, ordering) in &self.shown_columns {
                stuff.sort_by(|(spell1, level1), (spell2, level2)| {
                    ordering.compare(col, spell1, spell2, level1, level2)
                });
            }

            body.rows(15.0, stuff.len(), |mut row: TableRow<'_, '_>| {
                let (spell, level) = &stuff[row.index()];
                //row.set_selected(selected);
                for (col, _) in &self.shown_columns {
                    if col.get_bool() {
                        match col {
                            ColType::None => {}
                            ColType::Name(_) => {
                                row.col(|ui| {
                                    ui.add(egui::Label::new(&spell.name).selectable(false));
                                });
                            }
                            ColType::School(_) => {
                                row.col(|ui| {
                                    ui.add(egui::Label::new(&spell.school).selectable(false));
                                });
                            }
                            ColType::Level(_) => {
                                row.col(|ui| {
                                    ui.add(egui::Label::new(level).selectable(false));
                                });
                            }
                            ColType::Subschools(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.subschool)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Domain(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.domain)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Descriptors(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.descriptors)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Components(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.components)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Range(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.range).truncate().selectable(false),
                                    );
                                });
                            }
                            ColType::Area(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.area).truncate().selectable(false),
                                    );
                                });
                            }
                            ColType::Effect(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.effect)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Targets(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.targets)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Duration(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.duration)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::SavingThrow(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.saving_throw)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::SpellResistance(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.spell_resistance)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Description(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.short_description)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                            ColType::Source(_) => {
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(&spell.source)
                                            .truncate()
                                            .selectable(false),
                                    );
                                });
                            }
                        };
                    }
                }
                let row_response = row.response();
                let new_spell_id: u32 = stuff[row.index()].0.id;
                if row_response.clicked() {
                    if let Some(old_spell) = &self.selected_spell {
                        if old_spell.id == new_spell_id {
                            self.selected_spell = None;
                        } else {
                            self.selected_spell = Some(stuff[row.index()].0.clone())
                        }
                    } else {
                        self.selected_spell = Some(stuff[row.index()].0.clone())
                    }
                }
                if row_response.secondary_clicked() {
                    if let Some(old_spell) = &self.selected_spell_window {
                        if old_spell.id == new_spell_id {
                            self.selected_spell_window = None;
                            self.spell_window_active = false;
                        } else {
                            self.selected_spell_window = Some(stuff[row.index()].0.clone());
                            self.spell_window_active = true;
                        }
                    } else {
                        self.selected_spell_window = Some(stuff[row.index()].0.clone());
                        self.spell_window_active = true;
                    }
                }
            });
        }
    }

    fn selected_ui(&mut self, ctx: &egui::Context) {
        if let Some(old_spell) = &mut self.selected_spell {
            let r = egui::TopBottomPanel::bottom("bottom")
                .default_height(400.0)
                .resizable(true)
                .show(ctx, |ui| {
                    egui::containers::ScrollArea::vertical()
                        .auto_shrink(false)
                        .show(ui, |ui| render_spell(ui, old_spell))
                        .inner
                })
                .inner;
            match r {
                Some((new_spell, true)) => {
                    if let Some(old_spell) = &self.selected_spell_window {
                        if old_spell.id == new_spell.id {
                            self.selected_spell_window = None;
                        } else {
                            self.selected_spell_window = Some(new_spell);
                        }
                    } else {
                        self.selected_spell_window = Some(new_spell);
                    }
                }
                Some((new_spell, false)) => {
                    if let Some(old_spell) = &self.selected_spell {
                        if old_spell.id == new_spell.id {
                            self.selected_spell = None;
                        } else {
                            self.selected_spell = Some(new_spell);
                        }
                    } else {
                        self.selected_spell = Some(new_spell);
                    }
                }
                None => {}
            }
        }
    }

    fn update_filters(&mut self) -> bool {
        if self.filter_window.filters_changed || self.source_window.filters_changed {
            self.shown_value = Some(
                crate::spell::ALL_SPELLS
                    .iter()
                    .filter_map(|spell| {
                        spell.filter_map_level(
                            self.filter_window.class_or,
                            &self.filter_window.selected_classes,
                        )
                    })
                    .filter(|(spell, level)| self.filter_window.test(spell, level))
                    .filter(|(spell, _)| self.source_window.test(spell))
                    .collect(),
            );
            self.filter_window.filters_changed = false;
            self.source_window.filters_changed = false;
            true
        } else {
            false
        }
    }
}

fn render_spell(ui: &mut egui::Ui, spell: &mut Spell) -> Option<(Spell, bool)> {
    let meta: &SpellMeta = BONUS_INFO.get(&spell.id).unwrap();

    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(&spell.name).size(30.0));
        ui.add(
            egui::Hyperlink::from_label_and_url(
                egui::RichText::new("(d20PFsrd)").size(10.0),
                meta.d20pfsrd,
            )
            .open_in_new_tab(true),
        );
        ui.add(
            egui::Hyperlink::from_label_and_url(
                egui::RichText::new("(Archives)").size(10.0),
                meta.archives,
            )
            .open_in_new_tab(true),
        );
    });
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("School").strong().size(12.0));
        ui.label(
            egui::RichText::new(format!(
                "{}{}",
                &spell.school,
                if spell.subschool.is_empty() {
                    "".to_string()
                } else {
                    format!(" ({})", &spell.subschool)
                }
            ))
            .size(12.0),
        );
        ui.separator();
        ui.label(egui::RichText::new("Level").strong().size(12.0));
        ui.label(egui::RichText::new(&spell.spell_level).size(12.0));
        if !spell.domain.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new("Domain").strong().size(12.0));
            ui.label(egui::RichText::new(&spell.domain).size(12.0));
        }
        if !spell.bloodline.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new("Bloodline").strong().size(12.0));
            ui.label(egui::RichText::new(&spell.bloodline).size(12.0));
        }
    });
    ui.separator();
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Casting Time").strong().size(13.0));
        ui.label(egui::RichText::new(&spell.casting_time).size(13.0));
    });
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Components").strong().size(13.0));
        ui.label(egui::RichText::new(&spell.components).size(13.0));
    });
    ui.separator();
    if !spell.targets.is_empty() {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Target").strong().size(13.0));
            ui.label(egui::RichText::new(&spell.targets).size(13.0));
        });
    }
    if !spell.effect.is_empty() {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Effect").strong().size(13.0));
            ui.label(
                egui::RichText::new(format!(
                    "{}{}",
                    &spell.effect,
                    if spell.shapeable { " (S)" } else { "" }
                ))
                .size(13.0),
            );
        });
    }
    if !spell.range.is_empty() {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Range").strong().size(13.0));
            ui.label(egui::RichText::new(&spell.range).size(13.0));
        });
    }
    if !spell.area.is_empty() {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Area").strong().size(13.0));
            ui.label(egui::RichText::new(&spell.area).size(13.0));
        });
    }

    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Duration").strong().size(13.0));
        ui.label(
            egui::RichText::new(format!(
                "{}{}",
                &spell.duration,
                if spell.dismissible && !spell.duration.ends_with("(D)") {
                    " (D)"
                } else {
                    ""
                }
            ))
            .size(13.0),
        );
    });
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Saving throw").strong().size(13.0));
        ui.label(egui::RichText::new(format!("{};", &spell.saving_throw)).size(13.0));
        ui.label(egui::RichText::new("Spell Resistance").strong().size(13.0));
        ui.label(egui::RichText::new(&spell.spell_resistance).size(13.0));
    });
    ui.separator();
    let r = html2egui(&meta.description_struct, ui);
    //let mut cache = egui_commonmark::CommonMarkCache::default();
    //egui_commonmark::CommonMarkViewer::new().show(ui, &mut cache, meta.description_md);

    if spell.mythic {
        ui.separator();
        ui.label(
            egui::RichText::new(format!("Mythic {}", &spell.name))
                .strong()
                .size(14.0),
        );
        ui.separator();
        ui.label(&spell.mythic_text);
        if !spell.augmented.is_empty() {
            ui.separator();
            ui.label(&spell.augmented);
        }
    }

    r
}

#[derive(serde::Deserialize, serde::Serialize)]
struct FilterWindow {
    name: String,
    school: Vec<Spellschool>,
    school_or: bool,
    level: Vec<Level>,
    level_or: bool,
    subschool: Vec<Subschool>,
    subschool_or: bool,
    domain: Vec<Domain>,
    domain_or: bool,
    descriptor: Vec<SpellDescriptor>,
    descriptor_or: bool,
    components: Vec<SpellComponent>,
    components_or: bool,
    range: Vec<SpellRange>,
    range_or: bool,
    area: String,
    effect: String,
    targets: String,
    duration: String,
    save: Vec<Save>,
    save_or: bool,
    spell_res: Vec<SpellResistance>,
    spell_res_or: bool,
    description: String,
    prev_description: String,
    #[serde(skip)]
    keywords: Vec<Result<Regex, regex::Error>>,
    source: Vec<SpellSource>,
    source_or: bool,
    selected_classes: Vec<ClassType>,
    class_or: bool,
    #[serde(skip, default)]
    filters_changed: bool,
}

impl Default for FilterWindow {
    fn default() -> Self {
        Self {
            name: String::new(),
            school: Spellschool::get_all(),
            school_or: false,
            level: Level::get_all(),
            level_or: false,
            subschool: Subschool::get_all(),
            subschool_or: false,
            domain: Domain::get_all(),
            domain_or: false,
            descriptor: SpellDescriptor::get_all(),
            descriptor_or: false,
            components: SpellComponent::get_all(),
            components_or: false,
            range: SpellRange::get_all(),
            range_or: false,
            area: String::new(),
            effect: String::new(),
            targets: String::new(),
            duration: String::new(),
            save: Save::get_all(),
            save_or: false,
            spell_res: SpellResistance::get_all(),
            spell_res_or: false,
            description: String::new(),
            prev_description: String::new(),
            keywords: Vec::new(),
            source: SpellSource::get_all(),
            source_or: false,
            selected_classes: ClassType::get_all(),
            class_or: false,
            filters_changed: false,
        }
    }
}

impl FilterWindow {
    fn new() -> Self {
        Default::default()
    }

    fn filter_ui(&mut self, ctx: &egui::Context, filter_open: &mut bool) {
        egui::containers::Window::new("Filters")
            .open(filter_open)
            .show(ctx, |ui| {
                ui.label("Name");
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::text_edit::TextEdit::singleline(&mut self.name)
                                .hint_text("case insensitive search"),
                        )
                        .changed()
                    {
                        self.filters_changed = true;
                    };
                });
                ui.separator();
                ui.label("Description");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::text_edit::TextEdit::singleline(&mut self.description)
                            .hint_text("comma,seperated,keywords"),
                    );
                });
                ui.separator();
                filter_row!(ui, self, selected_classes, class_or, "Class");
                ui.separator();
                filter_row!(ui, self, school, school_or, "Spellschool");
                ui.separator();
                filter_row!(ui, self, level, level_or, "Spell Level");
                ui.separator();
                filter_row!(ui, self, components, components_or, "Components");
                ui.separator();
                filter_row!(ui, self, range, range_or, "Range");
                ui.separator();
                filter_row!(ui, self, subschool, subschool_or, "Subschool");
                ui.separator();
                filter_row!(ui, self, domain, domain_or, "Domain");
                ui.separator();
                filter_row!(ui, self, save, save_or, "Save");
                ui.separator();
                filter_row!(ui, self, spell_res, spell_res_or, "Spell Resistance");
                ui.separator();
                filter_row!(ui, self, descriptor, descriptor_or, "Descriptor");
            });
    }

    fn test(&self, spell: &Spell, level: &str) -> bool {
        spell
            .name
            .to_lowercase()
            .contains(&self.name.to_lowercase())
            && keyword_match(&spell.description.to_lowercase(), &self.keywords)
            && if self.level_or {
                let mut it = self.level.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(level) || it.any(|f| f.test(level))
                } else {
                    true
                }
            } else {
                self.level.iter().all(|f| f.test(level))
            }
            && if self.school_or {
                let mut it = self.school.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.school) || it.any(|f| f.test(&spell.school))
                } else {
                    true
                }
            } else {
                self.school.iter().all(|f| f.test(&spell.school))
            }
            && if self.subschool_or {
                let mut it = self.subschool.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.subschool) || it.any(|f| f.test(&spell.subschool))
                } else {
                    true
                }
            } else {
                self.subschool.iter().all(|f| f.test(&spell.subschool))
            }
            && if self.domain_or {
                let mut it = self.domain.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.domain) || it.any(|f| f.test(&spell.domain))
                } else {
                    true
                }
            } else {
                self.domain.iter().all(|f| f.test(&spell.domain))
            }
            && if self.save_or {
                let mut it = self.save.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.saving_throw) || it.any(|f| f.test(&spell.saving_throw))
                } else {
                    true
                }
            } else {
                self.save.iter().all(|f| f.test(&spell.saving_throw))
            }
            && if self.spell_res_or {
                let mut it = self.spell_res.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.spell_resistance) || it.any(|f| f.test(&spell.spell_resistance))
                } else {
                    true
                }
            } else {
                self.spell_res
                    .iter()
                    .all(|f| f.test(&spell.spell_resistance))
            }
            && if self.components_or {
                let mut it = self.components.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.special_test(spell) || it.any(|f| f.special_test(spell))
                } else {
                    true
                }
            } else {
                self.components.iter().all(|f| f.special_test(spell))
            }
            && if self.descriptor_or {
                let mut it = self.descriptor.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.descriptors) || it.any(|f| f.test(&spell.descriptors))
                } else {
                    true
                }
            } else {
                self.descriptor.iter().all(|f| f.test(&spell.descriptors))
            }
            && if self.range_or {
                let mut it = self.range.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.range) || it.any(|f| f.test(&spell.range))
                } else {
                    true
                }
            } else {
                self.range.iter().all(|f| f.test(&spell.range))
            }
            && if self.source_or {
                let mut it = self.source.iter().filter(|f| f.some_filter());
                if let Some(ff) = it.next() {
                    ff.test(&spell.source) || it.any(|f| f.test_exact(&spell.source))
                } else {
                    true
                }
            } else {
                self.source.iter().all(|f| f.test_exact(&spell.source))
            }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SourceWindow {
    source: Vec<SpellSource>,
    source_or: bool,
    filters_changed: bool,
}

impl Default for SourceWindow {
    fn default() -> Self {
        Self {
            source: SpellSource::get_all(),
            source_or: false,
            filters_changed: false,
        }
    }
}

impl SourceWindow {
    fn new() -> Self {
        Default::default()
    }

    fn filter_ui(&mut self, ctx: &egui::Context, filter_open: &mut bool) {
        egui::containers::Window::new("Sources")
            .open(filter_open)
            .show(ctx, |ui| {
                filter_row!(ui, self, source, source_or, "");
            });
    }

    fn test(&self, spell: &Spell) -> bool {
        if self.source_or {
            let mut it = self.source.iter().filter(|f| f.some_filter());
            if let Some(ff) = it.next() {
                ff.test(&spell.source) || it.any(|f| f.test_exact(&spell.source))
            } else {
                true
            }
        } else {
            self.source.iter().all(|f| f.test_exact(&spell.source))
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SpellWindow {}

impl Default for SpellWindow {
    fn default() -> Self {
        Self {}
    }
}

impl SpellWindow {
    fn new() -> Self {
        Default::default()
    }

    fn spell_ui(
        &mut self,
        ctx: &egui::Context,
        filter_open: &mut bool,
        spell: &mut Spell,
    ) -> Option<(Spell, bool)> {
        if let Some(r) = egui::containers::Window::new("Spell")
            .open(filter_open)
            .show(ctx, |ui| render_spell(ui, spell))
        {
            r.inner?
        } else {
            None
        }
    }
}

fn keyword_match(haystack: &str, keywords: &Vec<Result<Regex, regex::Error>>) -> bool {
    let mut ret = true;
    for word in keywords {
        ret &= if let Ok(re) = word {
            re.is_match(haystack)
        } else {
            false
        };
    }
    ret
}
