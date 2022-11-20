use super::load_json;
use crate::game::models::{
    gear_profile::{GearId, GearProfile},
    monster_profile::{MonsterId, MonsterProfile},
    monster_respawn_location::MonsterRespawnLocation,
};
use std::collections::HashMap;
pub fn load() -> Result<
    (
        HashMap<GearId, GearProfile>,
        HashMap<MonsterId, MonsterProfile>,
        Vec<MonsterRespawnLocation>,
    ),
    String,
> {
    let (gears, monsters_dict, monsters_respawn_location) =
        load_json::load().expect("Loading Gear Failed");
    Ok((gears, monsters_dict, monsters_respawn_location))
}
