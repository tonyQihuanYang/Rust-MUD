use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::frame::{Drawable, Frame};
use crate::{Directions, NUM_COLS, NUM_ROWS};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub direction: Directions,
    pub exploading: bool,
    frame: usize,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize, direction: Directions) -> Self {
        Self {
            x,
            y,
            direction,
            exploading: false,
            frame: 4,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploading {
            match self.direction {
                Directions::Up => {
                    if self.y < NUM_ROWS {
                        self.y -= 1;
                    }
                }
                Directions::Down => {
                    if self.y > 0 {
                        self.y += 1;
                    }
                }
                Directions::Left => {
                    if self.x > 0 {
                        self.x -= 1;
                    }
                }
                Directions::Right => {
                    if self.x < NUM_COLS {
                        self.x += 1;
                    }
                }
            };
            self.frame -= 1;
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploading = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn dead(&self) -> bool {
        (self.exploading && self.timer.ready)
            || self.frame == 0
            || self.y == 0
            || self.x == 0
            || self.y == NUM_ROWS - 1
            || self.x == NUM_COLS - 1
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        match self.direction {
            Directions::Up => {
                frame[self.x][self.y] = " ͡".to_string();
            }
            Directions::Down => {
                frame[self.x][self.y] = " ͝".to_string();
            }
            Directions::Left => {
                frame[self.x][self.y] = "(".to_string();
            }
            Directions::Right => {
                frame[self.x][self.y] = ")".to_string();
            }
        }
    }
}

