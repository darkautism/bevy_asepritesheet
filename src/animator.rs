use crate::{core::SpriteAnimController, sprite::*};
use bevy::{prelude::*, sprite::Anchor};

// Struct Definitions: ---------------------------------------------------------

/// Specifies a specific time within a aprite animation
#[derive(Clone, Copy, PartialEq)]
pub enum AnimTimestamp {
    /// exactly how many seconds have passed since the start
    Seconds(f32),
    /// a percentage (from 0 to 1) of how complete the animation is at the specified timestamp.
    /// a value of 0 will always be the beginning and a value of 1 will always be the very end of
    /// the animation, no matter how long or short it is
    Normalized(f32),
}

/// A component used to animate a [`TextureAtlasSprite`], which contains a
/// [`sprite::Sheet`] for data and reference about frames and animations
#[derive(Component, Clone)]
pub struct SpriteAnimator {
    pub time_scale: f32,
    cur_time: f32,
    cur_anim: Option<AnimHandle>,
    last_frame_start: f32,
    last_anim_index: usize,
    target_time: Option<AnimTimestamp>,
}

#[derive(Bundle, Default, Clone)]
pub struct AnimatedSpriteBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub spritesheet: Handle<Spritesheet>,
    pub animator: SpriteAnimator,
}

#[derive(Component, Clone, Copy)]
pub struct AnimEventSender;

#[derive(Event, Debug, Clone)]
pub struct AnimFinishEvent {
    pub entity: Entity,
    pub anim: AnimHandle,
}

// Struct Implementations: -----------------------------------------------------

impl Default for SpriteAnimator {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            cur_time: default(),
            cur_anim: default(),
            last_frame_start: default(),
            last_anim_index: default(),
            target_time: default(),
        }
    }
}

impl SpriteAnimator {
    /// Create a sprite animator with the specified time scale (default is 1.0, 0.5 is half
    /// speed, 2.0 is 2x fast forwarded animations, etc)
    pub fn new(time_scale: f32) -> Self {
        Self {
            time_scale,
            cur_time: default(),
            cur_anim: default(),
            last_frame_start: default(),
            last_anim_index: default(),
            target_time: default(),
        }
    }

    /// Create a sprite animator from the specified animation
    pub fn from_anim(anim_handle: AnimHandle) -> Self {
        Self {
            cur_anim: Some(anim_handle),
            ..Default::default()
        }
    }

    /// The current animation playtime elapsed since the animation was started
    pub fn cur_time(&self) -> f32 {
        self.cur_time
    }

    /// The elapsed time in the current animation normalized from 0 to 1, 0
    /// meaning none of the animation has played, while 1 means the entire
    /// animation has played
    pub fn get_cur_time_normalized(&self, sheet: &Spritesheet) -> f32 {
        // return 0 if no animation
        let cur_anim = if let Some(handle) = self.cur_anim.as_ref() {
            if let Ok(val) = sheet.get_anim(handle) {
                val
            } else {
                return 0.0;
            }
        } else {
            return 0.0;
        };

        self.cur_time / cur_anim.total_time()
    }

    /// Set the current elapsed time of the currently playing animation
    pub fn set_cur_time(&mut self, time: AnimTimestamp) {
        self.target_time = Some(time);
    }

    /// Set the current elapsed time in the animation
    fn set_cur_time_seconds(&mut self, seconds: f32, sheet: &Spritesheet) {
        // return if no anim
        let cur_anim = sheet.get_anim(self.cur_anim.as_ref().unwrap()).unwrap();

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
                    self.last_frame_start = target_loops * anim_time + loop_cur_time;
                } else {
                    if target_loops > 0.0 {
                        self.last_frame_start = target_loops * anim_time;
                        self.last_anim_index = 0;
                    } else {
                        self.reset_persistent_data();
                    }
                }
            } else {
                target_time = cur_anim.total_time();
                self.last_anim_index = sheet.anim_count() - 1;
                self.last_frame_start = target_time - sheet.frames[self.last_anim_index].duration;
            }
        }
        self.cur_time = target_time;
    }

    /// Set the amount of time elapsed in the current animation, 0 being none,
    /// and 1 being the full length of the animation
    fn set_cur_time_normalized(&mut self, time_normalized: f32, sheet: &Spritesheet) {
        // return if no anim
        let cur_anim = sheet.get_anim(self.cur_anim.as_ref().unwrap()).unwrap();

        // calculate the non-normalized time and apply it
        let target_time = time_normalized * cur_anim.total_time();
        self.set_cur_time_seconds(target_time, sheet);
    }

    /// A handle to the currently playing animation if there is one
    pub fn cur_anim(&self) -> &Option<AnimHandle> {
        &self.cur_anim
    }

    /// Check to see if the current animation matches the specified animation
    pub fn is_cur_anim(&self, handle: AnimHandle) -> bool {
        if let Some(cur_anim) = self.cur_anim {
            cur_anim == handle
        } else {
            false
        }
    }

    /// Start playing the specified animation and returns true if it exists, else returns false
    pub fn set_anim(&mut self, anim: AnimHandle) {
        if let Some(cur_anim) = self.cur_anim {
            if cur_anim != anim {
                self.restart_anim();
            }
        }
        self.cur_anim = Some(anim);
    }

    /// Start playing the animation at the specified index
    pub fn set_anim_index(&mut self, anim_index: usize) {
        self.set_anim(AnimHandle::from_index(anim_index))
    }

    /// Stop playing the animation so the animator is not playing any animation
    pub fn stop_anim(&mut self) {
        self.reset_persistent_data();
        self.cur_anim = None;
    }

    /// Restart the curent animation from the beginning
    pub fn restart_anim(&mut self) {
        self.last_anim_index = 0;
        self.last_frame_start = 0.0;
        self.cur_time = 0.0;
    }

    /// Play and apply the animation to the specified [`TextureAtlasSprite`] over the specified
    /// elapsed time (delta)
    pub fn animate(
        &mut self,
        delta: f32,
        self_entity: &Entity,
        sheet: &Spritesheet,
        img_handle: &mut Handle<Image>,
        sprite: &mut Sprite,
        atlas: &mut TextureAtlas,
        maybe_evts: Option<&mut EventWriter<AnimFinishEvent>>,
    ) {
        // ensure the image handle matches the sprite
        if *img_handle != sheet.img_handle() {
            *img_handle = sheet.img_handle();
        }
        
        // return if no animation is playing
        let cur_anim = if let Some(val) = self.cur_anim.as_ref() {
            if let Ok(val2) = sheet.get_anim(val) {
                val2
            } else {
                warn!("Invalid animation for spritesheet!");
                self.cur_anim = None;
                return;
            }
        } else {
            return;
        };

        let frames = &sheet.frames;
        let anim_frame_indices = cur_anim.frame_indices();

        let mut cur_frame = &frames[anim_frame_indices[self.last_anim_index]];
        let mut next_frame_time = self.last_frame_start + cur_frame.duration;

        if let Some(target_time) = self.target_time.take() {
            match target_time {
                AnimTimestamp::Seconds(secs) => {
                    self.set_cur_time_seconds(secs, sheet);
                }
                AnimTimestamp::Normalized(val) => {
                    self.set_cur_time_normalized(val, sheet);
                }
            }
        } else {
            self.cur_time += delta * cur_anim.time_scale * self.time_scale;
        }

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
                    }
                    AnimEndAction::Pause | AnimEndAction::Stop | AnimEndAction::Next(_) => {
                        self.cur_time = cur_anim.total_time();
                        self.last_anim_index = anim_len - 1;
                        cur_frame = &frames[anim_frame_indices[self.last_anim_index]];
                        break;
                    }
                }
            }

            cur_frame = &frames[anim_frame_indices[self.last_anim_index]];
            next_frame_time += cur_frame.duration;
        }

        // apply the new sprite and anchor in the texture atlas
        atlas.index = cur_frame.atlas_index;
        sprite.anchor = cloned_flipped_anchor(cur_frame.anchor, sprite.flip_x, sprite.flip_y);

        // behave according to the sprite end action if the animation ended
        if anim_ended {
            // send an event letting the program know the animation finished
            if let Some(evts) = maybe_evts {
                evts.send(AnimFinishEvent {
                    entity: *self_entity,
                    anim: *self.cur_anim.as_ref().unwrap(),
                });
            }

            // act according to end action type
            match cur_anim.end_action {
                AnimEndAction::Pause => {
                    self.time_scale = 0.0;
                }
                AnimEndAction::Stop => {
                    self.stop_anim();
                }
                AnimEndAction::Next(anim) => {
                    self.set_anim(anim);
                }
                _ => {}
            }
        }
    }

    fn reset_persistent_data(&mut self) {
        self.restart_anim();
    }
}

// Systems: --------------------------------------------------------------------

/// system that runs in the [`PostUpdate`] schedule to update all the animated spritesheets in the
/// ecs world, some paramaters can be tweaked globally with the [`SpriteAnimController`] resource
pub fn animate_sprites(
    time: Res<Time>,
    spritesheet_assets: Res<Assets<Spritesheet>>,
    anim_controller: Res<SpriteAnimController>,
    mut events: EventWriter<AnimFinishEvent>,
    mut query: Query<(
        Entity,
        &mut Sprite,
        &mut TextureAtlas,
        &mut SpriteAnimator,
        &Handle<Spritesheet>,
        &mut Handle<Image>,
        Option<&AnimEventSender>,
    )>,
) {
    if !anim_controller.is_active {
        return;
    }
    let time_scale = anim_controller.global_time_scale;
    for (
        entity,
        mut sprite,
        mut atlas,
        mut sprite_animator,
        sheet_handle,
        mut img_handle,
        maybe_evt_send,
    ) in &mut query
    {
        if let Some(sheet) = spritesheet_assets.get(sheet_handle) {
            // ensure the animator is using the correct texture atlas from the entity
            if let Some(sheet_atlas_handle) = sheet.atlas_handle() {
                if sheet_atlas_handle != atlas.layout {
                    atlas.layout = sheet_atlas_handle.clone();
                }
            }
            // only pass in the event writer if the entity has the event sender component
            let maybe_evts = if maybe_evt_send.is_some() {
                Some(&mut events)
            } else {
                None
            };
            sprite_animator.animate(
                time.delta_seconds() * time_scale,
                &entity,
                sheet,
                &mut img_handle,
                &mut sprite,
                &mut atlas,
                maybe_evts,
            );
        }
    }
}

// Utility: --------------------------------------------------------------------

pub fn cloned_flipped_anchor(anchor: Anchor, flip_x: bool, flip_y: bool) -> Anchor {
    match anchor {
        Anchor::TopCenter => {
            if flip_y {
                Anchor::BottomCenter
            } else {
                anchor
            }
        }
        Anchor::BottomCenter => {
            if flip_y {
                Anchor::TopCenter
            } else {
                anchor
            }
        }
        Anchor::CenterLeft => {
            if flip_x {
                Anchor::CenterRight
            } else {
                anchor
            }
        }
        Anchor::CenterRight => {
            if flip_x {
                Anchor::CenterLeft
            } else {
                anchor
            }
        }
        Anchor::TopLeft => {
            if flip_x {
                if flip_y {
                    Anchor::BottomRight
                } else {
                    Anchor::TopRight
                }
            } else if flip_y {
                Anchor::BottomLeft
            } else {
                anchor
            }
        }
        Anchor::TopRight => {
            if flip_x {
                if flip_y {
                    Anchor::BottomLeft
                } else {
                    Anchor::TopLeft
                }
            } else if flip_y {
                Anchor::BottomRight
            } else {
                anchor
            }
        }
        Anchor::BottomRight => {
            if flip_x {
                if flip_y {
                    Anchor::TopLeft
                } else {
                    Anchor::BottomLeft
                }
            } else if flip_y {
                Anchor::TopRight
            } else {
                anchor
            }
        }
        Anchor::BottomLeft => {
            if flip_x {
                if flip_y {
                    Anchor::TopRight
                } else {
                    Anchor::BottomRight
                }
            } else if flip_y {
                Anchor::TopRight
            } else {
                anchor
            }
        }
        Anchor::Custom(mut off) => {
            if flip_x {
                off.x *= -1.0;
            }
            if flip_y {
                off.y *= -1.0;
            }
            Anchor::Custom(off)
        }
        _ => anchor,
    }
}
