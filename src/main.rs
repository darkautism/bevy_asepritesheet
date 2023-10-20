mod aseprite_data;
mod sprite;

use bevy::{
    prelude::*,
    sprite::Anchor
};
use bevy_common_assets::json::JsonAssetPlugin;


#[derive(Resource)]
struct SpriteHandleResource(Handle<aseprite_data::SpriteSheetData>, Option<sprite::Sheet>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Playing
}

// -------------------------------------------------------------------------------------------------

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            DefaultPlugins,
            JsonAssetPlugin::<aseprite_data::SpriteSheetData>::new(&["sprite.json"])
        ))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, load.run_if(in_state(AppState::Loading)))
    ;
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(SpriteHandleResource(asset_server.load("witch.sprite.json"), None));
}

fn load(
    mut witch_data_handle: ResMut<SpriteHandleResource>,
    sprite_assets: Res<Assets<aseprite_data::SpriteSheetData>>,
    image_assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>
) {
    if witch_data_handle.1.is_none() {
        if let Some(witch_data) = sprite_assets.get(&witch_data_handle.0) {
            witch_data_handle.1 = Some(sprite::Sheet::from_data(
                &witch_data,
                asset_server.load::<Image, _>(&witch_data.meta.image),
                Anchor::Center
            ));
            println!("Sprite Data Loaded!")
        }
    }

    else if let Some(img) = image_assets.get(witch_data_handle.1.as_ref().unwrap().img_handle()) {
        state.set(AppState::Playing);
        println!("Image Loaded!");
    }
}