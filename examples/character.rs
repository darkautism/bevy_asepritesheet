use bevy::{
    prelude::*,
    sprite::Anchor, 
    core_pipeline::clear_color::ClearColorConfig
};
use bevy_asepritesheet::*;

#[derive(Resource)]
struct SpriteHandleResource(
    Handle<aseprite_data::SpritesheetData>, 
    Option<sprite::Sheet>
);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Playing
}

// -----------------------------------------------------------------------------

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            asset_plugin::SpritesheetAssetPlugin::new(&["sprite.json"])
        ))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            load.run_if(in_state(AppState::Loading)),
            sprite_animator::animate_sprites,
            control_animation
        ))
    ;
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(
        SpriteHandleResource(asset_server.load("witch.sprite.json"), None)
    );
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
    sprite_assets: Res<Assets<aseprite_data::SpritesheetData>>,
    image_assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>
) {

    if witch_data_handle.1.is_none() {
        if let Some(witch_data) = sprite_assets.get(&witch_data_handle.0) {
            println!("Sprite Data Loaded!");
            witch_data_handle.1 = Some(sprite::Sheet::from_data_image(
                &witch_data,
                asset_server.load::<Image, _>(&witch_data.meta.image),
                Anchor::Center
            ));
        }
    }

    else {
        let witch_spritesheet = witch_data_handle.1.as_mut().unwrap();
        if image_assets.contains(witch_spritesheet.img_handle()) {

            let non_looping_anims = [
                "bow",
                "fall_transition",
                "fall_land",
                "attack_light",
                "attack_heavy",
                "damage"
            ];
            for anim_name in non_looping_anims {
                if let Some(anim) = witch_spritesheet.get_anim_mut(
                    &witch_spritesheet.get_anim_handle(anim_name).unwrap()
                ) {
                    anim.end_action = sprite::AnimEndAction::Pause;
                }
            }

            println!("Image Loaded!");
            state.set(AppState::Playing);
            commands.spawn(
                sprite_animator::AnimatedSpriteBundle{
                    sprite: SpriteSheetBundle{
                        texture_atlas: witch_spritesheet.create_atlas_handle(
                            &mut atlas_assets
                        ),
                        transform: Transform::from_scale(
                            Vec3::new(2.0, 2.0, 1.0)
                        ),
                        ..Default::default()
                    },
                    animator: sprite_animator::SpriteAnimator::from_sheet(
                            witch_data_handle.1.as_ref().unwrap().clone()
                    )
                }
            );
        }
    }
}

fn control_animation(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut sprite_animator::SpriteAnimator>
) {
    let mut anim_index:Option<usize> = None;
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
            _ => None
        };
        if anim_index.is_some() { break; }
    }
    if anim_index.is_none() { return; }
    let anim_index = anim_index.unwrap();

    for mut sprite_animator in &mut query {
        let _ = sprite_animator.set_anim_index(anim_index);
        sprite_animator.time_scale = 1.0;
    }
}