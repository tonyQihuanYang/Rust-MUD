use bevy_ecs::prelude::Component;

use naia_shared::{derive_serde, serde, Property, Replicate};

#[derive_serde]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Position {
    pub x: Property<i16>,
    pub y: Property<i16>,
    pub direction: Property<Direction>,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Self {
        Position::new_complete(x, y, Direction::Left)
    }
}

trait DirectionAngle {
    fn get_angles(&self) -> f32;
}

impl DirectionAngle for Position {
    fn get_angles(&self) -> f32 {
        let direction = (*self.direction).to_owned();
        match direction {
            Direction::Left => 0f32,
            Direction::Up => 90f32,
            Direction::Right => 180f32,
            Direction::Down => 270f32,
        }
    }
}

/*
*     <-  0f32
*     ->  180f32
*     ^   90f32
*     Down 270f32
   Left = 0
   Up = 90,
   Right = 180, // 123
   Down = 270,  // 124
*/
