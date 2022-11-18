use super::gear_profile::GearId;
use serde::Deserialize;
use std::collections::HashMap;

pub type MonsterId = u32;

#[derive(Deserialize, Clone, Debug)]
pub struct MonsterProfile {
    pub id: MonsterId,
    pub name: String,
    pub health: u64,
    pub exp: u64,
    pub fall_off: HashMap<GearId, f32>,
    pub max_fall_off: usize,
}
