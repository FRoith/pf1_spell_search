use convert_case::{Case, Casing};
use std::fs;
use std::include;

use html_to_struct::SpellDescriptionStruct;

include!("spell-build.rs");

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=spell-build.rs");
    println!("cargo::rerun-if-changed=db/spell_full - Updated 31Mar2020.csv");

    let spells = load_spell_table();
    let mut cont = String::new();
    cont += "use std::collections::HashMap;\n\n";
    cont += "#[derive(Debug, Clone)]
pub enum SpellDescriptionStruct {
    Body(Vec<SpellDescriptionStruct>),
    Paragraph(Vec<SpellDescriptionStruct>),
    Caption(Vec<SpellDescriptionStruct>),
    Italics(&'static str),
    Bold(&'static str),
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
    Text(&'static str),
}\n\n";
    cont += "lazy_static! {
    pub static ref BONUS_INFO: HashMap<u32, SpellMeta> = {
        let mut m = HashMap::new();\n";
    for spell in spells {
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
        cont += &format!("        m.insert({}_u32, SpellMeta{{d20pfsrd: {:?}, archives: {:?}, description_struct: {:?}.clone()}});\n", spell.id, d20pfsrd, archives, description_struct);
    }
    cont += "        m\n};\n}\n";

    fs::write("spell-generated.rs", cont).unwrap();
}

fn load_spell_table() -> Vec<Spell> {
    let data = include_str!("db/spell_full - Updated 31Mar2020.csv");
    let mut reader = csv::ReaderBuilder::new()
        .terminator(csv::Terminator::CRLF)
        .from_reader(data.as_bytes());
    reader.deserialize().filter_map(|x| x.ok()).collect()
}
