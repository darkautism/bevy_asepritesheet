use crate::{animator::animate_sprites, assets::SpritesheetAssetLoader, prelude::*};
use bevy::{asset::AssetPath, prelude::*, sprite::Anchor};

// Structs: -------------------------------------------------------------------

pub struct AsepritesheetPlugin {
    extensions: Vec<&'static str>,
}

/// Allows basic control over all [`crate::prelude::SpriteAnimator`] components
#[derive(Resource)]
pub struct SpriteAnimController {
    /// whether or not the animators will animate automatically, if turned off, individual sprite
    /// animator components can still be manually called and animated
    pub is_active: bool,
    /// the global time scale that will affect all animators
    pub global_time_scale: f32,
}

#[derive(Component)]
struct SpriteSheetLoader {
    on_complete: Option<fn(&mut Spritesheet)>,
}

// Implementations: -----------------------------------------------------------

impl Default for SpriteAnimController {
    fn default() -> Self {
        Self {
            is_active: true,
            global_time_scale: 1.0,
        }
    }
}

impl Plugin for AsepritesheetPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(SpritesheetAssetLoader {
            extensions: self.extensions.clone(),
        })
        .insert_resource(SpriteAnimController::default())
        .init_asset::<SpritesheetData>()
        .init_asset::<Spritesheet>()
        .add_event::<AnimFinishEvent>()
        .add_systems(PreUpdate, handle_spritesheet_loading)
        .add_systems(PostUpdate, animate_sprites);
    }
}

impl AsepritesheetPlugin {
    pub fn new(extensions: &[&'static str]) -> Self {
        AsepritesheetPlugin {
            extensions: extensions.to_owned(),
        }
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
/// if needed. The spritesheet handle is returned. The [`SpritesheetData`]
/// asset is loaded and then a [`Spritesheet`] is automatically generated from the data. This also
/// starts loading the [`Image`] and generates the [`TextureAtlas`]. The [`Handle<Image>`] can be
/// found in the spritesheet
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
pub fn load_spritesheet_then<'a>(
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

/// A utility function used to load a spritesheet and return the handle. The [`SpritesheetData`]
/// asset is loaded and then a [`Spritesheet`] is automatically generated from the data. This also
/// starts loading the [`Image`] and generates the [`TextureAtlas`]. The [`Handle<Image>`] can be
/// found in the spritesheet
///
/// # Arguments
/// * `asset_server` the asset server from bevy that is used to load the asset from the sp
///     specified path
/// * `path` the asset path that points to the spritesheet file
/// * `on_load` a closure that executes when the spritesheet data has finished loading and the
///     spritesheet is created from the data. The [`Spritesheet`] is passed in as a mutable
///     reference so that you can modify it's animations or do whatever else you need to
pub fn load_spritesheet<'a>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    path: impl Into<AssetPath<'a>>,
    frame_anchor: Anchor,
) -> Handle<Spritesheet> {
    let spr_dat_handle = asset_server.load::<SpritesheetData>(path);
    let spr_handle = asset_server.add::<Spritesheet>(default());
    commands.spawn((
        spr_dat_handle.clone(),
        spr_handle.clone(),
        frame_anchor,
        SpriteSheetLoader { on_complete: None },
    ));
    spr_handle
}
