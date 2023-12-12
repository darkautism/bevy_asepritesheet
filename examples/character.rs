// This example creates a simple bevy app that loads a character animation spritesheet and allows
// the user to switch animations by using the number keys 0-9 and the top row letter keys Q-P.
// The spritesheet animations are set up to use some useful features such as animation transitions,
// animation end actions, and animation events.

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::Anchor};
use bevy_asepritesheet::{
    aseprite_data::SpritesheetData,
    asset_plugin::SpritesheetAssetPlugin,
    sprite::{AnimEndAction, Spritesheet},
    sprite_animator::{AnimFinishEvent, AnimatedSpriteBundle, SpriteAnimator},
};

// Entry Point: ----------------------------------------------------------------

fn main() {
    App::new()
        // adding the SpritesheetAssetPlugin adds the asset loader to the game
        // and the AnimFinishEvent event which is sent whenever any animation
        // an animated spritesheet is complete
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAssetPlugin::new(&["sprite.json"]),
        ))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                load.run_if(in_state(AppState::Loading)),
                control_animation,
                log_anim_events,
            ),
        )
        .run();
}

// Struct Definitions: ---------------------------------------------------------

#[derive(Resource)]
struct SpriteDataResource {
    entity_created: bool,
    spritesheet_data: Handle<SpritesheetData>,
    spritesheet: Option<Spritesheet>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Playing,
}

// Utility: --------------------------------------------------------------------

/// format the spritesheet animations for the witch character
fn format_witch_anims(sheet: &mut Spritesheet) -> Result<(), ()> {
    // get handles for all the needed animations
    let handle_idle = sheet.get_anim_handle("idle");
    let _handle_running = sheet.get_anim_handle("running");
    let handle_bow = sheet.get_anim_handle("bow");
    let handle_jump_prepare = sheet.get_anim_handle("jump_prepare");
    let handle_jump = sheet.get_anim_handle("jump");
    let handle_fall_transition = sheet.get_anim_handle("fall_transition");
    let handle_falling = sheet.get_anim_handle("falling");
    let handle_fall_land = sheet.get_anim_handle("fall_land");
    let _handle_slide = sheet.get_anim_handle("slide");
    let handle_attack_light = sheet.get_anim_handle("attack_light");
    let handle_attack_heavy = sheet.get_anim_handle("attack_heavy");
    let handle_damage = sheet.get_anim_handle("damage");
    let _handle_face_background = sheet.get_anim_handle("face_background");

    // have the bow animation pause at the end
    let anim_bow = sheet.get_anim_mut(&handle_bow)?;
    anim_bow.end_action = AnimEndAction::Pause;

    // when the jump prepare animation finishes, play the jump animation
    let anim_jump_prepare = sheet.get_anim_mut(&handle_jump_prepare)?;
    anim_jump_prepare.end_action = AnimEndAction::Next(handle_jump);

    // when the jump animation finishes, play the fall transition animation
    let anim_jump = sheet.get_anim_mut(&handle_jump)?;
    anim_jump.end_action = AnimEndAction::Next(handle_fall_transition);

    // when the fall transition animation finishes, play the falling animation
    let anim_fall_transition = sheet.get_anim_mut(&handle_fall_transition)?;
    anim_fall_transition.end_action = AnimEndAction::Next(handle_falling);

    // when the falling animation finishes, play the fall land animation
    let anim_falling = sheet.get_anim_mut(&handle_falling)?;
    anim_falling.end_action = AnimEndAction::Next(handle_fall_land);

    // when the fall land animation finishes, play the idle animation
    let anim_fall_land = sheet.get_anim_mut(&handle_fall_land)?;
    anim_fall_land.end_action = AnimEndAction::Next(handle_idle);

    // when the attack light animation finishes, play the idle animation
    let anim_attack_light = sheet.get_anim_mut(&handle_attack_light)?;
    anim_attack_light.end_action = AnimEndAction::Next(handle_idle);

    // when the attack_heavy animation finishes, play the idle animation
    let anim_attack_heavy = sheet.get_anim_mut(&handle_attack_heavy)?;
    anim_attack_heavy.end_action = AnimEndAction::Next(handle_idle);

    // when the damage animation finishes, play the idle animation
    let anim_damage = sheet.get_anim_mut(&handle_damage)?;
    anim_damage.end_action = AnimEndAction::Next(handle_idle);

    Ok(())
}

// Systems: --------------------------------------------------------------------

/// Initial set up system that runs at start of the game
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // insert the resource that holds information about our spritesheet asset
    commands.insert_resource(SpriteDataResource {
        spritesheet_data: asset_server.load("witch.sprite.json"),
        spritesheet: None,
        entity_created: false,
    });

    // create the camera so we can see the sprite
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.3, 0.3, 0.3)),
            ..Default::default()
        },
        ..Default::default()
    });
}

/// System that handles loading/processing assets while in 'loading' appstate
fn load(
    mut commands: Commands,
    mut witch_data: ResMut<SpriteDataResource>,
    sprite_assets: Res<Assets<SpritesheetData>>,
    image_assets: Res<Assets<Image>>,
    mut atlas_assets: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>,
    mut anim_spr_queue: Query<(&mut Handle<TextureAtlas>, &mut SpriteAnimator)>,
) {
    // if the witch sprite object is not yet created
    if witch_data.spritesheet.is_none() {
        // if the spritesheet data is loaded and parsed
        if let Some(witch_sheet_data) = sprite_assets.get(&witch_data.spritesheet_data) {
            println!("Sprite Data Loaded!");

            // create the spritesheet
            let mut sheet = Spritesheet::from_data(
                &witch_sheet_data,
                &asset_server,
                &Anchor::Center,
                &mut atlas_assets,
            );

            // set up the animations to behave properly
            format_witch_anims(&mut sheet).expect("ERROR: Could not format animations");

            // store the spritesheet in the data resource
            witch_data.spritesheet = Some(sheet);
        }
    }

    // after spritesheet has been created and stored in the resource
    if witch_data.entity_created {
        if let Some(witch_sheet) = witch_data.spritesheet.as_ref() {
            if image_assets.contains(&witch_sheet.img_handle()) {
                println!("Image Loaded!");
                let (mut atlas, mut anim_spr) = anim_spr_queue.single_mut();

                // apply the sprite sheet and it's texture atlas to the
                // animated sprite entity
                anim_spr.set_spritesheet(witch_sheet.clone());
                *atlas = witch_sheet.atlas_handle().clone().unwrap();

                // finish the loading state of the app and move on
                state.set(AppState::Playing);
            }
        }
    }

    // create entity if it hasn't yet been created
    if !witch_data.entity_created {
        // spawn the animated sprite entity
        commands.spawn(
            // use the animated sprite bundle to spawn an entity with all
            // the needed components to have an animated sprite
            AnimatedSpriteBundle {
                sprite_bundle: SpriteSheetBundle {
                    transform: Transform::from_scale(Vec3::new(4.0, 4.0, 1.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        // mark the flag so it doesn't get created twice
        witch_data.entity_created = true;
    }
}

/// System that allows the player to select the character animation with keys
/// 0 - 9 and q - p
fn control_animation(input: Res<Input<KeyCode>>, mut query: Query<&mut SpriteAnimator>) {
    // get animation index from keypress
    let mut anim_index: Option<usize> = None;
    for key in input.get_just_pressed() {
        anim_index = match key {
            KeyCode::Key0 => Some(0),
            KeyCode::Key1 => Some(1),
            KeyCode::Key2 => Some(2),
            KeyCode::Key3 => Some(3),
            KeyCode::Key4 => Some(4),
            KeyCode::Key5 => Some(5),
            KeyCode::Key6 => Some(6),
            KeyCode::Key7 => Some(7),
            KeyCode::Key8 => Some(8),
            KeyCode::Key9 => Some(9),
            KeyCode::Q => Some(10),
            KeyCode::W => Some(11),
            KeyCode::E => Some(12),
            KeyCode::R => Some(13),
            KeyCode::T => Some(14),
            KeyCode::Y => Some(15),
            KeyCode::U => Some(16),
            KeyCode::I => Some(17),
            KeyCode::O => Some(18),
            KeyCode::P => Some(19),
            _ => None,
        };
        if anim_index.is_some() {
            break;
        }
    }
    if anim_index.is_none() {
        return;
    }
    let anim_index = anim_index.unwrap();

    // apply the animation index, or log warning if invalid index
    for mut sprite_animator in &mut query {
        sprite_animator.time_scale = 1.0;
        if !sprite_animator.set_anim_index(anim_index) {
            warn!("WARN: Invalid index");
        }
    }
}

/// System that handles logging a message whenever an animation finishes playing
fn log_anim_events(mut events: EventReader<AnimFinishEvent>, animators: Query<&SpriteAnimator>) {
    for event in events.read() {
        // don't print the message if the animation is looping
        if let Ok(animator) = animators.get(event.entity) {
            if let Some(anim_sheet) = animator.spritesheet() {
                if let Ok(anim) = anim_sheet.get_anim(&event.anim) {
                    if anim.end_action == AnimEndAction::Loop {
                        continue;
                    }
                }
            }
        }
        println!("Animation {:?} complete!", event.anim);
    }
}
