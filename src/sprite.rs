use crate::aseprite_data;
use aseprite_data::SpritesheetData;
use bevy::{prelude::*, reflect::TypeUuid, sprite::Anchor};
use std::{ops::*, usize};

// Struct Definitions: ---------------------------------------------------------

/// A spritesheet object containing processed data from the deserialized
/// aseprite data. Used as reference data for the
/// [`crate::sprite_animator::SpriteAnimator`] component
#[derive(Asset, TypeUuid, TypePath, Default, Clone, Debug)]
#[uuid = "13361c8f-a7f0-4db8-8492-c3d5387ffa7b"]
pub struct Spritesheet {
    /// A set of every possible frame that can be used for an animation within
    /// the spritesheet
    pub frames: Vec<Frame>,
    anims: Vec<Anim>,
    img_handle: Handle<Image>,
    img_size: Vec2,
    atlas_handle: Option<Handle<TextureAtlas>>,
}

/// A parsed spritesheet animation that determines which sprite frames will be
/// drawn when active
#[derive(Clone, Debug)]
pub struct Anim {
    /// The name of the animation that can be used to find it with
    /// [`Sheet::get_anim_handle``]
    pub name: String,

    /// A speed multiplier for the animation play rate, normal rate is 1.0,
    /// 0.0 is completely paused, and 2.0 will play twice as fast
    pub time_scale: f32,

    /// How the animation behaves when it reaches the end
    pub end_action: AnimEndAction,

    /// A set of the individual frame indices in the sprite frame set
    frames_indices: Vec<usize>,

    /// The total length of the animation in seconds
    total_time: f32,
}

/// A handle for [`Anim`] that can be used as a reference to play specific
/// animations on a spritesheet
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimHandle {
    index: Option<usize>,
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
    rect: Rect, // TODO remove ?
}

/// Enum for setting different end behaviors of a sprite's animation,
/// default is [`AnimEndAction::Loop`]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnimEndAction {
    /// Stop the animation after completion, sets current animation to [`None`]
    Stop,
    // Pause the animator after completion
    Pause,
    // Loop through the animation, restarts from the beginning upon completion
    Loop,
    // After completion, play the next specified animation
    Next(AnimHandle),
}

// Struct Implementations: -----------------------------------------------------

#[allow(dead_code)]
impl Spritesheet {
    /// Create a new spritesheet object from the specified data, should
    /// generally not be used unless you are generating spritesheets entirely
    /// in code
    pub fn new(
        frames: Vec<Frame>,
        anims: Vec<Anim>,
        img_handle: Handle<Image>,
        img_size: Vec2,
    ) -> Self {
        Spritesheet {
            frames: frames,
            anims: anims,
            img_handle: img_handle,
            img_size: img_size,
            atlas_handle: None,
        }
    }

    /// Create a new spritesheet from given aseprite json data and a specified
    /// image asset. Use if the image path in the aseprite data does not
    /// properly point to the location of the image asset. NOTE: image paths
    /// are NOT relative to the json file, they are relative to the bevy asset
    /// directory
    pub fn from_data_image(
        data: &SpritesheetData,
        img_handle: Handle<Image>,
        frame_anchor: &Anchor,
        atlas_assets: &mut Assets<TextureAtlas>,
    ) -> Self {
        // construct and return a spritesheet from the data given
        let mut sheet = Spritesheet::default();
        sheet.copy_from(data, frame_anchor);
        sheet.img_handle = img_handle;

        // creat the atlas asset handle
        sheet.create_atlas_handle(atlas_assets);

        sheet
    }

    /// Create a spritesheet from the specified parsed aseprite json data. The
    /// image will be loaded from the specified path in the aseprite json data
    /// NOTE: image paths are NOT relative to the json file, they are relative
    /// to the bevy asset directory. If you need to specify a different image
    /// path, you can load the image asset manually and pass the handle to
    /// [`Sheet::from_data_image`] instead
    pub fn from_data(
        data: &SpritesheetData,
        asset_server: &Res<AssetServer>,
        frame_anchor: &Anchor,
        atlas_assets: &mut Assets<TextureAtlas>,
    ) -> Self {
        Spritesheet::from_data_image(
            data,
            asset_server.load(&data.meta.image),
            frame_anchor,
            atlas_assets,
        )
    }

    /// copy all the data from the specified spritesheet data into self
    pub fn copy_from(&mut self, data: &SpritesheetData, frame_anchor: &Anchor) {
        // populate create a frames vec to store all frames in sprite data
        let mut frames = Vec::<Frame>::new();
        for (i, frame_data) in data.frames.iter().enumerate() {
            // get frame offset from original frame top left corner
            // (only relevant if frames were trimmed by aseprite in the export)
            let frame_offset = Vec2::new(
                frame_data.sprite_source_size.x as f32,
                frame_data.sprite_source_size.y as f32,
            );

            // get size of the new trimmed frame
            let trimmed_frame_size =
                Vec2::new(frame_data.frame.w as f32, frame_data.frame.h as f32);

            // calculate the new sprite anchor based on how much the frame was
            // trimmed and moved by aseprite
            let anchor_target = frame_anchor
                .as_vec()
                // offset by 0.5 since bevy considers <-0.5,-0.5> to be top left
                // for some reason
                .add(Vec2::splat(0.5))
                // get original frame size data
                .mul(Vec2::from(frame_data.source_size))
                // correct for frame offset
                .sub(frame_offset)
                // scale anchor pos to new trimmed size
                .div(trimmed_frame_size)
                // reset the 0.5 offset post-move
                .sub(Vec2::splat(0.5))
                // invert y offset since aseprite and bevy use differnt coord
                // systems
                .mul(Vec2::new(1.0, -1.0));

            // construct frame container from calculated data
            let frame = Frame {
                atlas_index: i,
                duration: frame_data.duration as f32 * 0.001,
                anchor: Anchor::Custom(anchor_target),
                rect: frame_data.frame.into(),
            };

            // add frame to collection
            frames.push(frame);
        }

        // create and populate a vec for all the sprite animations
        let mut anims = Vec::<Anim>::new();
        for tag_data in &data.meta.frame_tags {
            // construct animation container from data
            let mut anim = Anim {
                name: tag_data.name.clone(),
                frames_indices: (tag_data.from..=tag_data.to).collect(),
                time_scale: 1.0,
                end_action: AnimEndAction::Loop,
                total_time: 0.0,
            };

            // calculate total animation time and add to anim collection
            anim.calculate_total_time(&frames);
            anims.push(anim);
        }

        self.frames = frames;
        self.anims = anims;
        self.img_size = data.meta.size.into();
    }

    /// copy all the data from the specified spritesheet data into self + load and use the image
    /// specified in the spritesheet data
    pub fn copy_from_with_image(
        &mut self,
        data: &SpritesheetData,
        frame_anchor: &Anchor,
        asset_server: &Res<AssetServer>,
    ) {
        self.copy_from(data, frame_anchor);
        self.img_handle = asset_server.load(&data.meta.image);
    }

    /// Get the image handle that the spritesheet is using
    pub fn img_handle(&self) -> Handle<Image> {
        self.img_handle.clone()
    }

    /// Get the dimensions of the image asset being used by the spritesheet
    pub fn img_size(&self) -> Vec2 {
        self.img_size.clone()
    }

    /// Get the total amount of animations that the spritesheet contains
    pub fn anim_count(&self) -> usize {
        self.anims.len()
    }

    /// Get a reference to the texture atlas being used for the spritesheet
    pub fn atlas_handle(&self) -> Option<Handle<TextureAtlas>> {
        self.atlas_handle.clone()
    }

    /// Create a texture atlas with each sprite frame on it if there isn't one
    /// already and return a handle to the atlas for referencing it later
    pub fn create_atlas_handle(
        &mut self,
        atlas_assets: &mut Assets<TextureAtlas>,
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

    /// Get a handle to the animation with the specified name, if it exists
    pub fn get_anim_handle<T: AsRef<str>>(&self, name: T) -> AnimHandle {
        for (i, anim) in self.anims.iter().enumerate() {
            if anim.name == name.as_ref() {
                return AnimHandle::from_index(i);
            }
        }
        AnimHandle::invalid()
    }

    /// Get a reference to the specified animation, if it exists
    pub fn get_anim(&self, handle: &AnimHandle) -> Result<&Anim, ()> {
        if !handle.is_valid() {
            return Err(());
        }
        let index = handle.index.unwrap();
        if self.anims.len() > index {
            Ok(&self.anims[index])
        } else {
            Err(())
        }
    }

    /// Get a mutable reference to the specified animation, if it exists
    pub fn get_anim_mut(&mut self, handle: &AnimHandle) -> Result<&mut Anim, ()> {
        if !handle.is_valid() {
            return Err(());
        }
        let index = handle.index.unwrap();
        if self.anims.len() > index {
            Ok(&mut self.anims[index])
        } else {
            Err(())
        }
    }
}

#[allow(dead_code)]
impl Anim {
    /// The set of indices referring to all the frames in the spritesheet that
    /// the animation consists of
    pub fn frame_indices(&self) -> &Vec<usize> {
        &self.frames_indices
    }

    /// The total amount of time that it takes to play the animation, in seconds
    /// NOTE: does not take into account time_scale
    pub fn total_time(&self) -> f32 {
        self.total_time
    }

    fn calculate_total_time(&mut self, frames: &Vec<Frame>) {
        let mut time = 0.0;
        for frame_index in &self.frames_indices {
            time += frames[*frame_index].duration;
        }
        self.total_time = time;
    }
}

#[allow(dead_code)]
impl AnimHandle {
    /// Create an animation handle that refers to an animation of the specified
    /// index on any spritesheet
    pub fn from_index(index: usize) -> Self {
        AnimHandle { index: Some(index) }
    }

    /// Create an invalid handle
    pub fn invalid() -> Self {
        AnimHandle { index: None }
    }

    /// Whether or not the handle is valid. NOTE just because it's valid does not mean that the
    /// animation exists when it's used to retreive an animation
    pub fn is_valid(&self) -> bool {
        self.index.is_some()
    }
}
