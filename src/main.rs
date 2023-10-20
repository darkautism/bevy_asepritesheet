mod aseprite_data;
mod sprite;

use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

fn main() {

    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins,
            JsonAssetPlugin::<aseprite_data::SpriteSheetData>::new(&["sprite.json"])
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, step)
    ;

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SpriteHandleResource(asset_server.load("witch.sprite.json")));
    commands.insert_resource(ImageHandleResource(asset_server.load("witch.png")));
}

fn step(
    witch_data_handle: Res<SpriteHandleResource>,
    sprite_assets: Res<Assets<aseprite_data::SpriteSheetData>>,
) {
    if let Some(witch_data) = sprite_assets.get(&witch_data_handle.0){
        // TODO
    }
}

#[derive(Resource)]
struct SpriteHandleResource(Handle<aseprite_data::SpriteSheetData>);

#[derive(Resource)]
struct ImageHandleResource(Handle<Image>);