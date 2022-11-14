pub mod frame;
pub mod main;
pub mod render;
pub mod sections;
pub mod skills;

pub const WINDOW_HEIGHT: usize = 50; // Y
pub const WINDOW_WIDTH: usize = 80; // X

pub const MAP_X_START: usize = 0;
pub const MAP_X_END: usize = 40;
pub const MAP_Y_START: usize = 0;
pub const MAP_Y_END: usize = 20;

pub const MONSTERS_X_START: usize = 41;
pub const MONSTERS_X_END: usize = WINDOW_WIDTH - 1;
pub const MONSTERS_Y_START: usize = 11;
pub const MONSTERS_Y_END: usize = WINDOW_HEIGHT - 1;

pub const LOG_X_START: usize = 0;
pub const LOG_X_END: usize = 40;
pub const LOG_Y_START: usize = 21;
pub const LOG_Y_END: usize = WINDOW_HEIGHT - 1;
