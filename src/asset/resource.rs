use bevy::prelude::*;

use super::custom::asset::{CustomAsset, TestAsset};

#[derive(Resource)]
pub struct DefaultSceneAssets {
    pub fox: Handle<Gltf>,
}
#[derive(Resource)]
pub struct DefaultSceneAssetsLoaded {
    pub is_loaded_fox: bool,
}

#[derive(Resource)]
pub struct BaseAssets {
    pub font: Handle<Font>,
    pub custom: Handle<CustomAsset>,
    pub test: Handle<TestAsset>,
}
#[derive(Resource)]
pub struct BaseAssetsLoaded {
    pub is_loaded_font: bool,
    pub is_loaded_custom: bool,
    pub is_loadedd_test: bool,
}
