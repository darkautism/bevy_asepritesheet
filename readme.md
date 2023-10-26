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
	are parsed by the package and turned into individual frames on a 
	TextureAtlas  
* Frame Trimming ✅ - Aseprite can trim frame rects to be tight and fitted 
	together to avoid unnecesssary spacing between frames, this package can 
	account for each frame's trim and adjust anchors accordingly so animations 
	don't jitter when this feature is used  
* Frame Duration ✅ -  
* FrameTags ✅ -  
* FrameTag Range ✅ -  
* FrameTag Direction ❌ -  