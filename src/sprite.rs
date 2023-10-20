use bevy::{
	prelude::*, 
	sprite::Anchor
};

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

	pub fn img_handle(&self) -> Handle<Image> {
		self.img_handle.clone()
	}

	pub fn img_size(&self) -> Vec2 {
		self.img_size.clone()
	}

	pub fn get_atlas(&self) -> TextureAtlas {
		let mut atlas = TextureAtlas::new_empty(self.img_handle(), self.img_size());
		for frame in &self.frames {
			atlas.add_texture(frame.rect.clone());
		}
		atlas
	}
}