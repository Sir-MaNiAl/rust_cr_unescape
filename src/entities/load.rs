use serde::Deserialize;
use std::{collections::BTreeMap, fs::File, io, io::prelude::*, path::Path};

const ENTITIES_FILE_PATH: &str = "src/entities/entities.json";

pub fn load() -> io::Result<BTreeMap<String, String>> {
    let player_file_path = Path::new(ENTITIES_FILE_PATH);

    let mut f = File::open(player_file_path)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let entities: Entities = serde_json::from_str(&buffer)?;
    let characters = entities.characters;

    Ok(characters)
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Entities {
    characters: BTreeMap<String, String>,
    #[serde(rename(deserialize = "optional-;"))]
    optional: Vec<String>,
}
