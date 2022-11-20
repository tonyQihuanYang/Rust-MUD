use super::frame::{to_string, Frame, FrameMsg};
use crossterm::{
    cursor::MoveTo,
    style::{style, Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::White)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if to_string(s) != to_string(&(last_frame[x][y])) || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                match s {
                    FrameMsg::String(msg) => {
                        print!("{}", msg);
                    }
                    FrameMsg::Str(msg) => {
                        print!("{}", msg);
                    }
                    FrameMsg::StyledStr(msg) => {
                        print!("{}", msg);
                    }
                    FrameMsg::StyledString(msg) => {
                        print!("{}", msg);
                    }
                }
            }
        }
    }

    stdout.flush().unwrap();
}
