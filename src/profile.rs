use crate::{frame::Drawable, PROFILE_X};
pub struct Profile {
    pub id: u8,
    pub exp: u8,
    pub name: String,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: 0,
            exp: 0,
            name: "userName".to_string(),
        }
    }
}

impl Drawable for Profile {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[PROFILE_X][0] = self.name.clone();
        frame[PROFILE_X][1] = self.exp.to_string();
    }
}
