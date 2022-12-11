use bevy_ecs::component::Component;

use naia_shared::{EntityProperty, Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct KeyCommand {
    pub entity: EntityProperty,
    pub w: Property<bool>,
    pub s: Property<bool>,
    pub a: Property<bool>,
    pub d: Property<bool>,
    pub direction: Property<f32>,
}

impl KeyCommand {
    pub fn new(w: bool, s: bool, a: bool, d: bool, direction: f32) -> Self {
        KeyCommand::new_complete(w, s, a, d, direction)
    }
}
