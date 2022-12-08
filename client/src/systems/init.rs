use bevy::{
    ecs::system::Commands,
    log::info,
    prelude::{AssetServer, Assets, Camera2dBundle, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use naia_bevy_client::Client;

use naia_bevy_demo_shared::{
    protocol::{Auth, Protocol},
    Channels,
};

use crate::resources::{Global, PlayerTextures, SpellsTextures};

const PLAYER_HEAD_SPRITE: &str = "eyes5/idle_0.png";
const PLAYER_EYE_SPRITE: &str = "head1/idle_0.png";
const PLAYER_BODY_SHEET: &str = "body.png";
const SPELL_SHEET: &str = "spell.png";

pub fn init(
    mut commands: Commands,
    mut client: Client<Protocol, Channels>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Naia Bevy Client Demo started");

    client.auth(Auth::new("charlie", "12345"));
    client.connect("http://127.0.0.1:14191");

    // Setup Colors
    commands.init_resource::<Global>();

    let texture_handle = asset_server.load(PLAYER_BODY_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(2048.0, 2048.0),
        3,
        2,
        // None,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    // let texture_atlas =
    //     TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);

    let body = texture_atlases.add(texture_atlas.clone());

    let player_textures = PlayerTextures {
        head: asset_server.load(PLAYER_HEAD_SPRITE),
        eye: asset_server.load(PLAYER_EYE_SPRITE),
        body,
        length: texture_atlas.len(),
    };
    commands.insert_resource(player_textures);

    // Insert Spell
    let texture_handle = asset_server.load(SPELL_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 8, 4, None, None);
    let spell_1 = texture_atlases.add(texture_atlas.clone());
    let spell_textures = SpellsTextures {
        spell_1,
        length: texture_atlas.len(),
    };

    commands.insert_resource(spell_textures);
}
