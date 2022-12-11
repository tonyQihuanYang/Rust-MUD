use bevy::{
    ecs::system::Commands,
    log::info,
    prelude::{AssetServer, Assets, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use naia_bevy_client::Client;

use naia_bevy_demo_shared::{
    protocol::{Auth, Protocol},
    Channels,
};

use crate::resources::{Global, SpellsTextures};

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
