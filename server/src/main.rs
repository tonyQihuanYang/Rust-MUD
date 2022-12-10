use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_log::{info, LogPlugin};

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};

use naia_bevy_demo_shared::{protocol::Protocol, shared_config, Channels};

mod resources;
mod systems;

use crate::systems::{enemy, enermy_movement, events, init, spell, tick};

// use crate::systems::{enemy::enemy, events, init::init, tick::tick};

fn main() {
    info!("Naia Bevy Server Demo starting up");

    // Build App
    App::default()
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system(enemy)
        .add_plugin(spell::SpellPlugin)
        // .add_system_to_stage(Stage::Tick, spell::spwan_spell_system)
        // .add_system_to_stage(Stage::Tick, spell::update_spell_system)
        // .add_system_to_stage(Stage::Tick, spell::detect_spell_collision)
        .add_system_to_stage(Stage::Tick, enermy_movement)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .run();
}
