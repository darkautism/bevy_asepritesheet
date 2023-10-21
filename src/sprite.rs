use std::{ops::*, usize};
use bevy::{
	prelude::*, 
	sprite::Anchor
};

use crate::aseprite_data;

#[derive(Clone, Debug)]
pub struct Sheet {
	pub frames: Vec<Frame>,
	anims: Vec<Anim>,
	img_handle: Handle<Image>,
	img_size: Vec2,
	atlas_handle: Option<Handle<TextureAtlas>>
}

#[derive(Clone, Debug)]
pub struct Anim {
	pub name: String,
	pub frames: Vec<usize>,
	pub time_scale: f32,
	pub end_action: AnimEndAction
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimHandle{
	index: usize
}

#[derive(Clone, Debug)]
pub struct Frame {
	pub atlas_index: usize,
	pub duration: f32,
	pub anchor: Anchor,
	rect: Rect // TODO remove ?
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum AnimEndAction {
	Pause,
	LoopOver,
	PingPong
}

// -------------------------------------------------------------------------------------------------

#[allow(dead_code)]
impl Sheet {
	
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
				.mul(Vec2::from(frame_data.source_size))
				.sub(frame_offset)
				.div(trimmed_frame_size)
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
				frames: (tag_data.from .. tag_data.to).collect(),
				time_scale: 1.0,
				end_action: AnimEndAction::LoopOver
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

	pub fn img_handle(&self) -> &Handle<Image> {
		&self.img_handle
	}

	pub fn img_size(&self) -> Vec2 {
		self.img_size.clone()
	}

	pub fn anim_count(&self) -> usize { self.anims.len() }

	pub fn atlas_handle(&self) -> Option<&Handle<TextureAtlas>> {
		if self.atlas_handle.is_none() { None } else { Some(self.atlas_handle.as_ref().unwrap()) }
	}

	pub fn create_atlas_handle(
		&mut self, 
		atlas_assets: &mut Assets<TextureAtlas>
	) -> Handle<TextureAtlas> {
		if let Some(handle) = &self.atlas_handle {
			atlas_assets.remove(handle);
		}
		let mut atlas = TextureAtlas::new_empty(self.img_handle().clone(), self.img_size());
		for frame in &self.frames {
			atlas.add_texture(frame.rect.clone());
		}
		let handle = atlas_assets.add(atlas);
		self.atlas_handle = Some(handle.clone());
		handle
	}

	pub fn get_anim_handle<T: AsRef<str>>(&self, name: T) -> Option<AnimHandle> {
		for (i, anim) in self.anims.iter().enumerate() {
			if anim.name == name.as_ref() { return Some(AnimHandle { index: i }) }
		}
		None
	}

	pub fn get_anim(&self, handle: &AnimHandle) -> Option<&Anim> {
		if self.anims.len() > handle.index {
			Some(&self.anims[handle.index])
		}
		else{
			None
		}
	}

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
	pub fn from_index(index: usize) -> Self {
		AnimHandle { index }
	}
}