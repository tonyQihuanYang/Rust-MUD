use crate::{Directions, NUM_COLS, NUM_ROWS};
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Monster {
    pub id: u8,
    pub name: String,
    pub health: u64,
    pub x: usize,
    pub y: usize,
    pub exp: u64,
    pub respawn_time: u64,
}

impl Monster {
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn be_attacked(&mut self, damage: u64) {
        // println!("{} - {}!", self.health, damage);
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }

    pub fn walk(&mut self) {
        let directions = vec![
            Directions::Up,
            Directions::Left,
            Directions::Right,
            Directions::Down,
        ];
        match directions.choose(&mut rand::thread_rng()) {
            Some(d) => match d {
                Directions::Up => {
                    if self.y + 1 < NUM_ROWS {
                        self.y = ((self.y as i32) + 1) as usize;
                    }
                }
                Directions::Down => {
                    if self.y - 1 > 0 {
                        self.y = ((self.y as i32) - 1) as usize;
                    }
                }
                Directions::Left => {
                    if self.x - 1 > 0 {
                        self.x = ((self.x as i32) - 1) as usize;
                    }
                }
                Directions::Right => {
                    if self.x + 1 < NUM_COLS {
                        self.x = ((self.x as i32) + 1) as usize;
                    }
                }
            },
            None => (),
        };
    }
}
