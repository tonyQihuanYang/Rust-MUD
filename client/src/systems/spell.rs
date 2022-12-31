use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
};
use naia_bevy_client::events::InsertComponentEvent;
use naia_bevy_demo_shared::protocol::{Position, ProtocolKind, Spell};

use crate::resources::SpellsTextures;

#[derive(Component)]
pub struct SpellTimer(pub Timer);
impl Default for SpellTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.015, TimerMode::Repeating))
    }
}

pub fn spwan_spell_system(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut local: Commands,
    textures: Res<SpellsTextures>,
    query: Query<&Position, With<Spell>>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::Spell) = event {
            if let Ok(pos) = query.get(*entity) {
                local
                    .entity(*entity)
                    .insert(SpriteSheetBundle {
                        texture_atlas: textures.spell_1.clone(),
                        transform: Transform {
                            translation: Vec3::new(*pos.x as f32, *pos.y as f32, 90.0),
                            scale: Vec3::new(1.0, 1.0, 1.),
                            // rotation: Quat::from_rotation_z(45f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(SpellTimer::default());
            }
        }
    }
}

pub fn spell_animation_system(
    textures: Res<SpellsTextures>,
    time: Res<Time>,
    mut query: Query<(&mut SpellTimer, &mut TextureAtlasSprite), With<Spell>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if sprite.index < textures.length - 1 {
                sprite.index += 1; // move to next sprite cell
            } else {
                sprite.index = 0;
            }
        }
    }
}
