use crate::commands::{Cmds, PlayerCmds, SystemCmds};
use crate::game::models::player::{self, Player};
use super::monsters_controller::{MonstersControl};
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, sync::mpsc::Sender, time::Duration};
pub type UserId = i32;

pub struct PlayersControl {
    pub players: HashMap<UserId, Arc<RwLock<Player>>>,
    game_log_tx: Sender<Cmds>,
}

impl PlayersControl {
    pub fn new(game_log_tx: Sender<Cmds>) -> Self {
        Self {
            players: HashMap::new(),
            game_log_tx,
        }
    }

    pub fn update(&self, delta: Duration) {
        for (_, player_lock) in &self.players {
            let mut player = player_lock.write().unwrap();
            (*player).update(delta);
        }
    }

    pub fn detect_hits(&self, monsters_controller: &mut MonstersControl) {
        for (_, player_lock) in &self.players {
            let mut player = player_lock.write().unwrap();
            (*player).detect_hits(monsters_controller);
        }
    }

    pub fn execute_cmds(&mut self, cmd: PlayerCmds) {
        match cmd.clone() {
            PlayerCmds::Join(user_id) => {
                self.players.insert(
                    user_id,
                    Arc::new(RwLock::new(Player::new(self.game_log_tx.clone()))),
                );
            }
            PlayerCmds::MoveUp(user_id) => {
                if let Some(player_lock) = self.get_player(&user_id) {
                    let mut player = player_lock.write().unwrap();
                    (*player).move_up();
                }
            }
            PlayerCmds::MoveDown(user_id) => {
                if let Some(player_lock) = self.get_player(&user_id) {
                    let mut player = player_lock.write().unwrap();
                    (*player).move_down();
                }
            }
            PlayerCmds::MoveLeft(user_id) => {
                if let Some(player_lock) = self.get_player(&user_id) {
                    let mut player = player_lock.write().unwrap();
                    (*player).move_left();
                }
            }
            PlayerCmds::MoveRight(user_id) => {
                if let Some(player_lock) = self.get_player(&user_id) {
                    let mut player = player_lock.write().unwrap();
                    (*player).move_right();
                }
            }
            PlayerCmds::InputAttack(user_id) => {
                if let Some(player_lock) = self.get_player(&user_id) {
                    let mut player = player_lock.write().unwrap();
                    (*player).shoot();
                }
            }
            _ => (),
        }
    }

    fn get_player(&self, user_id: &UserId) -> Option<&Arc<RwLock<Player>>> {
        self.players.get(user_id)
    }
}
