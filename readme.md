# Bevy Asepritesheet

Bevy Asepritesheet is an asset loader and parser for 
[Bevy game engine](https://bevyengine.org/). Bevy Asepritesheet processes json 
spritesheet files exported by [Aseprite software](https://www.aseprite.org/).
It also has components and systems for building and managing animated sprite 
entities in the game engine.

## Features

Bevy Asepritesheet supports most features that aseprite exports as json data 
alongside your spritesheets. Here is a list of Aseprite features that are 
exported and whether they are supported in Bevy Asepritesheet or not:

Key:  

✅ - fully supported and implementated  
❌ - not supported  

Features:  
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
	included in the animation by this library  
* FrameTag Direction ❌ - In Aseprite you can specify that a FrameTag either 
	plays in reverse or forwards, I don't really see a point to this so I'm not
	going to implement it, you can always reverse the frames if you want to 
	reverse the animation  