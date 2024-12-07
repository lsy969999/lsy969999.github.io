use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use super::custom::CustomAsset;

#[derive(AssetCollection, Resource)]
pub struct DefaultSceneAssets {
    #[asset(path = "models/Fox.glb")]
    pub fox: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct DungeonSceneAssets {
    #[asset(path = "models/Fox.glb")]
    pub fox: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct BaseAssets {
    #[asset(path = "fonts/GalmuriMono11.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "ron/some.custom.ron")]
    pub custom: Handle<CustomAsset>,
}
