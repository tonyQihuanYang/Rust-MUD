use crate::commands::{Cmds, PlayerCmds, SystemCmds};
use crate::game::models::player::Player;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, sync::mpsc::Sender};
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

    pub fn execute_cmds(&self, cmd: PlayerCmds) {
        match cmd.clone() {
            PlayerCmds::Join(user_id) => if let Some(player) = self.get_player(&user_id) {},
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

    fn join_player(&mut self, user_id: UserId) {
        self.players.insert(
            user_id,
            Arc::new(RwLock::new(Player::new(self.game_log_tx.clone()))),
        );
    }

    fn get_player(&self, user_id: &UserId) -> Option<&Arc<RwLock<Player>>> {
        self.players.get(user_id)
    }
}
