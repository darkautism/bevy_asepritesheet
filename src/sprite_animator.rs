use bevy::prelude::*;
use crate::sprite::*;

// Struct Definitions: ---------------------------------------------------------

/// A component used to animate a [`TextureAtlasSprite`], which contains a 
/// [`sprite::Sheet`] for data and reference about frames and animations
#[derive(Component)]
pub struct SpriteAnimator {
	pub time_scale: f32,
	spritesheet: Option<Spritesheet>,
	cur_time: f32,
	cur_anim: Option<AnimHandle>,
	last_frame_start: f32,
	last_anim_index: usize
}

#[derive(Bundle, Default)]
pub struct AnimatedSpriteBundle {
	pub sprite: SpriteSheetBundle,
	pub animator: SpriteAnimator
}

#[derive(Event, Debug)]
pub struct AnimFinishEvent {
	pub entity: Entity,
	pub anim: AnimHandle
}

// Struct Implementations: -----------------------------------------------------

impl Default for SpriteAnimator {
	fn default() -> Self {
		Self {
			time_scale: 1.0,
			spritesheet: default(),
			cur_time: default(),
			cur_anim: default(),
			last_frame_start: default(),
			last_anim_index: default()
		}
	}
}

impl SpriteAnimator {

	/// Create a sprite animator with the specified time scale (default is 1.0, 0.5 is half 
	/// speed, 2.0 is 2x fast forwarded animations, etc)
	pub fn new(time_scale: f32) -> Self {
		Self {
			time_scale,
			spritesheet: default(),
			cur_time: default(),
			cur_anim: default(),
			last_frame_start: default(),
			last_anim_index: default()
		}
	}

	/// Create a sprite animator component from a given [`sprite::Sheet`]
	pub fn from_sheet(sheet: Spritesheet) -> Self {
		SpriteAnimator { 
			time_scale: 1.0,
			spritesheet: Some(sheet),
			cur_time: 0.0, 
			cur_anim: None,
			last_frame_start: 0.0,
			last_anim_index: 0
		}
	}

	/// Get a reference to the [`sprite::Sheet`] used by this component
	pub fn spritesheet(&self) -> Option<&Spritesheet> { 
		if let Some(sheet) = &self.spritesheet {
			Some(sheet)
		} else {
			None
		}
	}

	/// Get a mutable reference to the [`sprite::Sheet`] used by this component
	pub fn spritesheet_mut(&mut self) -> Option<&mut Spritesheet> {
		if let Some(sheet) = &mut self.spritesheet {
			Some(sheet)
		} else {
			None 
		}
	}

	/// The current animation playtime elapsed since the animation was started
	pub fn cur_time(&self) -> f32 { self.cur_time }

	/// The elapsed time in the current animation normalized from 0 to 1, 0
	/// meaning none of the animation has played, while 1 means the entire 
	/// animation has played
	pub fn cur_time_normalized(&self) -> f32 {
		
		// return if no sheet
		let sheet = if let Some(val) = &self.spritesheet {
			val
		} else {
			return 0.0;
		};

		// return 0 if no animation
		let cur_anim = 
			if let Some(handle) = self.cur_anim.as_ref() { 
				if let Ok(val) = sheet.get_anim(handle) { val } 
				else { return 0.0; }
			} 
			else { return 0.0; };

		self.cur_time / cur_anim.total_time()
	}

	/// Set the current elapsed time in the animation
	pub fn set_cur_time(&mut self, seconds: f32) {
		
		// return if no sheet
		let sheet = if let Some(val) = &self.spritesheet {
			val
		} else {
			return;
		};

		// return if no anim
		if self.cur_anim.is_none() { return; }
		let cur_anim = sheet.get_anim(
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
				if loop_cur_time <= loop_target_time {
					self.last_frame_start = 
						target_loops * 
						anim_time + 
						loop_cur_time
					;
				}
				else {
					if target_loops > 0.0 {
						self.last_frame_start = target_loops * anim_time;
						self.last_anim_index = 0;
					}
					else {
						self.reset_persistent_data();
					}
				}
			} else {
				target_time = cur_anim.total_time();
				self.last_anim_index = sheet.anim_count() - 1;
				self.last_frame_start = 
					target_time - 
					sheet.frames[self.last_anim_index].duration
				;
			}
		}
		self.cur_time = target_time;
	}

	/// Set the amount of time elapsed in the current animation, 0 being none,
	/// and 1 being the full length of the animation
	pub fn set_cur_time_normalized(&mut self, time_normalized: f32) {

		// return if no sheet
		let sheet = if let Some(val) = &self.spritesheet {
			val
		} else {
			return;
		};

		// return if no anim
		if self.cur_anim.is_none() { return; }
		let cur_anim = sheet.get_anim(
			self.cur_anim.as_ref().unwrap()
		).unwrap();

		// calculate the non-normalized time and apply it
		let target_time = time_normalized * cur_anim.total_time();
		self.set_cur_time(target_time);
	}

	/// A handle to the currently playing animation if there is one
	pub fn cur_anim(&self) -> &Option<AnimHandle> { &self.cur_anim }

	/// Start playing the specified animation if it exists, otherwise returns empty error
	pub fn set_anim(&mut self, anim: AnimHandle) -> Result<(), ()> {
		
		// return err if no sheet
		let sheet = if let Some(val) = &self.spritesheet {
			val
		} else {
			return Err(());
		};

		if sheet.get_anim(&anim).is_err() { 
			return Err(());
		}
		self.reset_persistent_data();
		self.cur_anim = Some(anim);
		Ok(())
	}

	/// Start playing the animation at the specified index
	pub fn set_anim_index(&mut self, anim_index: usize) -> Result<(), ()> {
		self.set_anim(AnimHandle::from_index(anim_index))?;
		Ok(())
	}

	/// Stop playing the animation so the animator is not playing any animation
	pub fn stop_anim(&mut self){
		self.reset_persistent_data();
		self.cur_anim = None;
	}

	/// Play and apply the animation to the specified [`TextureAtlasSprite`]
	/// over the specified elapsed time (delta)
	pub fn animate(
		&mut self, 
		self_entity: &Entity,
		sprite: &mut TextureAtlasSprite,
		events: &mut EventWriter<AnimFinishEvent>,
		delta: f32
	) {
		// return if no sheet
		let sheet = if let Some(val) = &self.spritesheet {
			val
		} else {
			return;
		};

		// return if no animation is playing
		let cur_anim = 
			if let Some(val) = self.cur_anim.as_ref() { 
				if let Ok(val2) = sheet.get_anim(val) { val2 } 
				else { return; }
			} 
			else { return; };

		let frames =  &sheet.frames;
		let anim_frame_indices = cur_anim.frame_indices();

		let mut cur_frame = &frames[anim_frame_indices[self.last_anim_index]];
		let mut next_frame_time = self.last_frame_start + cur_frame.duration;
		self.cur_time += delta * cur_anim.time_scale * self.time_scale;

		// increment the frame if current time has elapsed the current frame's
		// duration
		let mut anim_ended = false;
		while self.cur_time > next_frame_time {

			self.last_frame_start = next_frame_time;
			self.last_anim_index += 1;
			let anim_len = cur_anim.frame_indices().len();

			// check if the animation has ended and set a flag if so
			if self.last_anim_index >= anim_len {
				anim_ended = true;
				match cur_anim.end_action {
					AnimEndAction::Loop => {
						self.last_anim_index %= anim_len;
					},
					AnimEndAction::Pause | 
					AnimEndAction::Stop |
					AnimEndAction::Next(_) => {
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

		// apply the new sprite and anchor in the texture atlas
		sprite.index = cur_frame.atlas_index;
		sprite.anchor = cur_frame.anchor.clone();

		// behave according to the sprite end action if the animation ended
		if anim_ended {

			// send an event letting the program know the animation finished
			events.send(AnimFinishEvent { 
				entity: *self_entity,
				anim: *self.cur_anim.as_ref().unwrap()
			});

			// act according to end action type
			match cur_anim.end_action {
				AnimEndAction::Pause => {
					self.time_scale = 0.0;
				},
				AnimEndAction::Stop => {
					self.stop_anim();
				},
				AnimEndAction::Next(anim) => {
					self.set_anim(anim).expect(
						"ERROR: Failed to set specified animation"
					);
				}
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

// Systems: --------------------------------------------------------------------

pub fn animate_sprites(
	time: Res<Time>,
	mut events: EventWriter<AnimFinishEvent>,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut SpriteAnimator)>
) {
    for (entity, mut sprite, mut sprite_animator) in &mut query {
        if sprite_animator.cur_anim().is_none() {
            sprite_animator.set_anim_index(1).expect("ERROR: Invalid anim");
        }
        sprite_animator.animate(
			&entity, 
			&mut sprite, 
			&mut events, 
			time.delta_seconds()
		);
    }
}