use bevy_ecs::{
    prelude::Entity,
    query::With,
    system::{Query, ResMut},
};
use bevy_log::info;
use naia_bevy_server::{shared::Random, Server};

use naia_bevy_demo_shared::{
    protocol::{Color, ColorValue, Enemy, Position, Protocol, Spell},
    Channels,
};

use crate::resources::Global;

pub fn spwan_spell_system(mut global: ResMut<Global>, mut server: Server<Protocol, Channels>) {
    if global.spell_tick == 0 {
        info!("spawn spell server");
        let server = &mut server;
        let position = {
            let x = 16 * ((Random::gen_range_u32(0, 40) as i16) - 20);
            let y = 16 * ((Random::gen_range_u32(0, 30) as i16) - 15);
            Position::new(x, y)
        };
        server
            // Spawn new Square Entity
            .spawn()
            // Add Entity to main Room
            .enter_room(&global.main_room_key)
            // Insert Position component
            .insert(position)
            // Insert Color component
            .insert(Color::new(ColorValue::Red))
            .insert(Spell::new(1, 60, 60, 0, 0));
        global.spell_tick += 1;
    }
}

pub fn update_spell_system(
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
    mut query: Query<(Entity, &mut Spell)>,
) {
    for (entity, mut spell) in query.iter_mut() {
        // info!("update spell server");
        global.spell_tick += 1;
        if *spell.cur_tick < 200 {
            *spell.cur_tick += 1;
        } else {
            info!("despawn spell server");
            global.spell_tick = 0;
            server.entity_mut(&entity).despawn();
        }
    }
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
