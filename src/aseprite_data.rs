use std::ops::*;
use bevy::{
	prelude::*,
	sprite::Anchor,
	reflect::{ 
		TypeUuid, 
		TypePath 
	}
};
use crate::sprite;

/// A container to hold the json data output from aseprite
#[derive(serde::Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "73461c8f-e760-4fb8-8492-37d5387fca7b"]
pub struct SpriteSheetData {
    pub frames: Vec<FrameData>,
    pub meta: MetaData
}

/// A container to hold some data about individual frames from the sprite sheet
#[derive(serde::Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "d49c70a1-177b-44ff-b427-d3929c928667"]
pub struct FrameData {
	pub filename: String,
    pub frame: RectData,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: RectData,
    #[serde(rename = "sourceSize")]
    pub source_size: SizeData,
    pub duration: u32
}

/// A container to hold information about frame tags defined in aseprite
#[derive(serde::Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "f88c0866-6ed2-4b45-a6b6-7dcbe8c53f21"]
pub struct FrameTagData {
    pub name: String,
    pub from: usize,
    pub to: usize,
    pub direction: String
}

/// A container to hold spritesheet metadata
#[derive(serde::Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "ea8ca8be-43c4-4b89-98e0-54afef524261" ]
pub struct MetaData {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: SizeData,
    pub scale: String,
    #[serde(rename = "frameTags")]
	pub frame_tags: Vec<FrameTagData>
}

/// A container to hold size data objects used to specify 2d sizes output from aseprite
#[derive(serde::Deserialize, TypeUuid, TypePath, Clone, Copy, Debug)]
#[uuid = "a168bfd8-e587-4e52-89b3-58b50f6c1823"]
pub struct SizeData {
    pub w: u16,
    pub h: u16
}

/// A container to hold rect data objects used to specify 2d rectangles output from aseprite
#[derive(serde::Deserialize, TypeUuid, TypePath, Clone, Copy, Debug)]
#[uuid = "4643df56-80d8-4f49-91df-67fc95307b30"]
pub struct RectData {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16
}

// -------------------------------------------------------------------------------------------------

#[allow(dead_code)]
impl SpriteSheetData {
	pub fn parse_spritesheet(&self, img_handle: &Handle<Image>, frame_anchor: Anchor) -> sprite::Sheet {
		let mut frames = Vec::<sprite::Frame>::new();
		for frame_data in &self.frames {
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
			let frame = sprite::Frame{
				rect: frame_data.frame.into(),
				duration: frame_data.duration as f32,
				anchor: Anchor::Custom(anchor_target)
			};
			frames.push(frame);
		}
		let mut anims = Vec::<sprite::Anim>::new();
		for tag_data in &self.meta.frame_tags {
			let anim = sprite::Anim {
				frames: (tag_data.from .. tag_data.to).collect(),
				time_scale: 1.0,
				end_action: sprite::AnimEndAction::LoopOver
			};
			anims.push(anim);
		}
		sprite::Sheet::new(
			frames,
			anims,
			img_handle.clone(),
			self.meta.size.into()
		)
	}
}

impl From<SizeData> for Vec2 {
	fn from(value: SizeData) -> Self {
		Vec2 { 
			x: value.w as f32, 
			y: value.h as f32
		}
	}
}

impl From<RectData> for Rect {
	fn from(value: RectData) -> Self {
		Rect { 
			min: Vec2 { x: value.x as f32, y: value.y as f32 }, 
			max: Vec2 { x: (value.x + value.w) as f32, y: (value.y + value.h) as f32 }
		}
	}
}