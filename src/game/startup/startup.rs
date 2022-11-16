use super::{
    json_models::{gear_json::GearJson, monster_json::MonsterJson},
    load_json,
};
pub fn load() -> Result<(Vec<GearJson>, Vec<MonsterJson>), String> {
    let data = load_json::load().expect("Loading Gear Failed");
    Ok(data)
}
