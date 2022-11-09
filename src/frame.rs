use crate::{MONSTERS_LIST_X, MONSTERS_LIST_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crossterm::style::{StyledContent, Stylize};

pub enum FrameMsg {
    String(String),
    Str(&'static str),
    StyledString(StyledContent<String>),
    StyledStr(StyledContent<&'static str>),
}

pub type Frame = Vec<Vec<FrameMsg>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(WINDOW_WIDTH);

    for _ in 0..WINDOW_WIDTH {
        let mut col = Vec::with_capacity(WINDOW_HEIGHT);
        for _ in 0..WINDOW_HEIGHT {
            col.push(FrameMsg::Str(" "));
        }
        cols.push(col)
    }
    cols
}

pub fn to_string(msg: &FrameMsg) -> std::string::String {
    match msg {
        FrameMsg::String(val) => val.clone(),
        FrameMsg::Str(val) => val.to_string(),
        FrameMsg::StyledString(val) => val.to_string(),
        FrameMsg::StyledStr(val) => val.to_string(),
    }
}

pub fn clear_monster_list(cur_frame: &mut Frame) {
    // for y in MONSTERS_LIST_Y..WINDOW_HEIGHT {
    //     cur_frame[MONSTERS_LIST_X][y] = " ".to_string().repeat(WINDOW_WIDTH - MONSTERS_LIST_X);
    // }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
