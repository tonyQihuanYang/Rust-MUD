use crate::position::{self, Position};

use super::frame::{Drawable, Frame, FrameMsg};
use crossterm::style::Stylize;
use std::sync::{Arc, Mutex};
/**
 * -------  ROW  -----
 * -------  ROW  -----
 *   |       |
 * Column  Column
 *   |       |
 */

#[derive(Clone)]
pub struct SectionMsg {
    value: String,
    position: Option<Position>,
}

#[derive(Clone)]
pub struct Section {
    pub x_start: usize,
    pub x_end: usize,
    pub y_start: usize,
    pub y_end: usize,
    pub messages: Arc<Mutex<Vec<SectionMsg>>>,
}

impl Section {
    pub fn new(x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> Self {
        Self {
            x_start,
            x_end,
            y_start,
            y_end,
            messages: Arc::new(Mutex::new(Vec::with_capacity(y_end - y_start - 1))),
        }
    }

    pub fn add_str(&mut self, str: &str) {
        self.add_message(str.to_string(), None);
    }

    pub fn add_message(&mut self, message: String, position: Option<Position>) {
        self.push(SectionMsg {
            value: message,
            position,
        });
    }

    fn push(&self, value: SectionMsg) {
        let capacity = self.get_capacity();
        let messages_lock = Arc::clone(&self.messages);
        let mut messages = messages_lock.lock().unwrap();

        if messages.len() == capacity {
            messages.remove(capacity - 1);
        }
        messages.insert(0, value);
    }

    pub fn draw_outline(&self, frame: &mut Frame) {
        for x in self.x_start..self.x_end {
            frame[x][self.y_start] = FrameMsg::Str("-");
            frame[x][self.y_end] = FrameMsg::Str("-");
        }

        for y in (self.y_start + 1)..self.y_end {
            frame[self.x_start][y] = FrameMsg::Str("|");
            frame[self.x_end - 1][y] = FrameMsg::Str("|");
        }
    }

    //FIXME: Not cleaning...
    pub fn clear_frame(&self, frame: &mut Frame) {
        for x in self.x_start + 1..self.x_end - 1 {
            for y in self.y_start + 1..self.y_end {
                frame[x][y] = FrameMsg::Str(" ");
            }
        }
    }

    pub fn clear_messages(&self) {
        let messages_lock = Arc::clone(&self.messages);
        let mut messages = messages_lock.lock().unwrap();
        messages.clear();
    }

    fn get_capacity(&self) -> usize {
        self.y_end - self.y_start - 1
    }
}

impl Drawable for Section {
    fn draw(&self, frame: &mut Frame) {
        self.clear_frame(frame);
        let messages_lock = Arc::clone(&self.messages);
        let messages = messages_lock.lock().unwrap();
        for (index, msg) in messages.iter().enumerate() {
            let char_vec: Vec<char> = (*msg).value.chars().collect();

            if let Some(position) = (*msg).position.clone() {
                frame[position.x][position.y] = FrameMsg::StyledString((*msg).value.clone().red());
            } else {
                for (char_i, c) in char_vec.iter().enumerate() {
                    // TODO ADD logic to swap line if too long
                    frame[self.x_start + 1 + char_i][self.y_start + 1 + index] =
                        FrameMsg::StyledString(c.to_string().red());
                }
            }
        }
    }
}
