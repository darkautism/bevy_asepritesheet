// This is the most basic use example from the readme.md

use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;

fn main() {
	App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAssetPlugin::new(&["sprite.json"])
        ))
		.add_systems(Startup, setup)
		.add_systems(
			Update, 
			create_entity.run_if(
				resource_exists::<SpritesheetDataHandle>()
		))
    	.run();
}

#[derive(Resource)]
struct SpritesheetDataHandle(Handle<SpritesheetData>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(
		SpritesheetDataHandle(asset_server.load("witch.sprite.json"))
	);
	commands.spawn(Camera2dBundle::default());
}

fn create_entity(
	mut commands: Commands, 
	sheet_hndl_res: Res<SpritesheetDataHandle>,
	asset_server: Res<AssetServer>,
	sheet_assets: Res<Assets<SpritesheetData>>,
	mut atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
	// ensure the spritesheet data is loaded and retrieve it into sheet_data
	if let Some(sheet_data) = sheet_assets.get(&sheet_hndl_res.0) {
		// create the spritesheet instance from the spritesheet data
		let mut sheet = Spritesheet::from_data(
			&sheet_data, 
			&asset_server, 
			bevy::sprite::Anchor::default(),
		);
		// create entity with the animated sprite bundle and spritesheet data
		commands.spawn(
            AnimatedSpriteBundle {
                sprite: SpriteSheetBundle {
                    texture_atlas: 
						sheet.create_atlas_handle(&mut atlas_assets),
                    ..Default::default()
                },
                animator: SpriteAnimator::from_sheet(sheet),
            }
        );
		// remove the resource so this system no longer runs
		commands.remove_resource::<SpritesheetDataHandle>();
	}
}