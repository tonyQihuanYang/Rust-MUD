use crate::{
    commands::{Cmds, PlayerCmds, SendCmds},
    monsters::monsters::Monsters,
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
    position: Position,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new(game_log_tx: Sender<Cmds>) -> Self {
        Self {
            game_log_tx,
            profile: Profile::new(),
            position: Position::new(
                NUM_COLS / 2,
                NUM_ROWS - 1,
                Some(Bound::new(0, NUM_COLS, 0, NUM_ROWS)),
            ),
            shots: Vec::new(),
        }
    }
    pub fn move_up(&mut self) {
        self.position.move_up();
        self.moved();
    }
    pub fn move_down(&mut self) {
        self.position.move_down();
        self.moved();
    }

    pub fn move_left(&mut self) {
        self.position.move_left();
        self.moved();
    }

    pub fn move_right(&mut self) {
        self.position.move_right();
        self.moved();
    }

    pub fn moved(&self) {
        self.game_log_tx
            .send(Cmds::Player(PlayerCmds::Move(
                self.profile.id.clone(),
                self.position.clone(),
            )))
            .unwrap();
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 5 {
            self.shots.push(Shot::new(
                self.position.x,
                self.position.y - 1,
                Directions::Up,
            ));
            self.shots.push(Shot::new(
                self.position.x + 1,
                self.position.y,
                Directions::Right,
            ));
            self.shots.push(Shot::new(
                self.position.x - 1,
                self.position.y,
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

    pub fn detect_hits(&mut self, monsters: &mut Monsters) -> bool {
        let mut hit_something = false;

        for shot in self.shots.iter_mut() {
            if !shot.exploading {
                match monsters.kill_monster_at(shot.position.x, shot.position.y) {
                    Some(monster_killed) => {
                        self.profile.gain_exp(monster_killed.exp);
                        hit_something = true;
                        shot.explode();
                    }
                    None => {}
                }
            }
        }

        hit_something
    }
}

impl SendCmds for Player {
    fn send(&self) {
        // Refactor it
        // for shot in self.shots.iter() {
        //     shot.draw(frame);
        // }
        // self.game_log_tx
        //     .send(Cmds::Player(PlayerCmds::Move(self.position.clone())))
        //     .unwrap();
    }
}
