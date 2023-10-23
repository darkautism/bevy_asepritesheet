use bevy::prelude::*;
use crate::sprite::{self, AnimHandle, AnimEndAction};

/// A component used to animate a [`TextureAtlasSprite`], which contains a 
/// [`sprite::Sheet`] for data and reference about frames and animations
#[derive(Component)]
pub struct SpriteAnimator{
	pub time_scale: f32,
	spritesheet: sprite::Sheet,
	cur_time: f32,
	cur_anim: Option<sprite::AnimHandle>,
	last_frame_start: f32,
	last_anim_index: usize
}

// -----------------------------------------------------------------------------

#[allow(dead_code)]
impl SpriteAnimator {

	/// Create a sprite animator component from a given [`sprite::Sheet`]
	pub fn from_sheet(sheet: sprite::Sheet) -> Self {
		SpriteAnimator { 
			time_scale: 1.0,
			spritesheet: sheet,
			cur_time: 0.0, 
			cur_anim: None,
			last_frame_start: 0.0,
			last_anim_index: 0
		}
	}

	/// Get a reference to the [`sprite::Sheet`] used by this component
	pub fn spritesheet(&self) -> &sprite::Sheet { &self.spritesheet }

	/// Get a mutable reference to the [`sprite::Sheet`] used by this component
	pub fn spritesheet_mut(&mut self) -> &mut sprite::Sheet { 
		&mut self.spritesheet 
	}

	/// The current animation playtime elapsed since the animation was started
	pub fn cur_time(&self) -> f32 { self.cur_time }

	/// Set the current elapsed time in the animation
	pub fn set_cur_time(&mut self, seconds: f32) {
		
		if self.cur_anim.is_none() { return; }
		let cur_anim = self.spritesheet.get_anim(
			self.cur_anim.as_ref().unwrap()
		).unwrap();

		// some optomization in case the user inputs a high value, to avoid
		// looping through the animation multiple times in the next 
		// animation call
		let mut target_time = seconds;
		if self.cur_time > seconds {
			if cur_anim.end_action == AnimEndAction::Loop {
				let anim_time = cur_anim.total_time();
				let loop_cur_time = self.cur_time % anim_time;
				let loop_target_time = seconds % anim_time;
				let target_loops = (seconds / anim_time).floor();
				if loop_cur_time > loop_target_time {
					target_time = target_loops * anim_time + loop_target_time;
					self.last_frame_start += target_loops * anim_time;
				}
				else {
					self.reset_persistent_data();
				}
			} else {
				target_time = cur_anim.total_time();
				self.reset_persistent_data();
			}
		}
		self.cur_time = target_time;
	}

	/// A handle to the currently playing animation if there is one
	pub fn cur_anim(&self) -> &Option<sprite::AnimHandle> { &self.cur_anim }

	/// Start playing the specified animation if it exists
	/// 
	/// # Panics
	/// if the specified animation handle does not refer to an animation within
	/// the spritesheet
	pub fn set_anim(&mut self, anim: sprite::AnimHandle) {
		if self.spritesheet.get_anim(&anim).is_none() { 
			panic!("Specified animation does not exist in the spritesheet!"); 
		}
		self.reset_persistent_data();
		self.cur_anim = Some(anim);
	}

	/// Start playing the animation at the specified index
	pub fn set_anim_index(&mut self, anim_index: usize) {
		self.set_anim(AnimHandle::from_index(anim_index));
	}

	/// Stop playing the animation so the animator is not playing any animation
	pub fn stop_anim(&mut self){
		self.reset_persistent_data();
		self.cur_anim = None;
	}

	/// Play and apply the animation to the specified [`TextureAtlasSprite`]
	/// over the specified elapsed time (delta)
	pub fn animate(&mut self, sprite: &mut TextureAtlasSprite, delta: f32) {
		let cur_anim = 
			if let Some(val) = self.cur_anim.as_ref() { 
				if let Some(val2) = self.spritesheet.get_anim(val) { val2 } 
				else { return; }
			} 
			else { return; };

		let frames =  &self.spritesheet.frames;
		let anim_frame_indices = cur_anim.frame_indices();

		let mut cur_frame = &frames[anim_frame_indices[self.last_anim_index]];
		let mut next_frame_time = self.last_frame_start + cur_frame.duration;
		self.cur_time += delta * cur_anim.time_scale * self.time_scale;

		let mut anim_ended = false;
		while self.cur_time > next_frame_time {
			self.last_frame_start = next_frame_time;
			self.last_anim_index += 1;
			let anim_len = cur_anim.frame_indices().len();
			if self.last_anim_index >= anim_len {
				anim_ended = true;
				match cur_anim.end_action {
					AnimEndAction::Loop => {
						self.last_anim_index %= anim_len;
					},
					AnimEndAction::Pause | AnimEndAction::Stop => {
						self.cur_time = cur_anim.total_time();
						self.last_anim_index = anim_len - 1;
						cur_frame = &frames[
							anim_frame_indices[self.last_anim_index]
						];
						break;
					}
				}
			}
			cur_frame = &frames[anim_frame_indices[self.last_anim_index]];
			next_frame_time += cur_frame.duration;
		}

		sprite.index = cur_frame.atlas_index;
		sprite.anchor = cur_frame.anchor.clone();

		if anim_ended {
			match cur_anim.end_action {
				AnimEndAction::Pause => {
					self.time_scale = 0.0;
				},
				AnimEndAction::Stop => {
					self.stop_anim();
				},
				_ => { }
			}
		}
	}

	fn reset_persistent_data(&mut self) {
		self.last_anim_index = 0;
		self.last_frame_start = 0.0;
		self.cur_time = 0.0;
	}
}