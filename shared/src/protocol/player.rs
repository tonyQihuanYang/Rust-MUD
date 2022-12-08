use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Player {
    pub name: Property<String>,
    pub hp: Property<i32>,
}

impl Player {
    pub fn new(name: &str, hp: i32) -> Self {
        Player::new_complete(name.to_string(), hp)
    }
}
