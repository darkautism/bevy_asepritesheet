use std::{ops::*, usize};
use bevy::{
	prelude::*, 
	sprite::Anchor
};
use crate::aseprite_data;

// Struct Definitions: ---------------------------------------------------------

/// A spritesheet object containing processed data from the deserialized 
/// aseprite data. Used as reference data for the 
/// [`crate::sprite_animator::SpriteAnimator`] component
#[derive(Clone, Debug)]
pub struct Sheet {

	/// A set of every possible frame that can be used for an animation within 
	/// the spritesheet
	pub frames: Vec<Frame>,
	anims: Vec<Anim>,
	img_handle: Handle<Image>,
	img_size: Vec2,
	atlas_handle: Option<Handle<TextureAtlas>>
}

/// A parsed spritesheet animation that determines which sprite frames will be 
/// drawn when active
#[derive(Clone, Debug)]
pub struct Anim {

	/// The name of the animation that can be used to find it with 
	/// [`Sheet::get_anim_handle``]
	pub name: String,

	/// A set of the individual frames of the animation
	pub frames: Vec<usize>,

	/// A speed multiplier for the animation play rate, normal rate is 1.0, 
	/// 0.0 is completely paused, and 2.0 will play twice as fast
	pub time_scale: f32,

	/// How the animation behaves when it reaches the end
	pub end_action: AnimEndAction
}

/// A handle for [`Anim`] that can be used as a reference to play specific
/// animations on a spritesheet
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimHandle{
	index: usize
}

/// An animation frame, a single atomic piece of an animation, that holds 
/// information about the sprite as it should appear when the frame is active
#[derive(Clone, Debug)]
pub struct Frame {

	/// the index of the sprite frame rect on the texture atlas
	pub atlas_index: usize,

	/// the duration that the frame is displayed for, in seconds
	pub duration: f32,

	/// the anchor point of the frame
	pub anchor: Anchor,

	/// the frame rect - only used when building the texture atlas, maybe 
	/// not necessary?
	rect: Rect // TODO remove ?
}

/// Enum for setting different end behaviors of a sprite's animation, 
/// default is [`AnimEndAction::Loop`]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum AnimEndAction {
	
	/// Stop the animation after completion, sets current animation to [`None`]
	Stop,

	// Pause the animator after completion
	Pause,

	// Loop through the animation, restarts from the beginning upon completion
	Loop,

	// Play forward and then backward and repeat indefinitely
	PingPong
}

// Struct Implementations: -----------------------------------------------------

#[allow(dead_code)]
impl Sheet {
	
	/// Create a new spritesheet object from the specified data, should 
	/// generally not be used unless you are generating spritesheets entirely
	/// in code
	pub fn new(
		frames: Vec<Frame>, 
		anims: Vec<Anim>, 
		img_handle: Handle<Image>, 
		img_size: Vec2
	) -> Self {
		Sheet { 
			frames: frames,
			anims: anims, 
			img_handle: img_handle, 
			img_size: img_size,
			atlas_handle: None
		}
	}

	/// Create a new spritesheet from given aseprite json data and a specified
	/// image asset. Use if the image path in the aseprite data does not 
	/// properly point to the location of the image asset. NOTE: image paths
	/// are NOT relative to the json file, they are relative to the bevy asset
	/// directory
	pub fn from_data_image(
		data: &aseprite_data::SpriteSheetData, 
		img_handle: Handle<Image>, 
		frame_anchor: Anchor
	) -> Self {
		let mut frames = Vec::<Frame>::new();
		for (i, frame_data) in data.frames.iter().enumerate() {
			let frame_offset = Vec2::new(
				frame_data.sprite_source_size.x as f32,
				frame_data.sprite_source_size.y as f32
			);
			let trimmed_frame_size = Vec2::new(
				frame_data.frame.w as f32,
				frame_data.frame.h as f32
			);
			let anchor_target = frame_anchor.as_vec()
				.add(Vec2::splat(0.5))
				.mul(Vec2::from(frame_data.source_size))
				.sub(frame_offset)
				.div(trimmed_frame_size)
				.sub(Vec2::splat(0.5))
				.mul(Vec2::new(1.0, -1.0))
			;
			let frame = Frame{
				atlas_index: i,
				duration: frame_data.duration as f32 * 0.001,
				anchor: Anchor::Custom(anchor_target),
				rect: frame_data.frame.into()
			};
			frames.push(frame);
		}
		let mut anims = Vec::<Anim>::new();
		for tag_data in &data.meta.frame_tags {
			let anim = Anim {
				name: tag_data.name.clone(),
				frames: (tag_data.from ..= tag_data.to).collect(),
				time_scale: 1.0,
				end_action: AnimEndAction::Loop
			};
			anims.push(anim);
		}
		Sheet::new(
			frames,
			anims,
			img_handle,
			data.meta.size.into()
		)
	}

	/// Create a spritesheet from the specified parsed aseprite json data. The
	/// image will be loaded from the specified path in the aseprite json data
	/// NOTE: image paths are NOT relative to the json file, they are relative 
	/// to the bevy asset directory. If you need to specify a different image 
	/// path, you can load the image asset manually and pass the handle to 
	/// [`Sheet::from_data_image`] instead
	pub fn from_data(
		data: &aseprite_data::SpriteSheetData, 
		asset_server: AssetServer,
		frame_anchor: Anchor
	) -> Self {
		Sheet::from_data_image(
			data, 
			asset_server.load(&data.meta.image), 
			frame_anchor
		)
	}

	/// Get a reference to the image handle that the spritesheet is using
	pub fn img_handle(&self) -> &Handle<Image> {
		&self.img_handle
	}

	/// Get the dimensions of the image asset being used by the spritesheet
	pub fn img_size(&self) -> Vec2 {
		self.img_size.clone()
	}

	/// Get the total amount of animations that the spritesheet contains
	pub fn anim_count(&self) -> usize { self.anims.len() }

	/// Get a reference to the texture atlas being used for the spritesheet
	pub fn atlas_handle(&self) -> Option<&Handle<TextureAtlas>> {
		if self.atlas_handle.is_none() { 
			None 
		} else { 
			Some(self.atlas_handle.as_ref().unwrap()) 
		}
	}

	/// Create a texture atlas with each sprite frame on it if there isn't one 
	/// already and return a handle to the atlas for referencing it later
	pub fn create_atlas_handle(
		&mut self, 
		atlas_assets: &mut Assets<TextureAtlas>
	) -> Handle<TextureAtlas> {
		if let Some(handle) = &self.atlas_handle {
			atlas_assets.remove(handle);
		}
		let mut atlas = TextureAtlas::new_empty(
			self.img_handle().clone(), 
			self.img_size()
		);
		for frame in &self.frames {
			atlas.add_texture(frame.rect.clone());
		}
		let handle = atlas_assets.add(atlas);
		self.atlas_handle = Some(handle.clone());
		handle
	}

	/// Get a handle to the animation with the specified name, if it exists
	pub fn get_anim_handle<T: AsRef<str>>(
		&self, 
		name: T
	) -> Option<AnimHandle> {
		for (i, anim) in self.anims.iter().enumerate() {
			if anim.name == name.as_ref() { 
				return Some(AnimHandle { index: i });
			}
		}
		None
	}

	/// Get a reference to the specified animation, if it exists
	pub fn get_anim(&self, handle: &AnimHandle) -> Option<&Anim> {
		if self.anims.len() > handle.index {
			Some(&self.anims[handle.index])
		}
		else{
			None
		}
	}

	/// Get a mutable reference to the specified animation, if it exists
	pub fn get_anim_mut(&mut self, handle: &AnimHandle) -> Option<&mut Anim> {
		if self.anims.len() > handle.index {
			Some(&mut self.anims[handle.index])
		}
		else{
			None
		}
	}
}

#[allow(dead_code)]
impl AnimHandle {

	/// Create an animation handle that refers to an animation of the specified
	/// index on any spritesheet
	pub fn from_index(index: usize) -> Self {
		AnimHandle { index }
	}
}