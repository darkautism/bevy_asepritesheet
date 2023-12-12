// This example creates a simple bevy app that loads a character animation spritesheet and allows
// the user to switch animations by using the number keys 0-9 and the top row letter keys Q-P.
// The spritesheet animations are set up to use some useful features such as animation transitions,
// animation end actions, and animation events.

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::Anchor};
use bevy_asepritesheet::prelude::*;

// Entry Point: ----------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            // adding the SpritesheetAssetPlugin adds the asset loader to the game
            // and the AnimFinishEvent event which is sent whenever any animation
            // an animated spritesheet is complete
            AsepritesheetPlugin::new(&["sprite.json"]),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (control_animation, log_anim_events))
        .run();
}

// Utility: --------------------------------------------------------------------

/// format the spritesheet animations for the witch character
fn format_witch_anims(sheet: &mut Spritesheet) {
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
    if let Ok(anim_bow) = sheet.get_anim_mut(&handle_bow) {
        anim_bow.end_action = AnimEndAction::Pause;
    }

    // when the jump prepare animation finishes, play the jump animation
    if let Ok(anim_jump_prepare) = sheet.get_anim_mut(&handle_jump_prepare) {
        anim_jump_prepare.end_action = AnimEndAction::Next(handle_jump);
    }

    // when the jump animation finishes, play the fall transition animation
    if let Ok(anim_jump) = sheet.get_anim_mut(&handle_jump) {
        anim_jump.end_action = AnimEndAction::Next(handle_fall_transition);
    }

    // when the fall transition animation finishes, play the falling animation
    if let Ok(anim_fall_transition) = sheet.get_anim_mut(&handle_fall_transition) {
        anim_fall_transition.end_action = AnimEndAction::Next(handle_falling);
    }

    // when the falling animation finishes, play the fall land animation
    if let Ok(anim_falling) = sheet.get_anim_mut(&handle_falling) {
        anim_falling.end_action = AnimEndAction::Next(handle_fall_land);
    }

    // when the fall land animation finishes, play the idle animation
    if let Ok(anim_fall_land) = sheet.get_anim_mut(&handle_fall_land) {
        anim_fall_land.end_action = AnimEndAction::Next(handle_idle);
    }

    // when the attack light animation finishes, play the idle animation
    if let Ok(anim_attack_light) = sheet.get_anim_mut(&handle_attack_light) {
        anim_attack_light.end_action = AnimEndAction::Next(handle_idle);
    }

    // when the attack_heavy animation finishes, play the idle animation
    if let Ok(anim_attack_heavy) = sheet.get_anim_mut(&handle_attack_heavy) {
        anim_attack_heavy.end_action = AnimEndAction::Next(handle_idle);
    }

    // when the damage animation finishes, play the idle animation
    if let Ok(anim_damage) = sheet.get_anim_mut(&handle_damage) {
        anim_damage.end_action = AnimEndAction::Next(handle_idle);
    }
}

// Systems: --------------------------------------------------------------------

/// Initial set up system that runs at start of the game
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // create a camera so we can see the sprite
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.3, 0.3, 0.3)),
            ..Default::default()
        },
        ..Default::default()
    });

    // load the spritesheet and get it's handle to add to the animated sprite bundle
    let spritesheet_handle = load_spritesheet_then(
        &mut commands,
        &asset_server,
        "witch.sprite.json",
        Anchor::Center,
        // here we can define a callback with a mutable reference to the spritesheet that was 
        // loaded so that it can be tweaked after loading is complete
        |sheet| {
            println!("Spritesheet finished loading!");
            format_witch_anims(sheet);
        },
    );

    // create the animated sprite entity
    commands.spawn(AnimatedSpriteBundle {
        spritesheet: spritesheet_handle,
        ..Default::default()
    });
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
        sprite_animator.set_anim_index(anim_index);
    }
}

/// System that handles logging a message whenever an animation finishes playing
fn log_anim_events(
    mut events: EventReader<AnimFinishEvent>,
    spritesheet_assets: Res<Assets<Spritesheet>>,
    animated_sprite_query: Query<&Handle<Spritesheet>, With<SpriteAnimator>>,
) {
    for event in events.read() {
        // get the spritesheet handle off the animated sprite entity
        if let Ok(sheet_handle) = animated_sprite_query.get(event.entity) {
            if let Some(anim_sheet) = spritesheet_assets.get(sheet_handle) {
                // get the animation reference from the spritesheet
                if let Ok(anim) = anim_sheet.get_anim(&event.anim) {
                    // don't log anything if it's a looping animation
                    if anim.end_action == AnimEndAction::Loop {
                        continue;
                    }
                }
            }
        }
        println!("Animation {:?} complete!", event.anim);
    }
}
