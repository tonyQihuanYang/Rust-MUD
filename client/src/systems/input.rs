use bevy::{
    ecs::system::{Res, ResMut},
    input::{keyboard::KeyCode, Input},
    prelude::info,
};

use naia_bevy_client::Client;

use naia_bevy_demo_shared::{
    protocol::{KeyCommand, Protocol},
    Channels,
};

use crate::resources::Global;

pub fn input(
    mut global: ResMut<Global>,
    client: Client<Protocol, Channels>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let w = keyboard_input.pressed(KeyCode::W);
    let s = keyboard_input.pressed(KeyCode::S);
    let a = keyboard_input.pressed(KeyCode::A);
    let d = keyboard_input.pressed(KeyCode::D);
    let direction = if a {
        180f32
    } else if s {
        45f32
    } else if w {
        270f32
    } else {
        0f32
    };

    if !w && !s && !a && !d {
        return;
    }

    if let Some(command) = &mut global.queued_command {
        if w {
            *command.w = true;
            *command.direction = direction;
        }
        if s {
            *command.s = true;
            *command.direction = direction;
        }
        if a {
            *command.a = true;
            *command.direction = direction;
        }
        if d {
            *command.d = true;
            *command.direction = direction;
        }
    } else if let Some(owned_entity) = &global.owned_entity {
        let mut key_command = KeyCommand::new(w, s, a, d, direction);
        key_command.entity.set(&client, &owned_entity.confirmed);
        global.queued_command = Some(key_command);
    }
}
