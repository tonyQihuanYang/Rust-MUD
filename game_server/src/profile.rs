use crate::position::Position;

#[derive(Clone, Debug)]
pub struct Profile {
    pub id: i32,
    pub position: Position,
    pub exp: u64,
    pub name: String,
}

impl Profile {
    pub fn new(id: i32, pos: Position) -> Self {
        Self {
            id,
            exp: 0,
            name: "UserName".to_string(),
            position: pos,
        }
    }

    pub fn gain_exp(&mut self, exp: u64) {
        self.exp += exp;
    }
}
