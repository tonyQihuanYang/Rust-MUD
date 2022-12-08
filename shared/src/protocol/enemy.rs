use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Enemy;

impl Enemy {
    pub fn new() -> Self {
        Enemy::new_complete()
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
