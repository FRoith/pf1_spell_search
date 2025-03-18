use std::fmt::Display;
use std::fmt::Formatter;

use filter_derive::FilterReprMacro;
use filter_repr::{FilterRepr, FilterState};

use serde;

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match <String as serde::Deserialize>::deserialize(deserializer)?.as_ref() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(other),
            &"1 or 0",
        )),
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

impl Display for Spell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Spell {
    pub fn filter_map_level(
        &self,
        class_sel_or: bool,
        classes: &[ClassType],
    ) -> Option<(&Self, String)> {
        let res = if class_sel_or {
            let mut it = classes.iter().filter(|f| f.some_filter());
            if let Some(ff) = it.next() {
                ff.test_cls(self) || it.any(|f| f.test_cls(self))
            } else {
                true
            }
        } else {
            classes
                .iter()
                .filter(|f| f.some_filter())
                .all(|f| f.test_cls(self))
        };
        let lvls: Vec<String> = classes
            .iter()
            .filter(|c| *c.get_contained() == FilterState::Positive)
            .map(|c| match c.get_value(self) {
                Some(u) => u.to_string(),
                None => "-".to_string(),
            })
            .collect();
        let rv = if lvls.is_empty() {
            format!("{}", self.sla_level)
        } else if lvls.contains(&"/".to_string()) {
            format!("{}({})", self.sla_level, lvls.join("/"))
        } else {
            lvls.join("/")
        };

        if res {
            Some((self, rv))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Eq, PartialEq, FilterReprMacro)]
pub enum ClassType {
    Sorcerer(FilterState),
    Wizard(FilterState),
    Cleric(FilterState),
    Druid(FilterState),
    Ranger(FilterState),
    Bard(FilterState),
    Paladin(FilterState),
    Alchemist(FilterState),
    Summoner(FilterState),
    Witch(FilterState),
    Inquisitor(FilterState),
    Oracle(FilterState),
    Antipaladin(FilterState),
    Magus(FilterState),
    Adept(FilterState),
    Bloodrager(FilterState),
    Shaman(FilterState),
    Psychic(FilterState),
    Medium(FilterState),
    Mesmerist(FilterState),
    Occultist(FilterState),
    Spiritualist(FilterState),
    Skald(FilterState),
    Investigator(FilterState),
    Hunter(FilterState),
    UncSummoner(FilterState),
}

impl ClassType {
    pub fn title(&self) -> String {
        match self {
            Self::Sorcerer(_) => "Sorcerer",
            Self::Wizard(_) => "Wizard",
            Self::Cleric(_) => "Cleric",
            Self::Druid(_) => "Druid",
            Self::Ranger(_) => "Ranger",
            Self::Bard(_) => "Bard",
            Self::Paladin(_) => "Paladin",
            Self::Alchemist(_) => "Alchemist",
            Self::Summoner(_) => "Summoner",
            Self::Witch(_) => "Witch",
            Self::Inquisitor(_) => "Inquisitor",
            Self::Oracle(_) => "Oracle",
            Self::Antipaladin(_) => "Antipaladin",
            Self::Magus(_) => "Magus",
            Self::Adept(_) => "Adept",
            Self::Bloodrager(_) => "Bloodrager",
            Self::Shaman(_) => "Shaman",
            Self::Psychic(_) => "Psychic",
            Self::Medium(_) => "Medium",
            Self::Mesmerist(_) => "Mesmerist",
            Self::Occultist(_) => "Occultist",
            Self::Spiritualist(_) => "Spiritualist",
            Self::Skald(_) => "Skald",
            Self::Investigator(_) => "Investigator",
            Self::Hunter(_) => "Hunter",
            Self::UncSummoner(_) => "Unchained Summoner",
        }
        .to_string()
    }

    pub fn get_value(&self, spell: &Spell) -> Option<u32> {
        match self {
            Self::Sorcerer(FilterState::Positive) => spell.sor,
            Self::Wizard(FilterState::Positive) => spell.wiz,
            Self::Cleric(FilterState::Positive) => spell.cleric,
            Self::Druid(FilterState::Positive) => spell.druid,
            Self::Ranger(FilterState::Positive) => spell.ranger,
            Self::Bard(FilterState::Positive) => spell.bard,
            Self::Paladin(FilterState::Positive) => spell.paladin,
            Self::Alchemist(FilterState::Positive) => spell.alchemist,
            Self::Summoner(FilterState::Positive) => spell.summoner,
            Self::Witch(FilterState::Positive) => spell.witch,
            Self::Inquisitor(FilterState::Positive) => spell.inquisitor,
            Self::Oracle(FilterState::Positive) => spell.oracle,
            Self::Antipaladin(FilterState::Positive) => spell.antipaladin,
            Self::Magus(FilterState::Positive) => spell.magus,
            Self::Adept(FilterState::Positive) => spell.adept,
            Self::Bloodrager(FilterState::Positive) => spell.bloodrager,
            Self::Shaman(FilterState::Positive) => spell.shaman,
            Self::Psychic(FilterState::Positive) => spell.psychic,
            Self::Medium(FilterState::Positive) => spell.medium,
            Self::Mesmerist(FilterState::Positive) => spell.mesmerist,
            Self::Occultist(FilterState::Positive) => spell.occultist,
            Self::Spiritualist(FilterState::Positive) => spell.spiritualist,
            Self::Skald(FilterState::Positive) => spell.skald,
            Self::Investigator(FilterState::Positive) => spell.investigator,
            Self::Hunter(FilterState::Positive) => spell.hunter,
            Self::UncSummoner(FilterState::Positive) => spell.summoner_unchained,
            _ => None,
        }
    }

    pub fn get_all() -> Vec<Self> {
        [
            Self::Sorcerer(FilterState::None),
            Self::Wizard(FilterState::None),
            Self::Cleric(FilterState::None),
            Self::Druid(FilterState::None),
            Self::Ranger(FilterState::None),
            Self::Bard(FilterState::None),
            Self::Paladin(FilterState::None),
            Self::Alchemist(FilterState::None),
            Self::Summoner(FilterState::None),
            Self::Witch(FilterState::None),
            Self::Inquisitor(FilterState::None),
            Self::Oracle(FilterState::None),
            Self::Antipaladin(FilterState::None),
            Self::Magus(FilterState::None),
            Self::Adept(FilterState::None),
            Self::Bloodrager(FilterState::None),
            Self::Shaman(FilterState::None),
            Self::Psychic(FilterState::None),
            Self::Medium(FilterState::None),
            Self::Mesmerist(FilterState::None),
            Self::Occultist(FilterState::None),
            Self::Spiritualist(FilterState::None),
            Self::Skald(FilterState::None),
            Self::Investigator(FilterState::None),
            Self::Hunter(FilterState::None),
            Self::UncSummoner(FilterState::None),
        ]
        .into()
    }

    pub fn get_contained(&self) -> &FilterState {
        match self {
            ClassType::Sorcerer(filter_state) => filter_state,
            ClassType::Wizard(filter_state) => filter_state,
            ClassType::Cleric(filter_state) => filter_state,
            ClassType::Druid(filter_state) => filter_state,
            ClassType::Ranger(filter_state) => filter_state,
            ClassType::Bard(filter_state) => filter_state,
            ClassType::Paladin(filter_state) => filter_state,
            ClassType::Alchemist(filter_state) => filter_state,
            ClassType::Summoner(filter_state) => filter_state,
            ClassType::Witch(filter_state) => filter_state,
            ClassType::Inquisitor(filter_state) => filter_state,
            ClassType::Oracle(filter_state) => filter_state,
            ClassType::Antipaladin(filter_state) => filter_state,
            ClassType::Magus(filter_state) => filter_state,
            ClassType::Adept(filter_state) => filter_state,
            ClassType::Bloodrager(filter_state) => filter_state,
            ClassType::Shaman(filter_state) => filter_state,
            ClassType::Psychic(filter_state) => filter_state,
            ClassType::Medium(filter_state) => filter_state,
            ClassType::Mesmerist(filter_state) => filter_state,
            ClassType::Occultist(filter_state) => filter_state,
            ClassType::Spiritualist(filter_state) => filter_state,
            ClassType::Skald(filter_state) => filter_state,
            ClassType::Investigator(filter_state) => filter_state,
            ClassType::Hunter(filter_state) => filter_state,
            ClassType::UncSummoner(filter_state) => filter_state,
        }
    }

    pub fn n(&self) -> Self {
        match self {
            ClassType::Sorcerer(filter_state) => ClassType::Sorcerer(filter_state.n()),
            ClassType::Wizard(filter_state) => ClassType::Wizard(filter_state.n()),
            ClassType::Cleric(filter_state) => ClassType::Cleric(filter_state.n()),
            ClassType::Druid(filter_state) => ClassType::Druid(filter_state.n()),
            ClassType::Ranger(filter_state) => ClassType::Ranger(filter_state.n()),
            ClassType::Bard(filter_state) => ClassType::Bard(filter_state.n()),
            ClassType::Paladin(filter_state) => ClassType::Paladin(filter_state.n()),
            ClassType::Alchemist(filter_state) => ClassType::Alchemist(filter_state.n()),
            ClassType::Summoner(filter_state) => ClassType::Summoner(filter_state.n()),
            ClassType::Witch(filter_state) => ClassType::Witch(filter_state.n()),
            ClassType::Inquisitor(filter_state) => ClassType::Inquisitor(filter_state.n()),
            ClassType::Oracle(filter_state) => ClassType::Oracle(filter_state.n()),
            ClassType::Antipaladin(filter_state) => ClassType::Antipaladin(filter_state.n()),
            ClassType::Magus(filter_state) => ClassType::Magus(filter_state.n()),
            ClassType::Adept(filter_state) => ClassType::Adept(filter_state.n()),
            ClassType::Bloodrager(filter_state) => ClassType::Bloodrager(filter_state.n()),
            ClassType::Shaman(filter_state) => ClassType::Shaman(filter_state.n()),
            ClassType::Psychic(filter_state) => ClassType::Psychic(filter_state.n()),
            ClassType::Medium(filter_state) => ClassType::Medium(filter_state.n()),
            ClassType::Mesmerist(filter_state) => ClassType::Mesmerist(filter_state.n()),
            ClassType::Occultist(filter_state) => ClassType::Occultist(filter_state.n()),
            ClassType::Spiritualist(filter_state) => ClassType::Spiritualist(filter_state.n()),
            ClassType::Skald(filter_state) => ClassType::Skald(filter_state.n()),
            ClassType::Investigator(filter_state) => ClassType::Investigator(filter_state.n()),
            ClassType::Hunter(filter_state) => ClassType::Hunter(filter_state.n()),
            ClassType::UncSummoner(filter_state) => ClassType::UncSummoner(filter_state.n()),
        }
    }

    pub fn p(&self) -> Self {
        match self {
            ClassType::Sorcerer(filter_state) => ClassType::Sorcerer(filter_state.p()),
            ClassType::Wizard(filter_state) => ClassType::Wizard(filter_state.p()),
            ClassType::Cleric(filter_state) => ClassType::Cleric(filter_state.p()),
            ClassType::Druid(filter_state) => ClassType::Druid(filter_state.p()),
            ClassType::Ranger(filter_state) => ClassType::Ranger(filter_state.p()),
            ClassType::Bard(filter_state) => ClassType::Bard(filter_state.p()),
            ClassType::Paladin(filter_state) => ClassType::Paladin(filter_state.p()),
            ClassType::Alchemist(filter_state) => ClassType::Alchemist(filter_state.p()),
            ClassType::Summoner(filter_state) => ClassType::Summoner(filter_state.p()),
            ClassType::Witch(filter_state) => ClassType::Witch(filter_state.p()),
            ClassType::Inquisitor(filter_state) => ClassType::Inquisitor(filter_state.p()),
            ClassType::Oracle(filter_state) => ClassType::Oracle(filter_state.p()),
            ClassType::Antipaladin(filter_state) => ClassType::Antipaladin(filter_state.p()),
            ClassType::Magus(filter_state) => ClassType::Magus(filter_state.p()),
            ClassType::Adept(filter_state) => ClassType::Adept(filter_state.p()),
            ClassType::Bloodrager(filter_state) => ClassType::Bloodrager(filter_state.p()),
            ClassType::Shaman(filter_state) => ClassType::Shaman(filter_state.p()),
            ClassType::Psychic(filter_state) => ClassType::Psychic(filter_state.p()),
            ClassType::Medium(filter_state) => ClassType::Medium(filter_state.p()),
            ClassType::Mesmerist(filter_state) => ClassType::Mesmerist(filter_state.p()),
            ClassType::Occultist(filter_state) => ClassType::Occultist(filter_state.p()),
            ClassType::Spiritualist(filter_state) => ClassType::Spiritualist(filter_state.p()),
            ClassType::Skald(filter_state) => ClassType::Skald(filter_state.p()),
            ClassType::Investigator(filter_state) => ClassType::Investigator(filter_state.p()),
            ClassType::Hunter(filter_state) => ClassType::Hunter(filter_state.p()),
            ClassType::UncSummoner(filter_state) => ClassType::UncSummoner(filter_state.p()),
        }
    }

    pub fn test_cls(&self, spell: &Spell) -> bool {
        match self {
            ClassType::Sorcerer(FilterState::Positive) => spell.sor.is_some(),
            ClassType::Wizard(FilterState::Positive) => spell.wiz.is_some(),
            ClassType::Cleric(FilterState::Positive) => spell.cleric.is_some(),
            ClassType::Druid(FilterState::Positive) => spell.druid.is_some(),
            ClassType::Ranger(FilterState::Positive) => spell.ranger.is_some(),
            ClassType::Bard(FilterState::Positive) => spell.bard.is_some(),
            ClassType::Paladin(FilterState::Positive) => spell.paladin.is_some(),
            ClassType::Alchemist(FilterState::Positive) => spell.alchemist.is_some(),
            ClassType::Summoner(FilterState::Positive) => spell.summoner.is_some(),
            ClassType::Witch(FilterState::Positive) => spell.witch.is_some(),
            ClassType::Inquisitor(FilterState::Positive) => spell.inquisitor.is_some(),
            ClassType::Oracle(FilterState::Positive) => spell.oracle.is_some(),
            ClassType::Antipaladin(FilterState::Positive) => spell.antipaladin.is_some(),
            ClassType::Magus(FilterState::Positive) => spell.magus.is_some(),
            ClassType::Adept(FilterState::Positive) => spell.adept.is_some(),
            ClassType::Bloodrager(FilterState::Positive) => spell.bloodrager.is_some(),
            ClassType::Shaman(FilterState::Positive) => spell.shaman.is_some(),
            ClassType::Psychic(FilterState::Positive) => spell.psychic.is_some(),
            ClassType::Medium(FilterState::Positive) => spell.medium.is_some(),
            ClassType::Mesmerist(FilterState::Positive) => spell.mesmerist.is_some(),
            ClassType::Occultist(FilterState::Positive) => spell.occultist.is_some(),
            ClassType::Spiritualist(FilterState::Positive) => spell.spiritualist.is_some(),
            ClassType::Skald(FilterState::Positive) => spell.skald.is_some(),
            ClassType::Investigator(FilterState::Positive) => spell.investigator.is_some(),
            ClassType::Hunter(FilterState::Positive) => spell.hunter.is_some(),
            ClassType::UncSummoner(FilterState::Positive) => spell.summoner_unchained.is_some(),
            ClassType::Sorcerer(FilterState::Negative) => spell.sor.is_none(),
            ClassType::Wizard(FilterState::Negative) => spell.wiz.is_none(),
            ClassType::Cleric(FilterState::Negative) => spell.cleric.is_none(),
            ClassType::Druid(FilterState::Negative) => spell.druid.is_none(),
            ClassType::Ranger(FilterState::Negative) => spell.ranger.is_none(),
            ClassType::Bard(FilterState::Negative) => spell.bard.is_none(),
            ClassType::Paladin(FilterState::Negative) => spell.paladin.is_none(),
            ClassType::Alchemist(FilterState::Negative) => spell.alchemist.is_none(),
            ClassType::Summoner(FilterState::Negative) => spell.summoner.is_none(),
            ClassType::Witch(FilterState::Negative) => spell.witch.is_none(),
            ClassType::Inquisitor(FilterState::Negative) => spell.inquisitor.is_none(),
            ClassType::Oracle(FilterState::Negative) => spell.oracle.is_none(),
            ClassType::Antipaladin(FilterState::Negative) => spell.antipaladin.is_none(),
            ClassType::Magus(FilterState::Negative) => spell.magus.is_none(),
            ClassType::Adept(FilterState::Negative) => spell.adept.is_none(),
            ClassType::Bloodrager(FilterState::Negative) => spell.bloodrager.is_none(),
            ClassType::Shaman(FilterState::Negative) => spell.shaman.is_none(),
            ClassType::Psychic(FilterState::Negative) => spell.psychic.is_none(),
            ClassType::Medium(FilterState::Negative) => spell.medium.is_none(),
            ClassType::Mesmerist(FilterState::Negative) => spell.mesmerist.is_none(),
            ClassType::Occultist(FilterState::Negative) => spell.occultist.is_none(),
            ClassType::Spiritualist(FilterState::Negative) => spell.spiritualist.is_none(),
            ClassType::Skald(FilterState::Negative) => spell.skald.is_none(),
            ClassType::Investigator(FilterState::Negative) => spell.investigator.is_none(),
            ClassType::Hunter(FilterState::Negative) => spell.hunter.is_none(),
            ClassType::UncSummoner(FilterState::Negative) => spell.summoner_unchained.is_none(),
            _ => false,
        }
    }

    pub fn create_btn(&self, ui: &mut egui::Ui) -> Self {
        let btn = egui::Button::new(self.title());
        let resp = ui.add(if self.some_filter() {
            btn.fill(self.get_contained().get_color())
        } else {
            btn
        });

        if resp.clicked() {
            self.n()
        } else if resp.secondary_clicked() {
            self.p()
        } else {
            self.clone()
        }
    }
}
