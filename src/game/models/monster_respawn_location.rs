use super::monster_profile::MonsterId;
use crate::position::Position;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct MonsterRespawnLocation {
    pub id: MonsterId,
    pub respawn_position: Position,
    pub respawn_time: u64,
}
