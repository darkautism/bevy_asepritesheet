mod aseprite_data;
mod sprite;

use bevy::{
    prelude::*,
    sprite::Anchor, render::color::SrgbColorSpace, core_pipeline::clear_color::ClearColorConfig
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
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            JsonAssetPlugin::<aseprite_data::SpriteSheetData>::new(&["sprite.json"])
        ))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            load.run_if(in_state(AppState::Loading)),
            step
        ))
    ;
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(SpriteHandleResource(asset_server.load("witch.sprite.json"), None));
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.3, 0.3, 0.3)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn load(
    mut commands: Commands,
    mut witch_data_handle: ResMut<SpriteHandleResource>,
    mut atlas_assets: ResMut<Assets<TextureAtlas>>,
    sprite_assets: Res<Assets<aseprite_data::SpriteSheetData>>,
    image_assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>
) {

    if witch_data_handle.1.is_none() {
        if let Some(witch_data) = sprite_assets.get(&witch_data_handle.0) {
            println!("Sprite Data Loaded!");
            witch_data_handle.1 = Some(sprite::Sheet::from_data(
                &witch_data,
                asset_server.load::<Image, _>(&witch_data.meta.image),
                Anchor::Center
            ));
        }
    }

    else {
        let witch_spritesheet = witch_data_handle.1.as_mut().unwrap();
        if image_assets.contains(witch_spritesheet.img_handle()) {
            println!("Image Loaded!");
            state.set(AppState::Playing);
            commands.spawn(
                SpriteSheetBundle{
                    texture_atlas: witch_spritesheet.create_atlas_handle(&mut atlas_assets).clone(),
                    transform: Transform::from_scale(Vec3::new(2.0, 2.0, 1.0)),
                    ..Default::default()
                }
            );
        }
    }
}

fn step(mut query: Query<&mut TextureAtlasSprite>){
    for mut sprite in &mut query {
        sprite.index = sprite.index + 1;
        if sprite.index >= 50 { sprite.index = 0; }
    }
}