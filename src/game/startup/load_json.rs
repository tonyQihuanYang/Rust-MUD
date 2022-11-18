use crate::game::models::{
    monster_profile::{MonsterId, MonsterProfile},
    monster_respawn_location::MonsterRespawnLocation,
};

use super::json_models::gear_json::GearJson;
use std::{collections::HashMap, fs};

pub fn load() -> Result<
    (
        Vec<GearJson>,
        HashMap<MonsterId, MonsterProfile>,
        Vec<MonsterRespawnLocation>,
    ),
    String,
> {
    let gears = load_gears().expect("Load Gears Resource Failed");
    let monsters = load_monsters().expect("Load Monsters Dictionary Resource Failed");
    let monsters_respawn_location =
        load_monsters_respawn_location().expect("Load Monster Respawn Resource Failed");
    Ok((gears, monsters, monsters_respawn_location))
}

pub fn load_gears() -> Result<Vec<GearJson>, String> {
    let data = fs::read_to_string("./resource/json/gears.json").expect("Unable to read file");
    let gears: Vec<GearJson> = serde_json::from_str(&data).expect("Could not serde_json");
    Ok(gears)
}

pub fn load_monsters() -> Result<HashMap<MonsterId, MonsterProfile>, String> {
    let data = fs::read_to_string("./resource/json/monsters.json").expect("Unable to read file");
    let monsters: Vec<MonsterProfile> = serde_json::from_str(&data).expect("Unable to serde_json");
    let monsters_dict = monsters
        .into_iter()
        .map(|monster| (monster.id, monster))
        .collect::<HashMap<MonsterId, MonsterProfile>>();
    Ok(monsters_dict)
}

pub fn load_monsters_respawn_location() -> Result<Vec<MonsterRespawnLocation>, String> {
    let data =
        fs::read_to_string("./resource/json/monsters_location.json").expect("Unable to read file");
    let monsters_location: Vec<MonsterRespawnLocation> =
        serde_json::from_str(&data).expect("Unable to serde_json");
    Ok(monsters_location)
}
