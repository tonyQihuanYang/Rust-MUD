use crate::ui::frame::{Drawable, Frame, FrameMsg};
use crate::ui::{MAP_X_END, MAP_X_START, MAP_Y_END, MAP_Y_START};
use crate::{position::Position, Directions};
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

#[derive(Clone)]
pub struct Skill {
    pub position: Position,
    pub range: u8,
    pub attack_speed: u32,
    pub direction: Option<Directions>,
    curr_frame: Arc<RwLock<u8>>,
}

impl Skill {
    pub fn new(
        position: Position,
        range: u8,
        attack_speed: u32,
        direction: Option<Directions>,
    ) -> Self {
        Self {
            position,
            curr_frame: Arc::new(RwLock::new(1)),
            range,
            attack_speed,
            direction,
        }
    }

    pub fn is_active(&self) -> bool {
        let curr_frame_lock = Arc::clone(&self.curr_frame);
        let curr_frame = curr_frame_lock.read().unwrap();
        *curr_frame <= self.range
    }

    pub fn render(&self) {
        let range = self.range.clone();
        let curr_frame_lock = Arc::clone(&self.curr_frame);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(50));
            {
                let mut curr_frame = curr_frame_lock.write().unwrap();
                if *curr_frame <= range {
                    *curr_frame += 1;
                } else {
                    break;
                }
            }
        });
    }
}

impl Drawable for Skill {
    fn draw(&self, frame: &mut Frame) {
        if self.is_active() {
            let pos = self.position.clone();
            let curr_frame_lock = Arc::clone(&self.curr_frame);
            let curr_frame = curr_frame_lock.read().unwrap();
            let curr_frame_uszie = *curr_frame as usize;

            let new_t = pos.y + (1 * curr_frame_uszie);
            let new_d = if pos.y > (1 * curr_frame_uszie) {
                pos.y - (1 * curr_frame_uszie)
            } else {
                0
            };
            let new_l = if pos.x > (1 * curr_frame_uszie) {
                pos.x - (1 * curr_frame_uszie)
            } else {
                0
            };
            let new_r = pos.x + (1 * curr_frame_uszie);

            match self.position.bound.clone() {
                Some(bound) => {
                    if bound.is_y_in_bound(&new_t) {
                        frame[pos.x][new_t] = FrameMsg::Str("o");
                    }
                    if bound.is_y_in_bound(&new_d) {
                        frame[pos.x][new_d] = FrameMsg::Str("o");
                    }
                    if bound.is_x_in_bound(&new_l) {
                        frame[new_l][pos.y] = FrameMsg::Str("o");
                    }
                    if bound.is_x_in_bound(&new_r) {
                        frame[new_r][pos.y] = FrameMsg::Str("o");
                    }
                }
                _ => {
                    frame[pos.x][new_t] = FrameMsg::Str("o");
                    frame[pos.x][new_d] = FrameMsg::Str("o");
                    frame[new_l][pos.y] = FrameMsg::Str("o");
                    frame[new_r][pos.y] = FrameMsg::Str("o");
                }
            };
        }
    }
}
