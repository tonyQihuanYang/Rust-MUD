use crate::protocol::{Direction, KeyCommand, Position};

const SQUARE_SPEED: i16 = 3;

pub fn process_command(key_command: &KeyCommand, position: &mut Position) {
    if *key_command.w {
        *position.y = position.y.wrapping_sub(SQUARE_SPEED);
    }
    if *key_command.s {
        *position.y = position.y.wrapping_add(SQUARE_SPEED);
    }
    if *key_command.a {
        *position.x = position.x.wrapping_sub(SQUARE_SPEED);
    }
    if *key_command.d {
        *position.x = position.x.wrapping_add(SQUARE_SPEED);
    }
    // reset the rotation, then set the rotation
    *position.direction = Direction::Left;
    let key_cmd_direction = (*key_command.direction).to_owned();
    *position.direction = key_cmd_direction;
}
