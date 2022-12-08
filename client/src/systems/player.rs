use bevy::prelude::{info, Component, Query, Res, With};
use bevy::sprite::TextureAtlasSprite;
use bevy::time::{Time, Timer, TimerMode};
use naia_bevy_demo_shared::protocol::Player;

use crate::resources::PlayerTextures;

#[derive(Component)]
pub struct PlayerTimer(pub Timer);
impl Default for PlayerTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

pub fn player_animation_system(
    player_textures: Res<PlayerTextures>,
    time: Res<Time>,
    mut query: Query<(&mut PlayerTimer, &mut TextureAtlasSprite), With<Player>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if sprite.index < player_textures.length - 1 {
                sprite.index += 1; // move to next sprite cell
            } else {
                sprite.index = 0;
            }
        }
    }
}
