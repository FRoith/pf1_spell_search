use convert_case::{Case, Casing};
use std::fs;
use std::include;

include!("spell-build.rs");

fn main() {
    let spells = load_spell_table();
    let mut cont = String::new();
    //cont += &format!("pub const ALL_SPELLS: [Spell; {}] = [\n", spells.len());
    //for spell in spells {
    //    cont += &spell.to_string();
    //    cont += ",\n";
    //}
    //cont += "];\n";
    cont += "use std::collections::HashMap;
    lazy_static! {
    pub static ref BONUS_INFO: HashMap<u32, &'static SpellMeta> = {
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
        let description_md =
            &html2md::parse_html(&spell.description_formatted).replace("\n ", "\n\n");
        cont += &format!("        m.insert({}_u32, &SpellMeta{{d20pfsrd: {:?}, archives: {:?}, description_md: {:?}}});\n", spell.id, d20pfsrd, archives, description_md);
    }
    cont += "        m\n};\n}\n";

    fs::write("spell-generated.rs", cont).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}

fn load_spell_table() -> Vec<Spell> {
    let data = include_str!("db/spell_full - Updated 31Mar2020.csv");
    let mut reader = csv::ReaderBuilder::new()
        .terminator(csv::Terminator::CRLF)
        .from_reader(data.as_bytes());
    reader.deserialize().filter_map(|x| x.ok()).collect()
}
