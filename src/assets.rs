use crate::aseprite_data::SpritesheetData;
use bevy::asset::{io::Reader, AssetLoader, AsyncReadExt};
use serde_json::from_slice;

// Struct Definitions: ---------------------------------------------------------

pub(crate) struct SpritesheetAssetLoader {
    pub extensions: Vec<&'static str>,
}

// Struct Implementations: -----------------------------------------------------

impl AssetLoader for SpritesheetAssetLoader {
    type Asset = SpritesheetData;
    type Settings = ();
    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = from_slice::<SpritesheetData>(&bytes).expect("unable to decode sprite");
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
