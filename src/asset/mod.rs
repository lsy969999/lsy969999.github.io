use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use bevy_common_assets::ron::RonAssetPlugin;
use custom::CustomAsset;
use resource::{BaseAssets, DefaultSceneAssets, DungeonSceneAssets};
use system::{remove_default_res, remove_dungeon_res};

use crate::app::state::MyAppState;

pub mod custom;
pub mod event;
pub mod resource;
pub mod state;
pub mod system;
pub struct MyAssetPlugin;

impl Plugin for MyAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CustomAsset>::new(&["custom.ron"]));

        app.add_loading_state(
            LoadingState::new(MyAppState::BaseAssetLoading)
                .continue_to_state(MyAppState::DefaultSceneAssetLoading)
                .load_collection::<BaseAssets>(),
        )
        .add_loading_state(
            LoadingState::new(MyAppState::DefaultSceneAssetLoading)
                .continue_to_state(MyAppState::DefaultScene)
                .load_collection::<DefaultSceneAssets>(),
        )
        .add_loading_state(
            LoadingState::new(MyAppState::DungeonSceneAssetLoading)
                .continue_to_state(MyAppState::DungeonScene)
                .load_collection::<DungeonSceneAssets>(),
        );

        app.add_systems(
            OnEnter(MyAppState::DungeonSceneAssetLoading),
            remove_default_res,
        );
        app.add_systems(
            OnEnter(MyAppState::DefaultSceneAssetLoading),
            remove_dungeon_res,
        );
    }
}
