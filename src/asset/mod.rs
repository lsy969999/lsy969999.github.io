use bevy::prelude::*;
// use bevy_asset_loader::loading_state::{
//     config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
// };

use custom::CustomAssetLoaderPlugin;
use system::{
    check_base_asset_loading, check_default_scene_asset_loading, on_enter_base_asset_loading,
    on_enter_default_secne_asset_loading,
};

use crate::app::state::MyAppState;

pub mod custom;
pub mod event;
pub mod resource;
pub mod state;
pub mod system;
pub struct MyAssetPlugin;

impl Plugin for MyAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CustomAssetLoaderPlugin);

        app.add_systems(
            OnEnter(MyAppState::BaseAssetLoading),
            on_enter_base_asset_loading,
        );
        app.add_systems(
            Update,
            check_base_asset_loading.run_if(in_state(MyAppState::BaseAssetLoading)),
        );

        app.add_systems(
            OnEnter(MyAppState::DefaultSceneAssetLoading),
            on_enter_default_secne_asset_loading,
        );
        app.add_systems(
            Update,
            check_default_scene_asset_loading
                .run_if(in_state(MyAppState::DefaultSceneAssetLoading)),
        );
    }
}
