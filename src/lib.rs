pub mod frame;
pub mod monsters;
pub mod player;
pub mod profile;
pub mod render;
pub mod shot;

pub const WINDOW_HEIGHT: usize = 30;
pub const WINDOW_WIDTH: usize = 80;

pub const NUM_ROWS: usize = 20; // Height
pub const NUM_COLS: usize = 40; // Width

pub const PROFILE_X: usize = 60;
pub const PROFILE_HEIGHT: usize = 10;

pub const MONSTERS_LIST_X: usize = 60;
pub const MONSTERS_LIST_Y: usize = 11;

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
