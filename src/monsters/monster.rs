use crate::{position::Position, Directions, NUM_COLS, NUM_ROWS};
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Monster {
    pub id: u32,
    pub name: String,
    pub health: u64,
    pub x: usize,
    pub y: usize,
    pub exp: u64,
    pub respawn_time: u64,
}

#[derive(Clone, Debug)]
pub struct MonsterProfile {
    pub id: u32,
    pub name: String,
    pub health: u64,
    pub position: Position,
}

impl Monster {
    pub fn get_profile(&self) -> MonsterProfile {
        MonsterProfile {
            id: self.id,
            name: self.name.clone(),
            health: self.health,
            position: Position {
                x: self.x,
                y: self.y,
                bound: None,
            },
        }
    }
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn be_attacked(&mut self, damage: u64) {
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
