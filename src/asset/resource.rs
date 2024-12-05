use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct FoxAssets {
    #[asset(path = "models/Fox.glb")]
    pub fox: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct RequiredAssets {
    #[asset(path = "fonts/GalmuriMono11.ttf")]
    pub galmuri_mono11: Handle<Font>,
}
