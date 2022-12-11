use bevy::prelude::*;

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};

use naia_bevy_demo_shared::{protocol::Protocol, shared_config, Channels};

use crate::plugins::camera::CameraPlugin;
use crate::plugins::keyboard::KeyBoardInputPlugin;
use crate::systems::{events, init, player, spell, sync, tick};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 600.0;

pub fn run() {
    App::default()
        .insert_resource(Msaa { samples: 4 })
        // Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: HEIGHT * RESOLUTION,
                height: HEIGHT,
                title: "Monster Fighter".to_string(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                resizable: false,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        // .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        // Startup System
        .add_startup_system(init)
        // Realtime Gameplay Loop
        .add_system_to_stage(Stage::Connection, events::connect_event)
        .add_system_to_stage(Stage::Disconnection, events::disconnect_event)
        .add_system_to_stage(Stage::Rejection, events::reject_event)
        .add_plugin(KeyBoardInputPlugin)
        .add_plugin(CameraPlugin)
        .add_system_to_stage(Stage::ReceiveEvents, events::spawn_entity_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::insert_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::update_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::insert_enemy_event)
        // .add_system_to_stage(Stage::Frame, input)
        .add_system_to_stage(Stage::PostFrame, sync)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, player::player_animation_system)
        .add_system_to_stage(Stage::Tick, spell::spwan_spell_system)
        .add_system_to_stage(Stage::Tick, spell::spell_animation_system)
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .run();
}
