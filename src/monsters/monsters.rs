use super::monster::Monster;
use crate::{
    commands::{Cmds, MonsterCmds},
    position::Position,
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
                "id": 10000,
                "name": "Spider",
                "health": 40, 
                "x": 4,
                "y": 6,
                "exp": 100,
                "respawn_time": 5000
              },
              {
                "id": 10001,
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
    game_log_tx: Sender<Cmds>,
    pub enemies: Arc<Mutex<Vec<Monster>>>,
    monsters_lookup: Vec<Monster>,
    move_timer: Timer,
}

impl Monsters {
    pub fn new(game_log_tx: Sender<Cmds>) -> Self {
        Self {
            game_log_tx,
            enemies: Arc::new(Mutex::new(serde_json::from_str(MONSTERS_JSON).unwrap())),
            monsters_lookup: serde_json::from_str(MONSTERS_JSON).unwrap(),
            move_timer: Timer::from_millis(500),
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

    pub fn update(&mut self, delta: Duration) {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let enemies_lock = Arc::clone(&self.enemies);
            let mut enemies = enemies_lock.lock().unwrap();
            for monster in enemies.iter_mut() {
                monster.walk();
                self.game_log_tx
                    .send(Cmds::Monster(MonsterCmds::Move(
                        monster.id,
                        Position::new(monster.x, monster.y, None),
                    )))
                    .unwrap();
            }
        }
    }

    pub fn kill_monster_at(&mut self, x: usize, y: usize) -> Option<Monster> {
        let enemies_lock = Arc::clone(&self.enemies);
        let mut enemies = enemies_lock.lock().unwrap();

        if let Some(idx) = enemies
            .iter()
            .position(|monster| !monster.is_dead() && (monster.x == x) && (monster.y == y))
        {
            self.game_log_tx
                .send(Cmds::Monster(MonsterCmds::Damaged))
                .unwrap();
            enemies[idx].be_attacked(20);
            if enemies[idx].is_dead() {
                let enemy_killed = enemies[idx].clone();
                enemies.remove(idx.clone());
                self.game_log_tx
                    .send(Cmds::Monster(MonsterCmds::Dead(enemy_killed.get_profile())))
                    .unwrap();
                self.respawn(idx);
                Some(enemy_killed)
            } else {
                self.game_log_tx
                    .send(Cmds::Monster(MonsterCmds::Updated(enemies[idx].clone())))
                    .unwrap();
                None
            }
        } else {
            None
        }
    }
}
