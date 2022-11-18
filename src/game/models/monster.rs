use std::sync::atomic::{AtomicU32, Ordering};
static GLOBAL_MONSTERS_COUNT: AtomicU32 = AtomicU32::new(0);
use super::monster_profile::MonsterProfile;
use crate::{position::Position, Directions, NUM_COLS, NUM_ROWS};
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Monster {
    pub profile: MonsterProfile,
    pub id: u32,
    pub health: u64,
    //Fixme: change it to Position
    pub x: usize,
    pub y: usize,
}

impl Monster {
    pub fn new(profile: MonsterProfile, position: Position) -> Self {
        let new_monster_id = GLOBAL_MONSTERS_COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            profile: profile.clone(),
            id: new_monster_id,
            health: profile.health,
            x: position.x,
            y: position.y,
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
