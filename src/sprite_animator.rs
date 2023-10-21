use bevy::prelude::*;
use crate::sprite;

#[derive(Component)]
pub struct SpriteAnimator{
	spritesheet: sprite::Sheet,
	cur_time: f32,
	cur_anim: Option<sprite::AnimHandle>,
	last_frame_start: f32,
	last_anim_index: usize
}

// -------------------------------------------------------------------------------------------------

#[allow(dead_code)]
impl SpriteAnimator {

	pub fn from_sheet(sheet: sprite::Sheet) -> Self {
		SpriteAnimator { 
			spritesheet: sheet,
			cur_time: 0.0, 
			cur_anim: None,
			last_frame_start: 0.0,
			last_anim_index: 0
		}
	}

	pub fn spritesheet(&self) -> &sprite::Sheet { &self.spritesheet }

	pub fn spritesheet_mut(&mut self) -> &mut sprite::Sheet { &mut self.spritesheet }

	pub fn cur_time(&self) -> f32 { self.cur_time }

	pub fn cur_anim(&self) -> &Option<sprite::AnimHandle> { &self.cur_anim }

	pub fn set_anim(&mut self, anim: sprite::AnimHandle) {
		self.last_anim_index = 0;
		self.last_frame_start = 0.0;
		self.cur_time = 0.0;
		self.cur_anim = Some(anim);
	}

	pub fn stop_anim(&mut self){
		self.last_anim_index = 0;
		self.last_frame_start = 0.0;
		self.cur_time = 0.0;
		self.cur_anim = None;
	}

	pub fn animate(&mut self, sprite: &mut TextureAtlasSprite, delta: f32) {
		let cur_anim = 
			if let Some(val) = self.cur_anim.as_ref() { 
				if let Some(val2) = self.spritesheet.get_anim(val) { val2 } 
				else { return; }
			} 
			else { return; };

		let mut cur_frame = &self.spritesheet.frames[cur_anim.frames[self.last_anim_index]];
		let mut next_frame_time = self.last_frame_start + cur_frame.duration;
		self.cur_time += delta * cur_anim.time_scale;

		while self.cur_time > next_frame_time {
			self.last_frame_start = next_frame_time;
			self.last_anim_index += 1;
			let anim_len = cur_anim.frames.len();
			if self.last_anim_index >= anim_len {
				self.last_anim_index %= anim_len;
			}
			cur_frame = &self.spritesheet.frames[cur_anim.frames[self.last_anim_index]];
			next_frame_time += cur_frame.duration;
		}

		sprite.index = cur_frame.atlas_index;
		sprite.anchor = cur_frame.anchor.clone();
	}

}