use bevy::{
    ecs::system::{Res, ResMut},
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use naia_bevy_client::{Client, Stage};

use naia_bevy_demo_shared::{
    protocol::{KeyCommand, Protocol, SpellKeyCommand},
    Channels,
};

use crate::resources::Global;

pub struct KeyBoardInputPlugin;
impl Plugin for KeyBoardInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(Stage::Frame, player_movement_input)
            .add_system_to_stage(Stage::Frame, player_acctack_input)
            .add_system_to_stage(Stage::Tick, send_key_commands);
    }
}

pub fn player_movement_input(
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

pub fn player_acctack_input(
    mut global: ResMut<Global>,
    client: Client<Protocol, Channels>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let space = keyboard_input.just_released(KeyCode::Space);
    if !space {
        return;
    }

    info!("Pressed space");
    if let Some(owned_entity) = &global.owned_entity {
        let mut key_command = SpellKeyCommand::new(space);
        key_command.entity.set(&client, &owned_entity.confirmed);
        global.queued_spell_command = Some(key_command);
    }

    //     if let Some(command) = &mut global.queued_spell_command {
    //         *command.space = true;
    //     } else if let Some(owned_entity) = &global.owned_entity {
    //         let mut key_command = SpellKeyCommand::new(space);
    //         key_command.entity.set(&client, &owned_entity.confirmed);
    //         global.queued_spell_command = Some(key_command);
    //     }
}

pub fn send_key_commands(
    mut global: ResMut<Global>,
    mut client: Client<Protocol, Channels>,
    // mut position_query: Query<&mut Position>,
) {
    if let Some(command) = global.queued_spell_command.take() {
        // if let Some(predicted_entity) = global
        //     .owned_entity
        //     .as_ref()
        //     .map(|owned_entity| owned_entity.predicted)
        // {
        if let Some(client_tick) = client.client_tick() {
            if global.spell_command_history.can_insert(&client_tick) {
                // Record command
                global
                    .spell_command_history
                    .insert(client_tick, command.clone());

                // Send command
                client.send_message(Channels::PlayerCommand, &command);
                info!("Send Cmd");
                // Apply command
                // if let Ok(mut position) = position_query.get_mut(predicted_entity) {
                //     shared_behavior::process_command(&command, &mut position);
                // }
            }
        }
        // }
    }
}
