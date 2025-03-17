use convert_case::{Case, Casing};
use egui_extras::{Column, TableBuilder, TableRow};
use filter_derive::FilterReprMacro;
use filter_repr::FilterRepr;

use regex::Regex;
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

use crate::{
    filters::{
        Domain, FilterState, Level, Save, SpellComponent, SpellDescriptor, SpellRange,
        SpellResistance, Spellschool, Subschool,
    },
    util::toggle,
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
    fn title(&self) -> String {
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

    fn get_value(&self, spell: &Spell) -> Option<u32> {
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

    fn get_all() -> Vec<Self> {
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

    fn get_contained(&self) -> &FilterState {
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

    fn test_cls(&self, spell: &Spell) -> bool {
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

    fn create_btn(&self, ui: &mut egui::Ui) -> Self {
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

impl Spell {
    fn filter_map_level(
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
                    self.spell_table
                        .filter_window
                        .filter_ui(ctx, &mut self.filter_window_active);
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
            filter_string: String::new(),
            selected_spell: None,
            filter_window: FilterWindow::new(),
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
                        match col {
                            ColType::Name(_) => {
                                ui.menu_button("🔎", |ui| {
                                    ui.text_edit_singleline(&mut self.filter_window.name)
                                });
                            }
                            ColType::Description(_) => {
                                ui.menu_button("🔎", |ui| {
                                    ui.text_edit_singleline(&mut self.filter_window.description)
                                });
                            }
                            _ => {}
                        }

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

    fn render_body(&mut self, body: egui_extras::TableBody<'_>) {
        let mut stuff: Vec<(&Spell, String)> = self
            .value
            .iter()
            .filter_map(|spell| {
                spell.filter_map_level(
                    self.filter_window.class_or,
                    &self.filter_window.selected_classes,
                )
            })
            .filter(|(spell, level)| self.filter_window.test(spell, level))
            .collect();
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
            ui.add(
                egui::Hyperlink::from_label_and_url(
                    egui::RichText::new("(d20PFsrd)").size(10.0),
                    format!(
                        "https://www.d20pfsrd.com/magic/all-spells/{}/{}/",
                        spell.name.to_lowercase().chars().next().unwrap(),
                        spell.name.to_case(Case::Kebab),
                    ),
                )
                .open_in_new_tab(true),
            );
            ui.add(
                egui::Hyperlink::from_label_and_url(
                    egui::RichText::new("(Archives)").size(10.0),
                    format!(
                        "https://aonprd.com/SpellDisplay.aspx?ItemName={}",
                        spell.name.to_lowercase(),
                    ),
                )
                .open_in_new_tab(true),
            );
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
    source: String,
    selected_classes: Vec<ClassType>,
    class_or: bool,
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
            source: String::new(),
            selected_classes: ClassType::get_all(),
            class_or: false,
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
                    ui.text_edit_singleline(&mut self.name);
                });
                ui.separator();
                ui.label("Description");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.description);
                    if self.description != self.prev_description {
                        self.keywords = self
                            .description
                            .to_lowercase()
                            .split(",")
                            .map(|x| Regex::new(&format!("\\b{}\\b", x)))
                            .collect();
                    }
                    self.prev_description = self.description.clone();
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Class");
                    ui.add(toggle(&mut self.class_or));
                });
                ui.horizontal_wrapped(|ui| {
                    for c in &mut self.selected_classes {
                        *c = c.create_btn(ui);
                    }
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
                    ui.label("Range");
                    ui.add(toggle(&mut self.range_or));
                });
                ui.horizontal_wrapped(|ui| {
                    for range in &mut self.range {
                        *range = range.create_btn(ui);
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
