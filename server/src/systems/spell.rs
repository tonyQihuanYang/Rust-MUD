use bevy::sprite::collide_aabb::collide;
use bevy_app::{App, Plugin};
use bevy_ecs::{
    prelude::Entity,
    query::With,
    system::{Query, ResMut},
};
use bevy_log::info;
use bevy_math::{Vec2, Vec3};
// use bevy_sprite::collide_aabb::collide;
use naia_bevy_server::{Server, Stage};

use naia_bevy_demo_shared::{
    protocol::{Enemy, Position, Protocol, Spell},
    Channels,
};

use crate::resources::Global;

pub struct SpellPlugin;
impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        // .add_system_to_stage(Stage::Tick, spell::spwan_spell_system)
        // .add_system_to_stage(Stage::Tick, spell::update_spell_system)
        app.add_system_to_stage(Stage::Tick, spwan_spell_system)
            .add_system_to_stage(Stage::Tick, detect_spell_collision);
    }
}

pub fn spwan_spell_system(
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,

    mut position_query: Query<&mut Position>,
) {
    // Process all received commands
    let main_room_key = global.main_room_key.clone();
    for (entity, last_command) in global.player_last_spell_command.drain() {
        if let Ok(mut position) = position_query.get_mut(entity) {
            if *last_command.space {
                info!("pressed space pos {} - {}", *position.x, *position.y);
                server
                    // Spawn new Square Entity
                    .spawn()
                    // Add Entity to main Room
                    .enter_room(&main_room_key)
                    // Insert Position component
                    .insert(position.clone())
                    .insert(Spell::new(1, 60, 60, 0, 0));
            }
        }
    }
    server.send_all_updates();
}

pub fn detect_spell_collision(
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
    mut spell_query: Query<(&Position, Entity), With<Spell>>,
    mut enemy_query: Query<(&Position, Entity), With<Enemy>>,
) {
    let size = 32.0;

    for (spell_pos, spell_entity) in spell_query.iter_mut() {
        for (enemy_pos, enemy_entity) in enemy_query.iter_mut() {
            if collide(
                Vec3::new(*spell_pos.x as f32, *spell_pos.y as f32, 0.0),
                Vec2::splat(size),
                Vec3::new(*enemy_pos.x as f32, *enemy_pos.y as f32, 0.0),
                Vec2::splat(size),
            )
            .is_some()
            {
                info!("Hitted");

                // server.user_scope(..).exclude(..);
                server.entity_mut(&enemy_entity).despawn();
                server.entity_mut(&spell_entity).despawn();
                global.enemy_count += 1;
            }
        }
    }
}

pub fn update_spell_system(
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
    mut query: Query<(Entity, &mut Spell)>,
) {
    // for (entity, mut spell) in query.iter_mut() {
    //     // info!("update spell server");
    //     global.spell_tick += 1;
    //     if *spell.cur_tick < 200 {
    //         *spell.cur_tick += 1;
    //     } else {
    //         info!("despawn spell server");
    //         global.spell_tick = 0;
    //         server.entity_mut(&entity).despawn();
    //     }
    // }
}

// pub fn despawn_spell_system(
//     mut global: ResMut<Global>,
//     mut server: Server<Protocol, Channels>,
//     mut query: Query<Entity, With<Spell>>,
// ) {
//     global.spell_tick = 0;
//     for entity in query.iter_mut() {
//         server.entity(entity).despawn();
//     }
// }
