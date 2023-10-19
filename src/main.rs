use bevy::prelude::*;
use bevy::reflect::{TypeUuid, TypePath};
use bevy_common_assets::json::JsonAssetPlugin;

fn main() {

    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins,
            JsonAssetPlugin::<SpriteSheetData>::new(&[".json"])
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, step)
    ;

    app.run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>
) {
    let witch_image = assets.load::<Image, _>("witch.png");
}

fn step(time: Res<Time>){
    
}

// -------------------------------------------------------------------------------------------------

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "73461c8f-e760-4fb8-8492-37d5387fca7b"]
pub struct SpriteSheetData {
    pub frames: Vec<FrameData>,
    pub meta: MetaData
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "d49c70a1-177b-44ff-b427-d3929c928667"]
pub struct FrameData {
    pub frame: RectData,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: RectData,
    #[serde(rename = "sourceSize")]
    pub source_size: SizeData,
    pub duration: u32
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "f88c0866-6ed2-4b45-a6b6-7dcbe8c53f21"]
pub struct FrameTagData {
    pub name: String,
    pub from: u16,
    pub to: u16,
    pub direction: String
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "ea8ca8be-43c4-4b89-98e0-54afef524261" ]
pub struct MetaData {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: SizeData,
    pub scale: String
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "a168bfd8-e587-4e52-89b3-58b50f6c1823"]
pub struct SizeData {
    pub w: u16,
    pub h: u16
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "4643df56-80d8-4f49-91df-67fc95307b30"]
pub struct RectData {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16
}