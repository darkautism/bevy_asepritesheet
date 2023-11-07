use bevy::{
	prelude::*, 
	asset::{
		AssetLoader,
		io::Reader, 
		AsyncReadExt
	}
};
use serde_json::from_slice;
use crate::{aseprite_data::SpritesheetData, sprite_animator};

// Struct Definitions: ---------------------------------------------------------

pub struct SpritesheetAssetPlugin {
	extensions: Vec<&'static str>
}

struct SpritesheetAssetLoader {
    extensions: Vec<&'static str>
}

// Struct Implementations: -----------------------------------------------------

impl Plugin for SpritesheetAssetPlugin {
	fn build(&self, app: &mut App) {
		app
			.register_asset_loader(SpritesheetAssetLoader { extensions: self.extensions.clone() })
			.init_asset::<SpritesheetData>()
			.add_event::<sprite_animator::AnimFinishEvent>()
        	.add_systems(PostUpdate, sprite_animator::animate_sprites)
		;
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
	
	type Asset = SpritesheetData;
	type Settings = ();
	type Error = std::io::Error;

	fn load<'a>(
		&'a self,
		reader: &'a mut Reader,
		settings: &'a Self::Settings,
		load_context: &'a mut bevy::asset::LoadContext,
	) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
		Box::pin(async move {
			let mut bytes = Vec::new();
			reader.read_to_end(&mut bytes).await?;
			let asset = from_slice::<SpritesheetData>(&bytes).expect("unable to decode sprite");
			Ok(asset)
		})
	}

	fn extensions(&self) -> &[&str] { &self.extensions }
}