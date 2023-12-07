pub mod asset_plugin;
pub mod aseprite_data;
pub mod sprite;
pub mod sprite_animator;

/// general purpose crate to re-export common types as a shortcut
pub mod prelude {
	pub use crate::asset_plugin::SpritesheetAssetPlugin;
	pub use crate::aseprite_data::SpritesheetData;
	pub use crate::sprite::{
		Spritesheet,
		AnimHandle,
	};
	pub use crate::sprite_animator:: {
		AnimatedSpriteBundle,
		SpriteAnimator,
		AnimFinishEvent,
	};
}