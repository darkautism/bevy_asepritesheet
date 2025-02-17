use crate::{
    animator::{animate_sprites, AnimationSet},
    assets::SpritesheetAssetLoader,
    prelude::*,
    sprite::{add_needed_atlas_handles, add_needed_img_handles},
};
use bevy::{
    asset::AssetPath, ecs::schedule::ScheduleLabel, prelude::*, sprite::Anchor,
    utils::intern::Interned,
};

// Structs: -------------------------------------------------------------------

pub struct AsepritesheetPlugin {
    schedule: Option<Interned<dyn ScheduleLabel>>,
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

/// Event which gets fired when a spritesheet is finished loading through a [`load_spritesheet`]
/// or similar function. Listen for it with [`EventReader<SpritesheetLoadedEvent>`]
#[derive(Event, Debug)]
pub struct SpritesheetLoadedEvent {
    /// The handle to the spritesheet that just finished loading
    pub handle: Handle<Spritesheet>,
    /// The handle to the JSON data asset that was used to generate the spritesheet
    pub data_handle: Handle<SpritesheetData>,
}

#[derive(Component)]
struct SpriteSheetLoader {
    on_complete: Option<fn(&mut Spritesheet)>,
}

// Implementations: -----------------------------------------------------------

impl Plugin for AsepritesheetPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(SpritesheetAssetLoader {
            extensions: self.extensions.clone(),
        })
        .init_asset::<SpritesheetData>()
        .init_asset::<Spritesheet>()
        .add_event::<AnimFinishEvent>()
        .add_event::<SpritesheetLoadedEvent>()
        .add_systems(PreUpdate, handle_spritesheet_loading);
        if let Some(schedule) = self.schedule {
            app.insert_resource(SpriteAnimController::default())
                .configure_sets(
                    schedule,
                    AnimationSet
                        .after_ignore_deferred(add_needed_atlas_handles)
                        .after_ignore_deferred(add_needed_img_handles),
                )
                .add_systems(
                    schedule,
                    (
                        add_needed_atlas_handles,
                        add_needed_img_handles,
                        animate_sprites.in_set(AnimationSet),
                    ),
                );
        } else {
            app.add_systems(
                PreUpdate,
                (add_needed_atlas_handles, add_needed_img_handles),
            );
        }
    }
}

impl Default for SpriteAnimController {
    fn default() -> Self {
        Self {
            is_active: true,
            global_time_scale: 1.0,
        }
    }
}

impl AsepritesheetPlugin {
    /// create a new instance of the asepritesheet plugin
    pub fn new(extensions: &[&'static str]) -> Self {
        AsepritesheetPlugin {
            schedule: Some(PostUpdate.intern()),
            extensions: extensions.to_owned(),
        }
    }
    /// can be used to set the schedule that the animator system runs in, default is [`PostUpdate`]
    pub fn in_schedule(mut self, schedule: impl ScheduleLabel) -> Self {
        self.schedule = Some(schedule.intern());
        self
    }
    /// spritesheets will not automatically animate, you will have to call the 'animate' function
    /// on the [`SpriteAnimator`] component manually
    pub fn without_anim(mut self) -> Self {
        self.schedule = None;
        self
    }
}

// Systems: -------------------------------------------------------------------

fn handle_spritesheet_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spritesheet_data_assets: Res<Assets<SpritesheetData>>,
    mut load_event_writer: EventWriter<SpritesheetLoadedEvent>,
    mut spritesheet_assets: ResMut<Assets<Spritesheet>>,
    mut atlas_assets: ResMut<Assets<TextureAtlasLayout>>,
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

                // execute callback
                if let Some(callback) = loader.on_complete {
                    (callback)(sheet);
                }

                // destroy loader entity and send the event signal that it's finished loading
                commands.entity(ent).despawn();
                load_event_writer.send(SpritesheetLoadedEvent {
                    handle: handle_spr.clone(),
                    data_handle: handle_spr_dat.clone(),
                });
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
