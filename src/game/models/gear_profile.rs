use serde::Deserialize;

pub type GearId = u32;

#[derive(Clone, Debug, Deserialize)]
pub struct GearProfile {
    pub id: GearId,
    pub name: String,
}
