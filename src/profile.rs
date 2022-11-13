use crate::commands::SendCmds;

#[derive(Clone, Debug)]
pub struct Profile {
    pub id: u32,
    pub exp: u64,
    pub name: String,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: 0,
            exp: 0,
            name: "UserName".to_string(),
        }
    }

    pub fn gain_exp(&mut self, exp: u64) {
        self.exp += exp;
    }
}

impl SendCmds for Profile {
    fn send(&self) {
        // frame[PROFILE_X][0] = FrameMsg::String(self.name.clone());
        // frame[PROFILE_X][1] = FrameMsg::String(self.exp.to_string());
    }
}
