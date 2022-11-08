use super::monster::Monster;
use crate::{
    frame::{Drawable, Frame},
    MONSTERS_LIST_X, MONSTERS_LIST_Y, NUM_COLS, NUM_ROWS,
};
use rusty_time::prelude::Timer;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    thread,
};

const MONSTERS_JSON: &str = r#"
            [
              {
                "id": 1,
                "name": "Spider",
                "health": 40, 
                "x": 4,
                "y": 6,
                "exp": 100,
                "respawn_time": 5000
              },
              {
                "id": 2,
                "name": "Budge Dragon",
                "health": 80, 
                "x": 8,
                "y": 10,
                "exp": 120,
                "respawn_time": 5000
              }
            ]
        "#;

pub struct Monsters {
    pub enemies: Arc<Mutex<Vec<Monster>>>,
    pub tx: Sender<String>,
    monsters_lookup: Vec<Monster>,
    move_timer: Timer,
    direction: i32,
}

impl Monsters {
    pub fn new(tx: Sender<String>) -> Self {
        Self {
            enemies: Arc::new(Mutex::new(serde_json::from_str(MONSTERS_JSON).unwrap())),
            monsters_lookup: serde_json::from_str(MONSTERS_JSON).unwrap(),
            move_timer: Timer::from_millis(500),
            direction: 1,
            tx,
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
                monster.walk();
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

    pub fn kill_monster_at(&mut self, x: usize, y: usize) -> Option<Monster> {
        let enemies_lock = Arc::clone(&self.enemies);
        let mut enemies = enemies_lock.lock().unwrap();
        if let Some(idx) = enemies
            .iter()
            .position(|monster| (monster.x == x) && (monster.y == y))
        {
            self.tx.send("GETTING ATTACK".to_string()).unwrap();
            enemies[idx].be_attacked(1);
            if enemies[idx].is_dead() {
                let enemy_killed = enemies[idx].clone();
                enemies.remove(idx.clone());
                self.respawn(idx);
                Some(enemy_killed)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Drawable for Monsters {
    fn draw(&self, frame: &mut Frame) {
        let enemies = Arc::clone(&self.enemies);
        let data = enemies.lock().unwrap();
        for (index, monster) in data.iter().enumerate() {
            frame[monster.x][monster.y] = monster.id.to_string();
            frame[MONSTERS_LIST_X][MONSTERS_LIST_Y as usize + index] =
                format!("{}(HP:{})", monster.name.clone(), monster.health.clone());
        }
    }
}
