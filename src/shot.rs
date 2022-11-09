use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::frame::{Drawable, Frame, FrameMsg};
use crate::{position::Position, Directions, NUM_COLS, NUM_ROWS};

pub struct Shot {
    pub position: Position,
    pub attach_damage: u64,
    pub direction: Directions,
    pub exploading: bool,
    frame: u8,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize, direction: Directions) -> Self {
        Self {
            position: Position::new(x, y, None),
            attach_damage: 10,
            direction,
            exploading: false,
            frame: 4,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploading {
            if self.frame == 0 {
                self.explode();
            } else {
                match self.direction {
                    Directions::Up => {
                        self.position.move_up();
                    }
                    Directions::Down => {
                        self.position.move_down();
                    }
                    Directions::Left => {
                        self.position.move_left();
                    }
                    Directions::Right => {
                        self.position.move_right();
                    }
                };
                self.frame -= 1;
                self.timer.reset();
            }
        }
    }

    pub fn explode(&mut self) {
        self.exploading = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn dead(&self) -> bool {
        (self.exploading && self.timer.ready)
            || self.frame == 0
            || self.position.y == 0
            || self.position.x == 0
            || self.position.y == NUM_ROWS - 1
            || self.position.x == NUM_COLS - 1
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        match self.direction {
            Directions::Up => {
                frame[self.position.x][self.position.y] = FrameMsg::Str(" ͡");
            }
            Directions::Down => {
                frame[self.position.x][self.position.y] = FrameMsg::Str(" ͝");
            }
            Directions::Left => {
                frame[self.position.x][self.position.y] = FrameMsg::Str("(");
            }
            Directions::Right => {
                frame[self.position.x][self.position.y] = FrameMsg::Str(")");
            }
        }
    }
}
