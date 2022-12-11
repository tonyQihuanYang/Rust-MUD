use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Enemy {
    pub movement_speed: Property<u8>,
    pub movement_tick: Property<u8>,
}

impl Enemy {
    pub fn new(movement_speed: u8) -> Self {
        Enemy::new_complete(movement_speed, 0)
    }
}

// #[derive(Component, Replicate)]
// #[protocol_path = "crate::protocol::Protocol"]
// pub struct Enemy {
//     pub name: Property<String>,
// }

// impl Enemy {
//     pub fn new(name: &str) -> Self {
//         Enemy::new_complete(name.to_string())
//     }
// }
