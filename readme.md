# Bevy Asepritesheet

Bevy Asepritesheet is an asset loader and parser for 
[Bevy game engine](https://bevyengine.org/). Bevy Asepritesheet processes json 
spritesheet files exported by [Aseprite software](https://www.aseprite.org/).
It also has components and systems for building and managing animated sprite 
entities in the game engine.

Bevy Asepritesheet is capable of parsing spritesheets that look like this into 
distinct animations based on the aseprite json data.

![Spritesheet](/assets/witch.png) ![Animation](/media/example.gif)

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

## Changelog

### 0.4.0

* ❌ update examples
* ✅ impl default for `AnimatedSpriteBundle`
* ✅ impl default for `SpriteAnimator`
* ✅ make `SpriteAnimator` not rely on having loaded sheet when initialized  
* ✅ rename `Sheet` type to `Spritesheet`  
* ✅ general code refactors to align better with idomatic rust ideals  

### 0.3.0

* ✅ adapt asset management module to work with bevy 0.12

## Usage Example

First, you'll need to add the dependency to your `Cargo.toml`` file:  
```toml
[dependencies]
bevy_aseprite = "0.4"
```

Then, you will need to add the plugin to your bevy app:  
```rs
use bevy::prelude::*;
use bevy_asepritesheet::asset_plugin::SpritesheetAssetPlugin;
use bevy_asepritesheet::sprite_animator::SpriteAnimator;
use bevy_asepritesheet::aseprite_data::*;

fn main() {
	App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAssetPlugin::new(&["sprite.json"])
        ))
    	.run();
}
```

And now you're able to load your json assets:  
```rs
// assume this system has been added to the app startup schedule
fn setup(asset_server: Res<AssetServer>) {
	let handle = asset_server.load("witch.sprite.json");
}
```

Once the asset is loaded, you can create the entity from the bundle:
```rs
// assume this system has been added to the app and the asset is loaded
fn create_entity(
	mut commands: Commands, 
	sheet_assets: Res<Assets<SpritesheetData>>,
	mut atlas_assets: Res<Assets<TextureAtlas>>
) {
	let handle = /*magically get handle to spritesheet asset*/;
	if let Some(sheet_data) = sheet_assets.get(&handle) {
		commands.spawn(
            AnimatedSpriteBundle{
                sprite: SpriteSheetBundle{
                    texture_atlas: 
						sheet_data.create_atlas_handle(&mut atlas_assets),
                    transform: 
						Transform::from_scale(Vec3::new(4.0, 4.0, 1.0)),
                    ..Default::default()
                },
                animator: SpriteAnimator::from_sheet(
                    witch_data_handle.spritesheet.as_ref().unwrap().clone()
                )
            }
        );
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