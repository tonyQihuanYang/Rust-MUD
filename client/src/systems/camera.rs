use bevy::prelude::*;
use naia_bevy_demo_shared::protocol::Player;

use crate::resources::Global;

const SQUARE_SIZE: f32 = 32.0;
pub fn setup_camera(mut commands: Commands) {
    info!("setup_camera");
    commands.spawn_bundle(Camera2dBundle::default());
    // commands.spawn_bundle(Camera2dBundle {
    //     projection: OrthographicProjection {
    //         scaling_mode: ScalingMode::None,
    //         top: 1.0,
    //         bottom: -1.0,
    //         right: 1.0 * RESOLUTION,
    //         left: -1.0 * RESOLUTION,
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });
}

pub fn camera_follow(
    mut global: ResMut<Global>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut cam_transform = camera_query.single_mut();
    if let Some(predicted_entity) = global
        .owned_entity
        .as_ref()
        .map(|owned_entity| owned_entity.predicted)
    {
        if let Ok(player_transform) = player_query.get(predicted_entity) {
            cam_transform.translation.x = player_transform.translation.x;
            cam_transform.translation.y = player_transform.translation.y;
        }
    }
}
