use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crossterm::style::{StyledContent, Stylize};

#[derive(Clone)]
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

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
