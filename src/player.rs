use std::time::Duration;

use crate::{
    frame::Drawable, monsters::Monsters, profile::Profile, shot::Shot, Directions, NUM_COLS,
    NUM_ROWS,
};

pub struct Player {
    profile: Profile,
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            profile: Profile::new(),
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }
    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 5 {
            self.shots
                .push(Shot::new(self.x, self.y - 1, Directions::Up));
            self.shots
                .push(Shot::new(self.x + 1, self.y, Directions::Right));
            self.shots
                .push(Shot::new(self.x - 1, self.y, Directions::Left));
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
                match monsters.kill_monster_at(shot.x, shot.y) {
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
        frame[self.x][self.y] = "A".to_string();
        self.profile.draw(frame);
    }
}

