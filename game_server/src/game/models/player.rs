use crate::{
    commands::{Cmds, PlayerCmds},
    game::controllers::monsters_controller::MonstersControl,
    position::{Bound, Position},
    profile::Profile,
    shot::Shot,
    Directions, NUM_COLS, NUM_ROWS,
};
use std::sync::mpsc::Sender;
use std::time::Duration;

pub struct Player {
    profile: Profile,
    game_log_tx: Sender<Cmds>,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new(game_log_tx: Sender<Cmds>) -> Self {
        Self {
            game_log_tx,
            profile: Profile::new(Position::new(
                NUM_COLS / 2,
                NUM_ROWS - 1,
                Some(Bound::new(0, NUM_COLS, 0, NUM_ROWS)),
            )),
            shots: Vec::new(),
        }
    }

    pub fn move_up(&mut self) {
        self.profile.position.move_up();
        self.moved();
    }
    pub fn move_down(&mut self) {
        self.profile.position.move_down();
        self.moved();
    }

    pub fn move_left(&mut self) {
        self.profile.position.move_left();
        self.moved();
    }

    pub fn move_right(&mut self) {
        self.profile.position.move_right();
        self.moved();
    }

    pub fn moved(&self) {
        self.game_log_tx
            .send(Cmds::Player(PlayerCmds::Move(
                self.profile.id.clone(),
                self.profile.position.clone(),
            )))
            .unwrap();
    }

    pub fn shoot(&mut self) -> bool {
        self.game_log_tx
            .send(Cmds::Player(PlayerCmds::Attack(self.profile.clone())))
            .unwrap();

        if self.shots.len() < 5 {
            self.shots.push(Shot::new(
                self.profile.position.x,
                self.profile.position.y - 1,
                Directions::Up,
            ));
            self.shots.push(Shot::new(
                self.profile.position.x + 1,
                self.profile.position.y,
                Directions::Right,
            ));
            self.shots.push(Shot::new(
                self.profile.position.x - 1,
                self.profile.position.y,
                Directions::Left,
            ));
            return true;
        }
        {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, monsters_controller: &mut MonstersControl) {
        for shot in self.shots.iter_mut() {
            if !shot.exploading {
                match monsters_controller.kill_monster_at(shot.position.x, shot.position.y) {
                    Some(monster_killed) => {
                        self.profile.gain_exp(monster_killed.profile.exp);
                        shot.explode();
                    }
                    None => {}
                }
            }
        }
    }
}
