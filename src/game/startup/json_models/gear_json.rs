use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GearJson {
    pub id: u32,
    pub name: String,
}
