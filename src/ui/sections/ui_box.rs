use crate::position::{self, Position};
use crate::ui::frame::{to_string, Frame, FrameMsg};
use crossterm::style::Stylize;
/**
 * -------  ROW  -----
 * -------  ROW  -----
 *   |       |
 * Column  Column
 *   |       |
 */

#[derive(Clone)]
pub struct UiBoxMsg {
    pub value: FrameMsg,
    pub position: Option<Position>,
}

#[derive(Clone)]
pub struct UiBox {
    pub x_start: usize,
    pub x_end: usize,
    pub y_start: usize,
    pub y_end: usize,
}

impl UiBox {
    pub fn new(x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> Self {
        Self {
            x_start,
            x_end,
            y_start,
            y_end,
        }
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

    pub fn draw_frame(&self, frame: &mut Frame, messages: &Vec<UiBoxMsg>) {
        for (index, msg) in messages.iter().enumerate() {
            if let Some(position) = (*msg).position.clone() {
                frame[position.x][position.y] = msg.value.clone();
            } else {
                let char_vec: Vec<char> = to_string(&msg.value).chars().collect();
                for (char_i, c) in char_vec.iter().enumerate() {
                    // TODO ADD logic to swap line if too long
                    // TODO, FIX COLOR
                    frame[self.x_start + 1 + char_i][self.y_start + 1 + index] =
                        FrameMsg::StyledString(c.to_string().red());
                }
            }
        }
    }

    pub fn clear_frame(&self, frame: &mut Frame) {
        for x in self.x_start + 1..self.x_end - 1 {
            for y in self.y_start + 1..self.y_end {
                frame[x][y] = FrameMsg::Str(" ");
            }
        }
    }

    pub fn get_y_capacity(&self) -> usize {
        self.y_end - self.y_start - 1
    }
}
