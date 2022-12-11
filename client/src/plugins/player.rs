use crate::resources::Global;
use crate::resources::PlayerTextures;
use bevy::prelude::*;
use naia_bevy_client::events::InsertComponentEvent;
use naia_bevy_client::Stage;
use naia_bevy_demo_shared::protocol::{Player, ProtocolKind};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_player_textures)
            .add_system_to_stage(Stage::Tick, player_animation_system)
            .add_system_to_stage(Stage::ReceiveEvents, insert_players_event);
    }
}

const PLAYER_HEAD_SPRITE: &str = "eyes5/idle_0.png";
const PLAYER_EYE_SPRITE: &str = "head1/idle_0.png";
const PLAYER_BODY_SHEET: &str = "body.png";
pub fn init_player_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(PLAYER_BODY_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(2048.0, 2048.0),
        3,
        2,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let body = texture_atlases.add(texture_atlas.clone());
    let player_textures = PlayerTextures {
        head: asset_server.load(PLAYER_HEAD_SPRITE),
        eye: asset_server.load(PLAYER_EYE_SPRITE),
        body,
        length: texture_atlas.len(),
    };
    commands.insert_resource(player_textures);
}

#[derive(Component)]
pub struct PlayerTimer(pub Timer);
impl Default for PlayerTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

pub fn insert_players_event(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut local: Commands,
    player_textures: Res<PlayerTextures>,
    query: Query<&Player>,
    global: ResMut<Global>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, ProtocolKind::Color) = event {
            if let Ok(_) = query.get(*entity) {
                // if the enity is owned, skip
                if let Some(owned_entity) = &global.owned_entity {
                    if owned_entity.confirmed == *entity {
                        return;
                    }
                } else {
                    // if no owned... return
                    return;
                }

                //render the other player
                local
                    .entity(*entity)
                    .insert(SpriteSheetBundle {
                        texture_atlas: player_textures.body.clone(),
                        transform: Transform {
                            translation: Vec3::new(10.0, 10.0, 15.0),
                            scale: Vec3::new(0.03125, 0.03125, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(PlayerTimer::default());

                // ORG
                // local.entity(*entity).insert(SpriteBundle {
                //     sprite: Sprite {
                //         custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                //         color,
                //         ..Default::default()
                //     },
                //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
                //     ..Default::default()
                // });
            }
        }
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
