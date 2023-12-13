pub mod animator;
pub mod aseprite_data;
pub mod assets;
pub mod core;
pub mod sprite;

/// general purpose crate to re-export common types as a shortcut
pub mod prelude {
    pub use crate::animator::{
        AnimEventSender, AnimFinishEvent, AnimatedSpriteBundle, SpriteAnimator,
    };
    pub use crate::aseprite_data::SpritesheetData;
    pub use crate::core::{
        load_spritesheet, load_spritesheet_then, AsepritesheetPlugin, SpriteAnimController,
        SpritesheetLoadedEvent,
    };
    pub use crate::sprite::{AnimEndAction, AnimHandle, Spritesheet};
}
