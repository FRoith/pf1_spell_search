use filter_derive::FilterReprMacro;
use filter_repr::FilterRepr;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum FilterState {
    None,
    Positive,
    Negative,
}

impl FilterState {
    pub fn get_color(&self) -> egui::Color32 {
        match self {
            FilterState::None => egui::Color32::TRANSPARENT,
            FilterState::Positive => egui::Color32::from_hex("#008800").unwrap(),
            FilterState::Negative => egui::Color32::from_hex("#880000").unwrap(),
        }
    }

    pub fn n(&self) -> Self {
        match self {
            FilterState::None => FilterState::Positive,
            FilterState::Positive => FilterState::Negative,
            FilterState::Negative => FilterState::None,
        }
    }

    pub fn p(&self) -> Self {
        match self {
            FilterState::None => FilterState::Negative,
            FilterState::Positive => FilterState::None,
            FilterState::Negative => FilterState::Positive,
        }
    }

    pub fn test(&self, spell: &str, value: &str) -> bool {
        match self {
            FilterState::None => true,
            FilterState::Positive => spell.to_lowercase().contains(&value.to_lowercase()),
            FilterState::Negative => !spell.to_lowercase().contains(&value.to_lowercase()),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Spellschool {
    Abjuration(FilterState),
    Conjuration(FilterState),
    Divination(FilterState),
    Enchantment(FilterState),
    Evocation(FilterState),
    Illusion(FilterState),
    Necromancy(FilterState),
    Transmutation(FilterState),
    Universal(FilterState),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Subschool {
    Calling(FilterState),
    Creation(FilterState),
    Healing(FilterState),
    Summoning(FilterState),
    Teleportation(FilterState),
    Scrying(FilterState),
    Charm(FilterState),
    Compulsion(FilterState),
    Figment(FilterState),
    Glamer(FilterState),
    Pattern(FilterState),
    Phantasm(FilterState),
    Shadow(FilterState),
    Polymorph(FilterState),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Level {
    _0(FilterState),
    _1(FilterState),
    _2(FilterState),
    _3(FilterState),
    _4(FilterState),
    _5(FilterState),
    _6(FilterState),
    _7(FilterState),
    _8(FilterState),
    _9(FilterState),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Domain {
    Air(FilterState),
    Animal(FilterState),
    Artifice(FilterState),
    Chaos(FilterState),
    Charm(FilterState),
    Community(FilterState),
    Darkness(FilterState),
    Death(FilterState),
    Destruction(FilterState),
    Earth(FilterState),
    Erosion(FilterState),
    Evil(FilterState),
    Fire(FilterState),
    Glory(FilterState),
    Good(FilterState),
    Healing(FilterState),
    Knowledge(FilterState),
    Law(FilterState),
    Liberation(FilterState),
    Luck(FilterState),
    Madness(FilterState),
    Magic(FilterState),
    Nobility(FilterState),
    Plant(FilterState),
    Protection(FilterState),
    Repose(FilterState),
    Ruins(FilterState),
    Rune(FilterState),
    Scalykind(FilterState),
    Strength(FilterState),
    Sun(FilterState),
    Travel(FilterState),
    Trickery(FilterState),
    Vermin(FilterState),
    Void(FilterState),
    War(FilterState),
    Water(FilterState),
    Weather(FilterState),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Save {
    None(FilterState),
    Will(FilterState),
    Reflex(FilterState),
    Fortitude(FilterState),
    Text(FilterState),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellResistance {
    Yes(FilterState),
    No(FilterState),
    Text(FilterState),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellComponent {
    Verbal(FilterState),
    Somatic(FilterState),
    Material(FilterState),
}

impl SpellComponent {
    pub fn special_test(&self, spell: &crate::app::Spell) -> bool {
        match self {
            Self::Verbal(FilterState::None) => true,
            Self::Somatic(FilterState::None) => true,
            Self::Material(FilterState::None) => true,
            Self::Verbal(FilterState::Positive) => spell.verbal,
            Self::Somatic(FilterState::Positive) => spell.somatic,
            Self::Material(FilterState::Positive) => spell.material,
            Self::Verbal(FilterState::Negative) => !spell.verbal,
            Self::Somatic(FilterState::Negative) => !spell.somatic,
            Self::Material(FilterState::Negative) => !spell.material,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellDescriptor {
    Acid(FilterState),
    Air(FilterState),
    Chaotic(FilterState),
    Cold(FilterState),
    Curse(FilterState),
    Darkness(FilterState),
    Death(FilterState),
    Disease(FilterState),
    Draconic(FilterState),
    Earth(FilterState),
    Electricity(FilterState),
    Emotion(FilterState),
    Evil(FilterState),
    Fear(FilterState),
    Fire(FilterState),
    Force(FilterState),
    Good(FilterState),
    #[allow(non_camel_case_types)]
    Language_Dependent(FilterState),
    Lawful(FilterState),
    Light(FilterState),
    Meditative(FilterState),
    #[allow(non_camel_case_types)]
    Mind_Affecting(FilterState),
    Pain(FilterState),
    Poison(FilterState),
    Ruse(FilterState),
    Shadow(FilterState),
    Sonic(FilterState),
    Water(FilterState),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellRange {
    Personal(FilterState),
    Touch(FilterState),
    Close(FilterState),
    Medium(FilterState),
    Long(FilterState),
}
