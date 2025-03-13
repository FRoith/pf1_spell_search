use egui_extras::{Column, TableBuilder, TableRow};
use filter_repr::FilterRepr;

use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

use crate::{
    filters::{
        Domain, Level, Save, SpellComponent, SpellDescriptor, SpellResistance, Spellschool,
        Subschool,
    },
    toggle::toggle,
};

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(de::Error::invalid_value(Unexpected::Str(other), &"1 or 0")),
    }
}

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

    fn compare(&self, col: &ColType, a: &Spell, b: &Spell, c: u32, d: u32) -> std::cmp::Ordering {
        match (self, col) {
            (Self::Ascending, ColType::Name(_)) => a.name.cmp(&b.name),
            (Self::Ascending, ColType::School(_)) => a.school.cmp(&b.school),
            (Self::Ascending, ColType::Level(_)) => c.cmp(&d),
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
            (Self::Descending, ColType::Level(_)) => d.cmp(&c),
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub enum ClassType {
    SpellLikeAbility,
    Sorcerer,
    Wizard,
    Cleric,
    Druid,
    Ranger,
    Bard,
    Paladin,
    Alchemist,
    Summoner,
    Witch,
    Inquisitor,
    Oracle,
    Antipaladin,
    Magus,
    Adept,
    Bloodrager,
    Shaman,
    Psychic,
    Medium,
    Mesmerist,
    Occultist,
    Spiritualist,
    Skald,
    Investigator,
    Hunter,
    UncSummoner,
}

impl ClassType {
    fn title(&self) -> String {
        match self {
            Self::SpellLikeAbility => "Spell Like Ability",
            Self::Sorcerer => "Sorcerer",
            Self::Wizard => "Wizard",
            Self::Cleric => "Cleric",
            Self::Druid => "Druid",
            Self::Ranger => "Ranger",
            Self::Bard => "Bard",
            Self::Paladin => "Paladin",
            Self::Alchemist => "Alchemist",
            Self::Summoner => "Summoner",
            Self::Witch => "Witch",
            Self::Inquisitor => "Inquisitor",
            Self::Oracle => "Oracle",
            Self::Antipaladin => "Antipaladin",
            Self::Magus => "Magus",
            Self::Adept => "Adept",
            Self::Bloodrager => "Bloodrager",
            Self::Shaman => "Shaman",
            Self::Psychic => "Psychic",
            Self::Medium => "Medium",
            Self::Mesmerist => "Mesmerist",
            Self::Occultist => "Occultist",
            Self::Spiritualist => "Spiritualist",
            Self::Skald => "Skald",
            Self::Investigator => "Investigator",
            Self::Hunter => "Hunter",
            Self::UncSummoner => "Unchained Summoner",
        }
        .to_string()
    }

    fn get_value(&self, spell: &Spell) -> Option<u32> {
        match self {
            Self::SpellLikeAbility => Some(spell.sla_level),
            Self::Sorcerer => spell.sor,
            Self::Wizard => spell.wiz,
            Self::Cleric => spell.cleric,
            Self::Druid => spell.druid,
            Self::Ranger => spell.ranger,
            Self::Bard => spell.bard,
            Self::Paladin => spell.paladin,
            Self::Alchemist => spell.alchemist,
            Self::Summoner => spell.summoner,
            Self::Witch => spell.witch,
            Self::Inquisitor => spell.inquisitor,
            Self::Oracle => spell.oracle,
            Self::Antipaladin => spell.antipaladin,
            Self::Magus => spell.magus,
            Self::Adept => spell.adept,
            Self::Bloodrager => spell.bloodrager,
            Self::Shaman => spell.shaman,
            Self::Psychic => spell.psychic,
            Self::Medium => spell.medium,
            Self::Mesmerist => spell.mesmerist,
            Self::Occultist => spell.occultist,
            Self::Spiritualist => spell.spiritualist,
            Self::Skald => spell.skald,
            Self::Investigator => spell.investigator,
            Self::Hunter => spell.hunter,
            Self::UncSummoner => spell.summoner_unchained,
        }
    }

    fn get_all() -> Vec<Self> {
        [
            Self::SpellLikeAbility,
            Self::Sorcerer,
            Self::Wizard,
            Self::Cleric,
            Self::Druid,
            Self::Ranger,
            Self::Bard,
            Self::Paladin,
            Self::Alchemist,
            Self::Summoner,
            Self::Witch,
            Self::Inquisitor,
            Self::Oracle,
            Self::Antipaladin,
            Self::Magus,
            Self::Adept,
            Self::Bloodrager,
            Self::Shaman,
            Self::Psychic,
            Self::Medium,
            Self::Mesmerist,
            Self::Occultist,
            Self::Spiritualist,
            Self::Skald,
            Self::Investigator,
            Self::Hunter,
            Self::UncSummoner,
        ]
        .into()
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
            (Self::Level(false), RowOrder::None),
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
            (Self::Description(false), RowOrder::None),
            (Self::Source(true), RowOrder::None),
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq, Clone)]
pub struct Spell {
    pub name: String,
    pub school: String,
    pub subschool: String,
    #[serde(rename = "descriptor")]
    pub descriptors: String,
    pub spell_level: String,
    pub casting_time: String,
    pub components: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub costly_components: bool,
    pub range: String,
    pub area: String,
    pub effect: String,
    pub targets: String,
    pub duration: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub dismissible: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub shapeable: bool,
    pub saving_throw: String,
    pub spell_resistance: String,
    pub description: String,
    pub description_formatted: String,
    pub source: String,
    pub full_text: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub verbal: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub somatic: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub material: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub focus: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub divine_focus: bool,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub sor: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub wiz: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub cleric: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub druid: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub ranger: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub bard: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub paladin: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub alchemist: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub summoner: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub witch: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub inquisitor: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub oracle: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub antipaladin: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub magus: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub adept: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub deity: Option<String>,
    #[serde(rename = "SLA_Level")]
    pub sla_level: u32,
    pub domain: String,
    pub short_description: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub acid: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub air: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub chaotic: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub cold: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub curse: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub darkness: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub death: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub disease: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub earth: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub electricity: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub emotion: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub evil: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub fear: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub fire: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub force: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub good: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub language_dependent: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub lawful: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub light: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub mind_affecting: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pain: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub poison: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub shadow: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub sonic: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub water: bool,
    pub linktext: String,
    pub id: u32,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub material_costs: Option<u32>,
    pub bloodline: String,
    pub patron: String,
    pub mythic_text: String,
    pub augmented: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub mythic: bool,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub bloodrager: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub shaman: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub psychic: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub medium: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub mesmerist: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub occultist: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub spiritualist: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub skald: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub investigator: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub hunter: Option<u32>,
    pub haunt_statistics: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub ruse: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub draconic: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub meditative: bool,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub summoner_unchained: Option<u32>,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SpellSearchApp {
    spell_table: SpellTable,
    #[serde(skip)]
    filter_window_active: bool,
}

impl Default for SpellSearchApp {
    fn default() -> Self {
        Self {
            spell_table: SpellTable::new(),
            filter_window_active: false,
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

                if self.filter_window_active {
                    self.spell_table.filter_window.filter_ui(ctx);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        self.spell_table.selected_ui(ctx);
        self.spell_table.table_ui(ctx);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SpellTable {
    #[serde(skip, default = "load_spell_table")]
    // This how you opt-out of serialization of a field
    value: Vec<Spell>,

    shown_columns: Vec<(ColType, RowOrder)>,
    selected_class: ClassType,
    filter_string: String,
    selected_spell: Option<Spell>,
    filter_window: FilterWindow,
}

fn load_spell_table() -> Vec<Spell> {
    let data = include_str!("../db/spell_full - Updated 31Mar2020.csv");
    let mut reader = csv::ReaderBuilder::new()
        .terminator(csv::Terminator::CRLF)
        .from_reader(data.as_bytes());
    reader.deserialize().filter_map(|x| x.ok()).collect()
}

impl SpellTable {
    fn new() -> Self {
        let shown_columns: Vec<(ColType, RowOrder)> = ColType::get_all();
        Self {
            value: load_spell_table(),
            shown_columns,
            selected_class: ClassType::SpellLikeAbility,
            filter_string: String::new(),
            selected_spell: None,
            filter_window: FilterWindow::new(),
        }
    }

    fn table_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and BottomPanel's
            egui::ComboBox::from_label("Class")
                .selected_text(self.selected_class.title())
                .show_ui(ui, |ui| {
                    for e in ClassType::get_all() {
                        let title = e.title();
                        ui.selectable_value(&mut self.selected_class, e, title);
                    }
                });

            ui.separator();

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
                    self.render_body(body);
                });
        });
    }

    fn render_header(&mut self, header: &mut egui_extras::TableRow<'_, '_>) {
        let mut clicked = ColType::None;
        for (col, order) in &mut self.shown_columns {
            if col.get_bool() {
                header.col(|ui| {
                    ui.horizontal(|ui| {
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
                        ui.heading(col.title());

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
                    });
                });
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

    fn render_body(&mut self, body: egui_extras::TableBody<'_>) {
        let mut stuff: Vec<(&Spell, u32)> = self
            .value
            .iter()
            .filter(|s| self.filter_window.test(s))
            .filter_map(|spell| {
                self.selected_class
                    .get_value(spell)
                    .map(|level| (spell, level))
            })
            .filter(|(_spell, sl)| self.filter_window.test_sl(format!("{}", sl)))
            .collect();
        for (col, ordering) in &self.shown_columns {
            stuff.sort_by(|(spell1, level1), (spell2, level2)| {
                ordering.compare(col, spell1, spell2, *level1, *level2)
            });
        }

        body.rows(15.0, stuff.len(), |mut row: TableRow<'_, '_>| {
            let (spell, level) = stuff[row.index()];
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
                                ui.add(egui::Label::new(format!("{}", level)).selectable(false));
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
                                    egui::Label::new(&spell.domain).truncate().selectable(false),
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
                                ui.add(egui::Label::new(&spell.range).truncate().selectable(false));
                            });
                        }
                        ColType::Area(_) => {
                            row.col(|ui| {
                                ui.add(egui::Label::new(&spell.area).truncate().selectable(false));
                            });
                        }
                        ColType::Effect(_) => {
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(&spell.effect).truncate().selectable(false),
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
                                    egui::Label::new(&spell.source).truncate().selectable(false),
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
        });
    }

    fn selected_ui(&mut self, ctx: &egui::Context) {
        if let Some(old_spell) = &self.selected_spell {
            egui::TopBottomPanel::bottom("bottom")
                .default_height(400.0)
                .resizable(true)
                .show(ctx, |ui| {
                    egui::containers::ScrollArea::vertical()
                        .auto_shrink(false)
                        .show(ui, |ui| Self::render_spell(ui, old_spell));
                });
        }
    }

    fn render_spell(ui: &mut egui::Ui, spell: &Spell) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(&spell.name).size(30.0));
        });
        ui.horizontal(|ui| {
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
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Casting Time").strong().size(13.0));
            ui.label(egui::RichText::new(&spell.casting_time).size(13.0));
        });
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Components").strong().size(13.0));
            ui.label(egui::RichText::new(&spell.components).size(13.0));
        });
        ui.separator();
        if !spell.targets.is_empty() {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Target").strong().size(13.0));
                ui.label(egui::RichText::new(&spell.targets).size(13.0));
            });
        }
        if !spell.effect.is_empty() {
            ui.horizontal(|ui| {
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
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Range").strong().size(13.0));
                ui.label(egui::RichText::new(&spell.range).size(13.0));
            });
        }
        if !spell.area.is_empty() {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Area").strong().size(13.0));
                ui.label(egui::RichText::new(&spell.area).size(13.0));
            });
        }

        ui.horizontal(|ui| {
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
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Saving throw").strong().size(13.0));
            ui.label(egui::RichText::new(format!("{};", &spell.saving_throw)).size(13.0));
            ui.label(egui::RichText::new("Spell Resistance").strong().size(13.0));
            ui.label(egui::RichText::new(&spell.spell_resistance).size(13.0));
        });
        ui.separator();
        let mut cache = egui_commonmark::CommonMarkCache::default();
        egui_commonmark::CommonMarkViewer::new().show(
            ui,
            &mut cache,
            &html2md::parse_html(&spell.description_formatted).replace("\n ", "\n\n"),
        );
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
    }
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
    range: String,
    area: String,
    effect: String,
    targets: String,
    duration: String,
    save: Vec<Save>,
    save_or: bool,
    spell_res: Vec<SpellResistance>,
    spell_res_or: bool,
    description: String,
    source: String,
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
            range: String::new(),
            area: String::new(),
            effect: String::new(),
            targets: String::new(),
            duration: String::new(),
            save: Save::get_all(),
            save_or: false,
            spell_res: SpellResistance::get_all(),
            spell_res_or: false,
            description: String::new(),
            source: String::new(),
        }
    }
}

impl FilterWindow {
    fn new() -> Self {
        Default::default()
    }

    fn filter_ui(&mut self, ctx: &egui::Context) {
        egui::containers::Window::new("Filters").show(ctx, |ui| {
            ui.label("Name");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.name);
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Spellschool");
                ui.add(toggle(&mut self.school_or));
            });
            ui.horizontal_wrapped(|ui| {
                for school in &mut self.school {
                    *school = school.create_btn(ui);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Spell Level");
                ui.add(toggle(&mut self.level_or));
            });
            ui.horizontal_wrapped(|ui| {
                for level in &mut self.level {
                    *level = level.create_btn(ui);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Components");
                ui.add(toggle(&mut self.components_or));
            });
            ui.horizontal_wrapped(|ui| {
                for comp in &mut self.components {
                    *comp = comp.create_btn(ui);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Subschool");
                ui.add(toggle(&mut self.subschool_or));
            });
            ui.horizontal_wrapped(|ui| {
                for subschool in &mut self.subschool {
                    *subschool = subschool.create_btn(ui);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Domain");
                ui.add(toggle(&mut self.domain_or));
            });
            ui.horizontal_wrapped(|ui| {
                for domain in &mut self.domain {
                    *domain = domain.create_btn(ui);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Save");
                ui.add(toggle(&mut self.save_or));
            });
            ui.horizontal_wrapped(|ui| {
                for save in &mut self.save {
                    *save = save.create_btn(ui);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Spell Resistance");
                ui.add(toggle(&mut self.spell_res_or));
            });
            ui.horizontal_wrapped(|ui| {
                for spell_res in &mut self.spell_res {
                    *spell_res = spell_res.create_btn(ui);
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Descriptor");
                ui.add(toggle(&mut self.descriptor_or));
            });
            ui.horizontal_wrapped(|ui| {
                for desc in &mut self.descriptor {
                    *desc = desc.create_btn(ui);
                }
            });
        });
    }

    fn test(&self, spell: &Spell) -> bool {
        spell
            .name
            .to_lowercase()
            .contains(&self.name.to_lowercase())
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
    }

    fn test_sl(&self, spell_level: String) -> bool {
        if self.level_or {
            let mut it = self.level.iter().filter(|f| f.some_filter());
            if let Some(ff) = it.next() {
                ff.test(&spell_level) || it.any(|f| f.test(&spell_level))
            } else {
                true
            }
        } else {
            self.level.iter().all(|f| f.test(&spell_level))
        }
    }
}
