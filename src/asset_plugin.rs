use crate::{aseprite_data::SpritesheetData, prelude::Spritesheet, sprite_animator};
use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, AsyncReadExt},
    prelude::*,
    sprite::Anchor,
};
use serde_json::from_slice;

// Struct Definitions: ---------------------------------------------------------

pub struct SpritesheetAssetPlugin {
    extensions: Vec<&'static str>,
}

struct SpritesheetAssetLoader {
    extensions: Vec<&'static str>,
}

#[derive(Component)]
struct SpriteSheetLoader {
    on_complete: Option<fn(&mut Spritesheet)>,
}

// Struct Implementations: -----------------------------------------------------

impl Plugin for SpritesheetAssetPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(SpritesheetAssetLoader {
            extensions: self.extensions.clone(),
        })
        .init_asset::<SpritesheetData>()
        .init_asset::<Spritesheet>()
        .add_event::<sprite_animator::AnimFinishEvent>()
        .add_systems(PreUpdate, handle_spritesheet_loading)
        .add_systems(PostUpdate, sprite_animator::animate_sprites);
    }
}

impl SpritesheetAssetPlugin {
    pub fn new(extensions: &[&'static str]) -> Self {
        SpritesheetAssetPlugin {
            extensions: extensions.to_owned(),
        }
    }
}

impl AssetLoader for SpritesheetAssetLoader {
    type Asset = SpritesheetData;
    type Settings = ();
    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = from_slice::<SpritesheetData>(&bytes).expect("unable to decode sprite");
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

// Systems: -------------------------------------------------------------------

fn handle_spritesheet_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spritesheet_data_assets: Res<Assets<SpritesheetData>>,
    mut spritesheet_assets: ResMut<Assets<Spritesheet>>,
    mut atlas_assets: ResMut<Assets<TextureAtlas>>,
    query: Query<(
        Entity,
        &Handle<SpritesheetData>,
        &Handle<Spritesheet>,
        &Anchor,
        &SpriteSheetLoader,
    )>,
) {
    // iterate through each loader
    for (ent, handle_spr_dat, handle_spr, anchor, loader) in &query {
        // if it's loaded
        if let Some(spr_data) = spritesheet_data_assets.get(handle_spr_dat) {
            if let Some(sheet) = spritesheet_assets.get_mut(handle_spr) {
                // create the spritesheet
                sheet.copy_from_with_image(spr_data, anchor, &asset_server);
                sheet.create_atlas_handle(&mut atlas_assets);

                // execute callback and destroy loader entity
                if let Some(callback) = loader.on_complete {
                    (callback)(sheet);
                }
                commands.entity(ent).despawn();
            }
        }
    }
}

// Utility: -------------------------------------------------------------------

/// a utility function used to load a spritesheet and optionally do some processing on it
/// if needed. The spritesheet handle is returned
///
/// # Arguments
/// * `asset_server` the asset server from bevy that is used to load the asset from the sp
///     specified path
/// * `path` the asset path that points to the spritesheet file
/// * `on_load` a closure that executes when the spritesheet data has finished loading and the
///     spritesheet is created from the data. The [`Spritesheet`] is passed in as a mutable
///     reference so that you can modify it's animations or do whatever else you need to
///
/// # Example
/// ```
/// load_spritesheet_then(
///     &mut commands,
///     &asset_server,
///     "my_spritesheet.sprite.json",
///     Anchor::Center,
///     |sheet|{
///         let attack = sheet.get_anim_handle("attack");
///         let idle = sheet.get_anim_handle("idle");
///         if let Ok(attack_anim) = sheet.get_anim_mut(attack) {
///             attack_anim.end_action = AnimEndAction::Next(idle);
///         }
///     });
/// ```
pub fn load_spritesheet_then<'a, F>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    path: impl Into<AssetPath<'a>>,
    frame_anchor: Anchor,
    on_load: fn(&mut Spritesheet),
) -> Handle<Spritesheet> {
    let spr_dat_handle = asset_server.load::<SpritesheetData>(path);
    let spr_handle = asset_server.add::<Spritesheet>(default());
    commands.spawn((
        spr_dat_handle.clone(),
        spr_handle.clone(),
        frame_anchor,
        SpriteSheetLoader {
            on_complete: Some(on_load),
        },
    ));
    spr_handle
}
