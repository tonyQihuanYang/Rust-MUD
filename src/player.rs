use crate::{
    frame::{Drawable, FrameMsg},
    monsters::monsters::Monsters,
    position::{Bound, Position},
    profile::Profile,
    shot::Shot,
    Directions, NUM_COLS, NUM_ROWS,
};
use std::time::Duration;

pub struct Player {
    profile: Profile,
    position: Position,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
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
    }
    pub fn move_down(&mut self) {
        self.position.move_down();
    }

    pub fn move_left(&mut self) {
        self.position.move_left();
    }

    pub fn move_right(&mut self) {
        self.position.move_right();
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

impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
        frame[self.position.x][self.position.y] = FrameMsg::Str("A");
        self.profile.draw(frame);
    }
}
