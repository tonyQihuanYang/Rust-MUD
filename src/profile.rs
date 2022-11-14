use crate::position::Position;

#[derive(Clone, Debug)]
pub struct Profile {
    pub id: u32,
    pub position: Position,
    pub exp: u64,
    pub name: String,
}

impl Profile {
    pub fn new(pos: Position) -> Self {
        Self {
            id: 0,
            exp: 0,
            name: "UserName".to_string(),
            position: pos,
        }
    }

    pub fn gain_exp(&mut self, exp: u64) {
        self.exp += exp;
    }
}
