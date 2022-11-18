use crate::game::models::{
    monster_profile::{MonsterId, MonsterProfile},
    monster_respawn_location::MonsterRespawnLocation,
};
use std::collections::HashMap;

use super::{json_models::gear_json::GearJson, load_json};
pub fn load() -> Result<
    (
        Vec<GearJson>,
        HashMap<MonsterId, MonsterProfile>,
        Vec<MonsterRespawnLocation>,
    ),
    String,
> {
    let (gears, monsters_dict, monsters_respawn_location) =
        load_json::load().expect("Loading Gear Failed");
    Ok((gears, monsters_dict, monsters_respawn_location))
}
