pub mod frame;
pub mod invaders;
pub mod player;
pub mod profile;
pub mod render;
pub mod shot;

pub const WINDOW_WIDTH: usize = 40;
pub const WINDOW_HEIGHT: usize = 60;

pub const NUM_ROWS: usize = 20; // Height
pub const NUM_COLS: usize = 40; // Width

pub const PROFILE_X: usize = 20;
pub const PROFILE_HEIGHT: usize = WINDOW_HEIGHT;

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
