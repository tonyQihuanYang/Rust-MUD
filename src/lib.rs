pub mod commands;
pub mod monsters;
pub mod player;
pub mod position;
pub mod profile;
pub mod server;
pub mod shot;
pub mod ui;

pub const WINDOW_HEIGHT: usize = 50; // Y
pub const WINDOW_WIDTH: usize = 80; // X

pub const NUM_ROWS: usize = 20; // Height
pub const NUM_COLS: usize = 40; // Width

pub const PROFILE_X: usize = 60;
pub const PROFILE_HEIGHT: usize = 10;

pub const MONSTERS_LIST_X: usize = 60;
pub const MONSTERS_LIST_Y: usize = 11;

pub const MONSTERS_X_START: usize = 41;
pub const MONSTERS_X_END: usize = WINDOW_WIDTH - 1;
pub const MONSTERS_Y_START: usize = 11;
pub const MONSTERS_Y_END: usize = WINDOW_HEIGHT - 1;

pub const LOG_X_START: usize = 0;
pub const LOG_X_END: usize = 40;
pub const LOG_Y_START: usize = 21;
pub const LOG_Y_END: usize = WINDOW_HEIGHT - 1;

#[derive(Clone)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
