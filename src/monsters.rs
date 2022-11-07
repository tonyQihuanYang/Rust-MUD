use crate::{
    frame::{Drawable, Frame},
    Directions, NUM_COLS, NUM_ROWS,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusty_time::prelude::Timer;
use serde::Deserialize;
use std::{cmp::max, time::Duration};
use std::{
    sync::{Arc, Mutex},
    thread,
};

const MONSTERS_JSON: &str = r#"
            [
              {
                "id": 1,
                "x": 4,
                "y": 6,
                "respawn_time": 5000
              },
              {
                "id": 2,
                "x": 8,
                "y": 10,
                "respawn_time": 5000
              }
            ]
        "#;

#[derive(Deserialize, Clone, Debug)]
pub struct Monster {
    pub id: u8,
    pub x: usize,
    pub y: usize,
    respawn_time: u64,
}

pub struct Monsters {
    pub enemies: Arc<Mutex<Vec<Monster>>>,
    monsters_lookup: Vec<Monster>,
    move_timer: Timer,
    direction: i32,
}

impl Monsters {
    pub fn new() -> Self {
        Self {
            enemies: Arc::new(Mutex::new(serde_json::from_str(MONSTERS_JSON).unwrap())),
            monsters_lookup: serde_json::from_str(MONSTERS_JSON).unwrap(),
            move_timer: Timer::from_millis(500),
            direction: 1,
        }
    }

    pub fn respawn(&mut self, index: usize) {
        let monster = self.monsters_lookup[index].clone();
        let mut respawn_timer = Timer::from_millis(monster.respawn_time);
        let enemies_lock = Arc::clone(&self.enemies);
        thread::spawn(move || loop {
            respawn_timer.update(Duration::from_millis(1000));
            if respawn_timer.ready {
                let mut enemies = enemies_lock.lock().unwrap();
                enemies.push(monster);
                break;
            }
            thread::sleep(Duration::from_millis(1000));
        });
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let enemies_lock = Arc::clone(&self.enemies);
            let mut enemies = enemies_lock.lock().unwrap();
            for monster in enemies.iter_mut() {
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
        let enemies_lock = Arc::clone(&self.enemies);
        let enemies = enemies_lock.lock().unwrap();
        enemies.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        let enemies_lock = Arc::clone(&self.enemies);
        let enemies = enemies_lock.lock().unwrap();
        enemies.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        let enemies_lock = Arc::clone(&self.enemies);
        let mut enemies = enemies_lock.lock().unwrap();
        if let Some(idx) = enemies
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            enemies.remove(idx.clone());
            self.respawn(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Monsters {
    fn draw(&self, frame: &mut Frame) {
        let enemies = Arc::clone(&self.enemies);
        let data = enemies.lock().unwrap();
        for invader in data.iter() {
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
