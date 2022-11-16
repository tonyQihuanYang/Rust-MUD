use super::json_models::{gear_json::GearJson, monster_json::MonsterJson};
use std::fs;

pub fn load() -> Result<(Vec<GearJson>, Vec<MonsterJson>), String> {
    let gears = load_gears().expect("Load Gears Failed");
    let monsters = load_monsters().expect("Load Gears Failed");
    Ok((gears, monsters))
}

pub fn load_gears() -> Result<Vec<GearJson>, String> {
    let data = fs::read_to_string("./resource/json/gears.json").expect("Unable to read file");
    let gears: Vec<GearJson> = serde_json::from_str(&data).expect("Could not serde_json");
    Ok(gears)
}

pub fn load_monsters() -> Result<Vec<MonsterJson>, String> {
    let data = fs::read_to_string("./resource/json/monsters.json").expect("Unable to read file");
    let monsters: Vec<MonsterJson> = serde_json::from_str(&data).expect("Unable to serde_json");
    Ok(monsters)
}
