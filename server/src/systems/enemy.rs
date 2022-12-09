use bevy_ecs::{
    query::With,
    system::{Query, ResMut},
};
use naia_bevy_server::{shared::Random, Server};

use naia_bevy_demo_shared::{
    protocol::{Color, ColorValue, Enemy, Position, Protocol},
    Channels,
};

use crate::resources::Global;

pub fn enemy(mut global: ResMut<Global>, mut server: Server<Protocol, Channels>) {
    if global.enemy_count < 3 {
        let server = &mut server;
        spawn_enemys(&global, server);
        global.enemy_count += 1;
    }
}

pub fn spawn_enemys(global: &ResMut<Global>, server: &mut Server<Protocol, Channels>) {
    // Position component
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
        .insert(Enemy);
}

pub fn enermy_movement(mut query: Query<&mut Position, With<Enemy>>) {
    for mut position in query.iter_mut() {
        match Random::gen_range_u32(1, 5) {
            1 => *position.x = position.x.wrapping_sub(3 as i16),
            2 => *position.x = position.x.wrapping_add(3 as i16),
            3 => *position.y = position.y.wrapping_sub(3 as i16),
            4 => *position.y = position.y.wrapping_add(3 as i16),
            _ => (),
        };
    }
}
