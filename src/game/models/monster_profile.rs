use serde::Deserialize;

pub type MonsterId = u32;

#[derive(Deserialize, Clone, Debug)]
pub struct MonsterProfile {
    pub id: MonsterId,
    pub name: String,
    pub health: u64,
    pub exp: u64,
}
