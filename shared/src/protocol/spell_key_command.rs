use bevy_ecs::component::Component;
use naia_shared::{EntityProperty, Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct SpellKeyCommand {
    pub entity: EntityProperty,
    pub space: Property<bool>,
}

impl SpellKeyCommand {
    pub fn new(space: bool) -> Self {
        SpellKeyCommand::new_complete(space)
    }
}
