// This is the most basic use example from the readme.md

use bevy::{prelude::*, sprite::Anchor};
use bevy_asepritesheet::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AsepritesheetPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn the camera so we can see the sprite
    commands.spawn(Camera2dBundle::default());

    // load the spritesheet and get it's handle
    let sheet_handle = load_spritesheet(
        &mut commands,
        &asset_server,
        "witch.sprite.json",
        Anchor::Center,
    );

    // spawn the animated sprite
    commands.spawn(AnimatedSpriteBundle {
        animator: SpriteAnimator::from_anim(AnimHandle::from_index(1)),
        spritesheet: sheet_handle,
        ..Default::default()
    });
}
