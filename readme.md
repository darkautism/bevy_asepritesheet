# Bevy Asepritesheet

Bevy Asepritesheet is an asset loader and parser for 
[Bevy game engine](https://bevyengine.org/). Bevy Asepritesheet processes json 
spritesheet files exported by [Aseprite software](https://www.aseprite.org/).
It also has components and systems for building and managing animated sprite 
entities in the game engine.

Bevy Asepritesheet is capable of parsing spritesheets that look like this into 
distinct animations based on the aseprite json data.

![Spritesheet](/assets/witch.png) ![Animation](/media/example.gif)

#### Important! - Json data export from Aseprite MUST be set to "Array"

![array_mode](/media/export_json_array.png)  
If set to "Hash", it will not work!

## Features

* Asset loader - asset loader can be specified to use whatever file extensions
you need  
* Sprite animator - the library includes a sprite animator component and 
bundle to help you spawn entities which have animated sprites on them  
* Plugin - the library includes a bevy plugin that you can add to the bevy 
app, this adds the asset loader, registers the animation event system, and sets 
up a system that automatically animates any sprite animator components in the 
game world  
* Animation events - animations send out an animation event whenever an 
animation ends, with data about what entity and what animation it was, so you
can listen to them and define your own behavior  
* Animation end actions - animations can loop, pause, stop, or transition to 
another animation upon completion, all configurable through the library

### Aseprite Feature Support

Bevy Asepritesheet supports most features that aseprite exports as json data 
alongside your spritesheets. Here is a list of Aseprite features that are 
exported and whether they are supported in Bevy Asepritesheet or not:

Key:  

✅ - fully supported and implementated  
❌ - not supported  

Aseprite Features:  
* Frame ✅ - Frame rects for each individual frame of the sprite sheet, these 
	are parsed by this library and turned into individual frames on a 
	TextureAtlas  
* Frame Trimming ✅ - Aseprite can trim frame rects to be tightly fitted 
	together to avoid unnecesssary spacing between frames, this library can 
	account for each frame's trim and adjust anchors accordingly so animations 
	don't jitter when this feature is used  
* Frame Duration ✅ - In aseprite, each frame has a specific duration that it
	shows in the animation for, this library includes that data in the animation 
	which will be reflected when the animation is played in-game  
* FrameTags ✅ - Frame tags are basicaly groups of frames in asprite that can 
	have a "tag", this library interprets these tags as animations, so if you 
	have a FrameTag of 10 frames in aserite named "running", this library will
	generate a "running" animation from that tag  
* FrameTag Range ✅ - the frames specified to be part of a frameTag are 
	included in the animation in game
* FrameTag Direction ❌ - In Aseprite you can specify that a FrameTag either 
	plays in reverse or forwards, I don't really see a point to this so I'm not
	going to implement it, you can always reverse the frames if you want to 
	reverse the animation  

## Compatibility

| bevy_asepritesheet | bevy | aseprite |
| ---- | ---- | ---- |
| 0.4.x | 0.12 | 1.2.40 |
| 0.3.x | 0.12 | 1.2.40 |
| 0.2.x | 0.11 | 1.2.40 |

## Usage Example

First, you'll need to add the dependency to your `Cargo.toml`` file:  
```toml
[dependencies]
bevy_aseprite = "0.4"
```

Then, you will need to add the plugin to your bevy app:  
```rs
use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;
fn main() {
	App::new()
        .add_plugins((SpritesheetAssetPlugin::new(&["sprite.json"])))
		.add_systems(Startup, setup)
		.add_systems(Update, create_entity
			.run_if(resource_exists::<SheetDataHandle>()))
    	.run();
}
```

And now you're able to load your json assets:  
```rs
#[derive(Resource)]
struct SheetDataHandle(Handle<SpritesheetData>);
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(
		SpritesheetDataHandle(asset_server.load("witch.sprite.json"))
	);
	commands.spawn(Camera2dBundle::default());
}
```

Once the asset is loaded, you can create the entity from the bundle:
```rs
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
```

### Run the example

To run the example:
```
cargo run --example character
```

To see the complete example, see [examples/character.rs](examples/character.rs).

# Asset Credit

The asset I used is a modified version of a free assset made by Legnops  
https://legnops.itch.io/red-hood-character

## Changelog

### 0.5.0

* ❌ update examples
* ❌ resource to handle animation logic
* ✅ utility function load_spritesheet to automatically load related assets
* ✅ rework how spritesheets are referenced from animator, now use handles
* ❌ get_anim_handle is now less strict (but the handles it returns can be invalid)
* ✅ set_anim is now less strict

### 0.4.2

* ✅ small bugfix

### 0.4.1

* ✅ fix bug that caused flipped sprites to have misaligned anchors  
* ✅ implement default for AnimHandle
* ✅ add is_cur_anim() to SpriteAnimator for easy compare current animation
* ✅ add restart_anim() to restart current animation of sprite animator

### 0.4.0

* ✅ update examples  
* ✅ fix oversight that made difficult to use `Spritesheet::from_data()`  
* ✅ impl default for `AnimatedSpriteBundle`  
* ✅ impl default for `SpriteAnimator`  
* ✅ make `SpriteAnimator` not rely on having loaded sheet when initialized  
* ✅ rename `Sheet` type to `Spritesheet`  
* ✅ general code refactors to align better with idomatic rust ideals  

### 0.3.0

* ✅ adapt asset management module to work with bevy 0.12