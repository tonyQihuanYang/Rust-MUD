use bevy::{
    ecs::system::Query,
    prelude::{info, Quat},
    transform::components::Transform,
};

use naia_bevy_demo_shared::protocol::Position;

pub fn sync(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = f32::from(*pos.x);
        transform.translation.y = f32::from(*pos.y) * -1.0;

        // let rotation = match *pos.direction {
        //     270f32 => Quat::from_rotation_z(0.0),                 // up
        //     90f32 => Quat::from_rotation_z(std::f32::consts::PI), // down
        //     180f32 => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2), // left
        //     _ => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2), // right
        // };

        let rotation = if *pos.direction == 180f32 {
            Quat::from_rotation_y(std::f32::consts::PI)
        } else {
            Quat::default()
        };
        transform.rotation = rotation;
    }
}
