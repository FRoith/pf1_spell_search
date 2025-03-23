use crate::spell::{SpellDescriptionStruct, ALL_SPELLS};

#[macro_export]
macro_rules! filter_row {
    ($ui:ident, $filter_window:expr, $value:ident, $or:ident, $label:expr) => {
        $ui.horizontal(|ui| {
            if ($label != "") {
                ui.label($label);
            }
            if ui.add(toggle(&mut $filter_window.$or)).changed() {
                $filter_window.filters_changed = true;
            }
        });
        $ui.horizontal_wrapped(|ui| {
            for x in &mut $filter_window.$value {
                let resp = x.create_btn(ui);
                if resp != *x {
                    $filter_window.filters_changed = true;
                }
                *x = resp;
            }
        });
    };
}

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool, text_off: &str, text_on: &str) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() || response.secondary_clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );

        let center = egui::pos2((rect.left() + rect.right()) / 2.0, rect.center().y);
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            if *on { text_off } else { text_on },
            egui::FontId::monospace(12.0),
            visuals.text_color(),
        );
    }

    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on, "OR", "AND")
}

pub fn html2egui(
    html: &'static SpellDescriptionStruct,
    ui: &mut egui::Ui,
) -> Option<(crate::spell::Spell, bool)> {
    let x = ui.spacing().item_spacing.x;

    struct2egui(html, ui, x, false, 0)
}

fn struct2egui(
    st: &'static SpellDescriptionStruct,
    ui: &mut egui::Ui,
    x: f32,
    inline: bool,
    body_index: usize,
) -> Option<(crate::spell::Spell, bool)> {
    match st {
        SpellDescriptionStruct::Body(spell_description_structs) => {
            egui::Frame::new()
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let mut ret = None;
                    for (i, c) in spell_description_structs.iter().enumerate() {
                        if let Some(res) = struct2egui(c, ui, x, false, i) {
                            ret = Some(res);
                        }
                    }
                    ui.spacing_mut().item_spacing.x = x;
                    ret
                })
                .inner
        }
        SpellDescriptionStruct::Paragraph(spell_description_structs) => {
            if spell_description_structs.is_empty() {
                None
            } else if inline {
                let mut ret = None;
                for c in spell_description_structs {
                    if let Some(res) = struct2egui(c, ui, x, inline, body_index) {
                        ret = Some(res);
                    }
                }
                ret
            } else {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let mut ret = None;
                    for c in spell_description_structs {
                        if let Some(res) = struct2egui(c, ui, x, inline, body_index) {
                            ret = Some(res);
                        }
                    }
                    ui.spacing_mut().item_spacing.x = x;
                    ret
                })
                .inner
            }
        }
        SpellDescriptionStruct::Caption(spell_description_structs) => {
            for c in spell_description_structs {
                match c {
                    SpellDescriptionStruct::Text(text) => {
                        ui.label(*text);
                    }
                    _ => panic!("non text element found in caption"),
                }
            }
            None
        }
        SpellDescriptionStruct::Italics(text) => {
            if let Some(spell) = ALL_SPELLS
                .iter()
                .find(|s| s.name.to_lowercase() == text.to_lowercase())
            {
                let res = ui.link(egui::RichText::new(*text));
                if res.clicked() {
                    Some((spell.clone(), false))
                } else if res.secondary_clicked() {
                    Some((spell.clone(), true))
                } else {
                    None
                }
            } else {
                ui.label(egui::RichText::new(*text).italics());
                None
            }
        }
        SpellDescriptionStruct::Bold(text) => {
            ui.label(egui::RichText::new(*text).strong());
            None
        }
        SpellDescriptionStruct::Sup(spell_description_structs) => {
            ui.spacing_mut().item_spacing.x = 0.0;
            let ret = ui
                .horizontal_top(|ui| {
                    let mut ret = None;
                    for c in spell_description_structs {
                        if let Some(res) = struct2egui(c, ui, x, false, body_index) {
                            ret = Some(res);
                        }
                    }
                    ret
                })
                .inner;
            ui.spacing_mut().item_spacing.x = x;
            ret
        }
        SpellDescriptionStruct::Br => {
            if !inline {
                ui.label("\n");
            }
            None
        }
        SpellDescriptionStruct::Listing(spell_description_structs) => {
            let ret = ui
                .vertical(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let mut ret = None;

                    for c in spell_description_structs {
                        if let SpellDescriptionStruct::Line(_) = c {
                            if let Some(res) = struct2egui(c, ui, x, true, body_index) {
                                ret = Some(res);
                            }
                        } else {
                            panic!("Found non-List element in Listing")
                        }
                    }
                    ui.spacing_mut().item_spacing.x = x;
                    ui.horizontal(|ui| {
                        ui.set_invisible();
                        ui.add(egui::Separator::default().horizontal());
                    });
                    ret
                })
                .inner;
            ret
        }
        SpellDescriptionStruct::Line(spell_description_structs) => {
            ui.horizontal(|ui| {
                ui.label("▪");

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let mut ret = None;
                    for c in spell_description_structs {
                        if let Some(res) = struct2egui(c, ui, x, true, body_index) {
                            ret = Some(res);
                        }
                    }
                    ui.spacing_mut().item_spacing.x = x;
                    ret
                })
                .inner
            })
            .inner
        }
        SpellDescriptionStruct::Table(spell_description_structs) => {
            let thead = spell_description_structs
                .iter()
                .find(|e| matches!(e, SpellDescriptionStruct::Thead(_)))
                .unwrap_or(&SpellDescriptionStruct::Br);

            let tbody = spell_description_structs
                .iter()
                .find(|e| matches!(e, SpellDescriptionStruct::Tbody(_)))
                .unwrap_or(&SpellDescriptionStruct::Br);

            struct2egui(&SpellDescriptionStruct::Br, ui, x, inline, body_index);
            ui.label(" ");

            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(false),
                        |ui| {
                            let n = rowlen(tbody);
                            ui.spacing_mut().item_spacing.x = x;

                            if let Some(tcaption) = spell_description_structs
                                .iter()
                                .find(|e| matches!(e, SpellDescriptionStruct::Caption(_)))
                            {
                                struct2egui(tcaption, ui, x, inline, body_index);
                            };

                            egui::Frame::new()
                                .inner_margin(egui::Margin::symmetric(4, -2))
                                .outer_margin(8)
                                .shadow(egui::Shadow::NONE)
                                .show(ui, |ui| {
                                    let tb = egui_extras::TableBuilder::new(ui)
                                        .id_salt(body_index)
                                        .striped(true)
                                        .auto_shrink(true)
                                        .cell_layout(egui::Layout::centered_and_justified(
                                            egui::Direction::LeftToRight,
                                        ))
                                        .columns(egui_extras::Column::auto(), n);
                                    let table = thead2egui(thead, tb, x);

                                    if let SpellDescriptionStruct::Tbody(tbody_rows) = tbody {
                                        tbody2egui(tbody, table, tbodylen(tbody, n), x);

                                        if let Some(SpellDescriptionStruct::Row(cells)) =
                                            tbody_rows.last()
                                        {
                                            if cells.len() != n {
                                                for cell in cells {
                                                    struct2egui(cell, ui, x, false, body_index);
                                                }
                                            }
                                        }
                                    } else {
                                        panic!("no tbody found!")
                                    }
                                });

                            if let Some(tfoot) = spell_description_structs
                                .iter()
                                .find(|e| matches!(e, SpellDescriptionStruct::Tfoot(_)))
                            {
                                struct2egui(tfoot, ui, x, inline, body_index);
                            };
                        },
                    );

                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("");
                    ui.spacing_mut().item_spacing.x = x;
                },
            );
            None
        }
        SpellDescriptionStruct::Header(spell_description_structs) => {
            ui.horizontal(|ui| {
                let mut ret = None;
                for c in spell_description_structs {
                    if let Some(res) = struct2egui(c, ui, x, true, body_index) {
                        ret = Some(res);
                    }
                }
                ret
            })
            .inner
        }
        SpellDescriptionStruct::Cell(spell_description_structs) => {
            ui.spacing_mut().item_spacing.x = 0.0;
            let ret = ui
                .horizontal(|ui| {
                    let mut ret = None;
                    for c in spell_description_structs {
                        if let Some(res) = struct2egui(c, ui, x, true, body_index) {
                            ret = Some(res);
                        }
                    }
                    ret
                })
                .inner;
            ui.spacing_mut().item_spacing.x = x;
            ret
        }
        SpellDescriptionStruct::Tfoot(spell_description_structs) => {
            spell_description_structs
                .iter()
                .filter_map(|e| match e {
                    SpellDescriptionStruct::Row(spell_description_structs) => {
                        Some(spell_description_structs)
                    }
                    _ => None,
                })
                .for_each(|c| {
                    for cc in c {
                        struct2egui(cc, ui, x, true, body_index);
                    }
                });
            None
        }
        SpellDescriptionStruct::Text(text) => {
            ui.label(*text);
            None
        }
        _ => {
            panic!("Encountered unknown struct {:?}", st);
        }
    }
}

fn thead2egui<'a>(
    head: &'static SpellDescriptionStruct,
    tb: egui_extras::TableBuilder<'a>,
    x: f32,
) -> egui_extras::Table<'a> {
    if let SpellDescriptionStruct::Thead(header_data) = head {
        if header_data.len() != 1 {
            tb.header(20.0, |_| {})
        } else if let Some(head_row) = header_data.first() {
            tb.header(20.0, |row| {
                trow2egui(head_row, row, x);
            })
        } else {
            tb.header(20.0, |_| {})
        }
    } else {
        tb.header(20.0, |_| {})
    }
}

fn tbody2egui(
    body_data: &'static SpellDescriptionStruct,
    t: egui_extras::Table<'_>,
    nrows: usize,
    x: f32,
) -> egui::scroll_area::ScrollAreaOutput<()> {
    if let SpellDescriptionStruct::Tbody(body_rows) = body_data {
        t.body(|body| {
            body.rows(20.0, nrows, |row| {
                trow2egui(body_rows.get(row.index()).unwrap(), row, x);
            });
        })
    } else {
        t.body(|_| {})
    }
}

fn trow2egui(
    row_data: &'static SpellDescriptionStruct,
    mut row: egui_extras::TableRow<'_, '_>,
    x: f32,
) {
    if let SpellDescriptionStruct::Row(cells) = row_data {
        for cell in cells {
            row.col(|ui| {
                cell2egui(cell, ui, x);
            });
        }
    }
}

fn cell2egui(cell: &'static SpellDescriptionStruct, ui: &mut egui::Ui, x: f32) {
    egui::Frame::new()
        .outer_margin(egui::Margin::symmetric(-4, -2))
        .inner_margin(0)
        .stroke(ui.style().visuals.window_stroke)
        .show(ui, |ui| {
            ui.set_width(ui.max_rect().width());
            ui.horizontal_centered(|ui| {
                ui.label("");
                struct2egui(cell, ui, x, true, 0);
                ui.label("");
            });
        });
}

fn tbodylen(tbody: &'static SpellDescriptionStruct, ncols: usize) -> usize {
    match tbody {
        SpellDescriptionStruct::Tbody(tb) => tb
            .iter()
            .filter(|r| rowlen(r) == ncols)
            .collect::<Vec<_>>()
            .len(),
        _ => 0,
    }
}

fn rowlen(telem: &'static SpellDescriptionStruct) -> usize {
    match telem {
        SpellDescriptionStruct::Tbody(tb) | SpellDescriptionStruct::Thead(tb) => {
            if let Some(r) = tb.first() {
                rowlen(r)
            } else {
                0
            }
        }
        SpellDescriptionStruct::Row(r) => r.len(),
        _ => 0,
    }
}
