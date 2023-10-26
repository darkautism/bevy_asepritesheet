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

## Usage Example

First, you'll need to add the dependency to your `Cargo.toml`` file:  
```toml
[dependencies]
bevy_aseprite = "0.2"
```

### NOTE  
Note - this will not actually work yet since I have not registered the package
as a rust crate

Then, you will need to add the plugin to your bevy app:  
```rs
use bevy::prelude::*;
use bevy_asepritesheet::asset_plugin::SpritesheetAssetPlugin;
use bevy_asepritesheet::sprite_animator::SpriteAnimator;
use bevy_asepritesheet::aseprite_data::SpritesheetData;

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
	let handle = /*magically gets handle*/;
	if let Some(sheet_data) = sheet_assets.get(&handle) {
		commands.spawn(
            AnimatedSpriteBundle{
                sprite: SpriteSheetBundle{
                    texture_atlas: sheet_data.create_atlas_handle(&mut atlas_assets),
                    transform: Transform::from_scale(Vec3::new(4.0, 4.0, 1.0)),
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

For a more complete example, see [examples/character.rs](examples/character.rs).