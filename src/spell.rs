use lazy_static::lazy_static;
use std::include;

include!("../spell-build.rs");
include!("../spell-generated.rs");

lazy_static! {
    pub static ref ALL_SPELLS: Vec<Spell> = {
        let data = include_str!("../db/spells.csv");
        let mut reader = csv::ReaderBuilder::new()
            .terminator(csv::Terminator::CRLF)
            .from_reader(data.as_bytes());
        reader.deserialize().filter_map(|x| x.ok()).collect()
    };
}
