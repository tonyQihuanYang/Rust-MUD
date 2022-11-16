use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MonsterJson {
    pub id: u32,
    pub name: String,
    pub health: u64,
    pub x: usize,
    pub y: usize,
    pub exp: u64,
    pub respawn_time: u64,
}
