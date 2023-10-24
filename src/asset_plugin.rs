use bevy::{
	prelude::*, 
	asset::{
		AssetLoader,
		LoadedAsset
	}
};
use serde_json::from_slice;
use crate::aseprite_data::SpritesheetData;

pub struct SpritesheetAssetPlugin {
	extensions: Vec<&'static str>
}

struct SpritesheetAssetLoader {
    extensions: Vec<&'static str>
}

// -----------------------------------------------------------------------------

impl Plugin for SpritesheetAssetPlugin {
	fn build(&self, app: &mut App) {
        app.add_asset::<SpritesheetData>().add_asset_loader(
			SpritesheetAssetLoader {
            	extensions: self.extensions.clone()
        	}
		);
	}

}

impl SpritesheetAssetPlugin {
	pub fn new(extensions: &[&'static str]) -> Self {
		SpritesheetAssetPlugin { 
			extensions: extensions.to_owned()
		}
	}
}

impl AssetLoader for SpritesheetAssetLoader {
	
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut bevy::asset::LoadContext,
	) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
			Box::pin(async move {
				let asset = from_slice::<SpritesheetData>(bytes)?;
				// TODO convert to sprite::sheet?
				load_context.set_default_asset(LoadedAsset::new(asset));
				Ok(())
			})
	}

	fn extensions(&self) -> &[&str] { &self.extensions }
}