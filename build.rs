use std::fmt;
use std::fs;
use std::include;
use std::path::Path;

include!("spell-build.rs");

fn main() {
    let spells = load_spell_table();
    let mut cont = "";
    //cont += &format!("pub const ALL_SPELLS: [Spell; {}] = [\n", spells.len());
    //for spell in spells {
    //    cont += &spell.to_string();
    //    cont += ",\n";
    //}
    //cont += "];\n";

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
