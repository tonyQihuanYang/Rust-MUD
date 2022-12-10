use bevy::{
    prelude::{info, Commands, Component, EventReader, Query, Res, Transform, Vec3, With},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
};
use naia_bevy_client::events::InsertComponentEvent;
use naia_bevy_demo_shared::protocol::{ProtocolKind, Spell};

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
    query: Query<&Spell>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::Spell) = event {
            if let Ok(_) = query.get(*entity) {
                local
                    .entity(*entity)
                    .insert(SpriteSheetBundle {
                        texture_atlas: textures.spell_1.clone(),
                        transform: Transform {
                            translation: Vec3::new(10.0, 10.0, 15.0),
                            scale: Vec3::new(1.0, 1.0, 1.),
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
