use bevy::{asset::LoadState, prelude::*};

use crate::app::state::MyAppState;

use super::resource::{BaseAssets, BaseAssetsLoaded, DefaultSceneAssets, DefaultSceneAssetsLoaded};

pub fn on_enter_base_asset_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BaseAssets {
        font: asset_server.load("fonts/GalmuriMono11.ttf"),
    });
    commands.insert_resource(BaseAssetsLoaded {
        is_loaded_font: false,
    });
}
pub fn check_base_asset_loading(
    asset_server: Res<AssetServer>,
    base_assets: Res<BaseAssets>,
    mut base_assets_loadded: ResMut<BaseAssetsLoaded>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    match asset_server.get_load_state(base_assets.font.id()) {
        Some(LoadState::Loaded) => {
            if !base_assets_loadded.is_loaded_font {
                base_assets_loadded.is_loaded_font = true;
                next_state.set(MyAppState::DefaultSceneAssetLoading);
            }
        }
        _ => {}
    }
}

pub fn on_enter_default_secne_asset_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(DefaultSceneAssets {
        fox: asset_server.load("models/Fox.glb"),
    });
    commands.insert_resource(DefaultSceneAssetsLoaded {
        is_loaded_fox: false,
    });
}
pub fn check_default_scene_asset_loading(
    asset_server: Res<AssetServer>,
    default_scene_assets: Res<DefaultSceneAssets>,
    mut default_scene_assets_loadded: ResMut<DefaultSceneAssetsLoaded>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    match asset_server.get_load_state(default_scene_assets.fox.id()) {
        Some(LoadState::Loaded) => {
            if !default_scene_assets_loadded.is_loaded_fox {
                default_scene_assets_loadded.is_loaded_fox = true;
                next_state.set(MyAppState::DefaultScene);
            }
        }
        _ => {}
    }
}
