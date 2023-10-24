use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use crate::aseprite_data::SpritesheetData;

pub struct SpritesheetAssetPlugin {
	file_extensions: Vec<&'static str>
}

// -----------------------------------------------------------------------------

impl Plugin for SpritesheetAssetPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(
			JsonAssetPlugin::<SpritesheetData>::new(&self.file_extensions)
        );
	}

}

impl SpritesheetAssetPlugin {
	pub fn new(file_extensions: &[&'static str]) -> Self {
		SpritesheetAssetPlugin { 
			file_extensions: file_extensions.to_owned()
		}
	}
}