use bevy_ecs::component::Component;

use naia_shared::{EntityProperty, Property, Replicate};

use super::Direction;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct KeyCommand {
    pub entity: EntityProperty,
    pub w: Property<bool>,
    pub s: Property<bool>,
    pub a: Property<bool>,
    pub d: Property<bool>,
    pub direction: Property<Direction>,
}

impl KeyCommand {
    pub fn new(w: bool, s: bool, a: bool, d: bool, direction: Direction) -> Self {
        KeyCommand::new_complete(w, s, a, d, direction)
    }
}
