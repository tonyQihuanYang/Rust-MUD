use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct CurrentUser {}

impl CurrentUser {
    pub fn new() -> Self {
        CurrentUser::new_complete()
    }
}
