use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::fs;
use std::include;

use html_to_struct::SpellDescriptionStruct;

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
pub struct CsvSpell {
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
    #[serde(skip_serializing)]
    pub description_formatted: String,
    pub source: String,
    #[serde(skip_serializing)]
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

include!("spell-build.rs");

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=spell-build.rs");
    println!("cargo::rerun-if-changed=db/spells.csv");

    let spells = load_spell_table();
    let mut cont = String::new();
    cont += "use std::collections::HashMap;\n\n";
    cont += "#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
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
}\n\n";
    cont += "lazy_static! {
    pub static ref BONUS_INFO: HashMap<u32, SpellMeta> = {
        let s = include_str!(\"gen-db/spell_meta.json\");
        serde_json::from_str(s).unwrap()
    };
    
    pub static ref ALL_SPELLS: Vec<Spell> = {
        let s = include_str!(\"gen-db/spell.json\");
        serde_json::from_str(s).unwrap()
    };\n}\n";
    let spell_metas: HashMap<u32, SpellMeta> = spells
        .iter()
        .map(|spell| {
            let d20pfsrd = format!(
                "https://www.d20pfsrd.com/magic/all-spells/{}/{}/",
                spell.name.to_lowercase().chars().next().unwrap(),
                spell.name.to_case(Case::Kebab),
            );
            let archives = format!(
                "https://aonprd.com/SpellDisplay.aspx?ItemName={}",
                spell.name.to_lowercase(),
            );
            let description_struct = html_to_struct::process(&spell.description_formatted);
            /*
            cont += &format!("        m.insert({}_u32, SpellMeta{{d20pfsrd: {:?}, archives: {:?}, description_struct: {:?}.clone()}});\n", spell.id, d20pfsrd, archives, description_struct);
            */
            (
                spell.id,
                SpellMeta {
                    d20pfsrd,
                    archives,
                    description_struct,
                },
            )
        })
        .collect();

    fs::create_dir_all("gen-db").unwrap();
    save_spell_meta_table(spell_metas);
    save_spell_table(spells);
    fs::write("spell-generated.rs", cont).unwrap();
}

fn load_spell_table() -> Vec<CsvSpell> {
    let data = include_str!("db/spells.csv");
    let mut reader = csv::ReaderBuilder::new()
        .terminator(csv::Terminator::CRLF)
        .from_reader(data.as_bytes());
    reader.deserialize().filter_map(|x| x.ok()).collect()
}

fn save_spell_table(spells: Vec<CsvSpell>) {
    let s = serde_json::to_string(&spells).unwrap();
    fs::write("gen-db/spell.json", s).unwrap();
}

fn save_spell_meta_table(spell_metas: HashMap<u32, SpellMeta>) {
    let s = serde_json::to_string(&spell_metas).unwrap();
    fs::write("gen-db/spell_meta.json", s).unwrap();
}
