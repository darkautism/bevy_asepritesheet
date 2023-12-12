use bevy::prelude::*;

// Structs: -------------------------------------------------------------------

/// Allows basic control over all [`crate::prelude::SpriteAnimator`] components
#[derive(Resource)]
pub struct SpriteAnimController {
	/// whether or not the animators will animate automatically, if turned off, individual sprite
	/// animator components can still be manually called and animated
    pub is_active: bool,
	/// the global time scale that will affect all animators
    pub global_time_scale: f32,
}

// Implementations: -----------------------------------------------------------

impl Default for SpriteAnimController {
    fn default() -> Self {
        Self {
            is_active: true,
            global_time_scale: 1.0,
        }
    }
}
