use std::marker::PhantomData;

use asset::{CustomAsset, TestAsset};
use bevy::{asset::AssetLoader, prelude::*};
use serde::de::DeserializeOwned;
use thiserror::Error;
pub mod asset;
pub struct CustomAssetLoaderPlugin;

impl Plugin for CustomAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CustomAsset>()
            .init_asset_loader::<RonAssetLoader<CustomAsset>>()
            .init_asset::<TestAsset>()
            .init_asset_loader::<RonAssetLoader<TestAsset>>();
    }
}

struct RonAssetLoader<T: Asset> {
    _marker: PhantomData<T>, // 제네릭 필드 대체
}

#[derive(Debug, Error)]
enum RonAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl<T> AssetLoader for RonAssetLoader<T>
where
    T: Asset + DeserializeOwned,
{
    type Asset = T;
    type Settings = ();
    type Error = RonAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<T>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

impl<T> Default for RonAssetLoader<T>
where
    T: Asset + DeserializeOwned,
{
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
