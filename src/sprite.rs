use std::ops::*;
use bevy::{
	prelude::*, 
	sprite::Anchor
};

use crate::aseprite_data;

#[derive(Clone, Debug)]
pub struct Sheet {
	pub frames: Vec<Frame>,
	pub anims: Vec<Anim>,
	img_handle: Handle<Image>,
	img_size: Vec2
}

#[derive(Clone, Debug)]
pub struct Anim {
	pub frames: Vec<usize>,
	pub time_scale: f32,
	pub end_action: AnimEndAction
}

#[derive(Clone, Debug)]
pub struct Frame {
	pub rect: Rect,
	pub duration: f32,
	pub anchor: Anchor
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
			img_size: img_size 
		}
	}

	pub fn img_handle(&self) -> &Handle<Image> {
		&self.img_handle
	}

	pub fn img_size(&self) -> Vec2 {
		self.img_size.clone()
	}

	pub fn get_atlas(&self) -> TextureAtlas {
		let mut atlas = TextureAtlas::new_empty(self.img_handle().clone(), self.img_size());
		for frame in &self.frames {
			atlas.add_texture(frame.rect.clone());
		}
		atlas
	}

	pub fn from_data(
		data: &aseprite_data::SpriteSheetData, 
		img_handle: Handle<Image>, 
		frame_anchor: Anchor
	) -> Self {
		let mut frames = Vec::<Frame>::new();
		for frame_data in &data.frames {
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
				rect: frame_data.frame.into(),
				duration: frame_data.duration as f32,
				anchor: Anchor::Custom(anchor_target)
			};
			frames.push(frame);
		}
		let mut anims = Vec::<Anim>::new();
		for tag_data in &data.meta.frame_tags {
			let anim = Anim {
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
}