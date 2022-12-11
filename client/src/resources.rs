use std::default::Default;

use bevy::{
    ecs::{entity::Entity, prelude::Resource},
    prelude::{Handle, Image},
    sprite::TextureAtlas,
};

use naia_bevy_client::CommandHistory;

use naia_bevy_demo_shared::protocol::{KeyCommand, SpellKeyCommand};

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

impl OwnedEntity {
    pub fn new(confirmed_entity: Entity, predicted_entity: Entity) -> Self {
        OwnedEntity {
            confirmed: confirmed_entity,
            predicted: predicted_entity,
        }
    }
}

#[derive(Default, Resource)]
pub struct Global {
    pub owned_entity: Option<OwnedEntity>,
    pub queued_command: Option<KeyCommand>,
    pub command_history: CommandHistory<KeyCommand>,
    pub queued_spell_command: Option<SpellKeyCommand>,
    pub spell_command_history: CommandHistory<SpellKeyCommand>,
}

#[derive(Resource, Clone)]
pub struct PlayerTextures {
    pub head: Handle<Image>,
    pub eye: Handle<Image>,
    pub body: Handle<TextureAtlas>,
    pub length: usize,
}

#[derive(Resource, Clone)]
pub struct SpellsTextures {
    pub spell_1: Handle<TextureAtlas>,
    pub length: usize,
}
