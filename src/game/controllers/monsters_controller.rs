use crate::game::models::{
    monster::Monster,
    monster_profile::{MonsterId, MonsterProfile},
    monster_respawn_location::MonsterRespawnLocation,
};
use crate::{
    commands::{Cmds, MonsterCmds},
    position::Position,
};
use rusty_time::prelude::Timer;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct MonstersControl {
    //FIX ME, change it to RWlock
    pub enemies: Arc<Mutex<Vec<Monster>>>,
    monsters_lookup: HashMap<MonsterId, MonsterProfile>,
    monsters_respawn_location: Vec<MonsterRespawnLocation>,
    game_log_tx: Sender<Cmds>,
    move_timer: Timer,
}

impl MonstersControl {
    pub fn new(
        game_log_tx: Sender<Cmds>,
        monsters_lookup: HashMap<MonsterId, MonsterProfile>,
        monsters_respawn_location: Vec<MonsterRespawnLocation>,
    ) -> Self {
        let monsters = monsters_respawn_location
            .clone()
            .into_iter()
            .filter(|respawn_info| monsters_lookup.contains_key(&respawn_info.id))
            .map(|respawn_info| {
                let monster_profile = monsters_lookup.get(&respawn_info.id).unwrap();
                println!("1");
                Monster::new(monster_profile.clone(), respawn_info.respawn_position)
            })
            .collect();

        Self {
            game_log_tx,
            monsters_lookup,
            monsters_respawn_location,
            enemies: Arc::new(Mutex::new(monsters)),
            move_timer: Timer::from_millis(500),
        }
    }

    pub fn respawn(&mut self, index: usize) {
        let monster_respawn_info = self.monsters_respawn_location.get(index);
        if let Some(respawn_info) = monster_respawn_info {
            let monster_profile = self.monsters_lookup.get(&respawn_info.id).unwrap();
            let monster = Monster::new(
                monster_profile.clone(),
                respawn_info.respawn_position.clone(),
            );
            let mut respawn_timer = Timer::from_millis(respawn_info.respawn_time);
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
                    .send(Cmds::Monster(MonsterCmds::Dead(enemy_killed.clone())))
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
