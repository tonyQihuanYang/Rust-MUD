use crate::{
    frame::{Drawable, Frame},
    Directions, NUM_COLS, NUM_ROWS,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusty_time::prelude::Timer;
use serde::Deserialize;
use std::{cmp::max, time::Duration};

#[derive(Deserialize, Debug)]
pub struct Monster {
    pub id: u8,
    pub x: usize,
    pub y: usize,
}

pub struct Monsters {
    pub army: Vec<Monster>,
    move_timer: Timer,
    direction: i32,
}

impl Monsters {
    pub fn new() -> Self {
        let data = r#"
            [
              {
                "id": 1,
                "x": 4,
                "y": 6
              },
              {
                "id": 2,
                "x": 8,
                "y": 10
              }
            ]
        "#;
        let army: Vec<Monster> = serde_json::from_str(data).unwrap();

        Self {
            army,
            move_timer: Timer::from_millis(500),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            for monster in self.army.iter_mut() {
                let directions = vec![
                    Directions::Up,
                    Directions::Left,
                    Directions::Right,
                    Directions::Down,
                ];
                //TODO move it into monster case itself
                match directions.choose(&mut rand::thread_rng()) {
                    Some(d) => match d {
                        Directions::Up => {
                            if monster.y + 1 < NUM_ROWS {
                                monster.y = ((monster.y as i32) - 1) as usize;
                            }
                        }
                        Directions::Down => {
                            if monster.y - 1 > 0 {
                                monster.y = ((monster.y as i32) + 1) as usize;
                            }
                        }
                        Directions::Left => {
                            if monster.x - 1 > 0 {
                                monster.x = ((monster.x as i32) - 1) as usize;
                            }
                        }
                        Directions::Right => {
                            if monster.x + 1 < NUM_COLS {
                                monster.x = ((monster.x as i32) + 1) as usize;
                            }
                        }
                    },
                    None => (),
                };
            }
            return true;
        }
        false
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Monsters {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                invader.id.to_string()
            } else {
                invader.id.to_string()
            }
        }
    }
}
