use bevy::prelude::*;

#[derive(Resource)]
pub struct MyAssets {
    pub fox: Handle<Gltf>,
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct MyAssetsLoaded {
    pub is_loaded_fox: bool,
    pub is_loaded_font: bool,
}
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
}
#[derive(Resource)]
pub struct BaseAssetsLoaded {
    pub is_loaded_font: bool,
}
