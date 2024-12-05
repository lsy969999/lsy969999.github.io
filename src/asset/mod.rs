use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use resource::{FoxAssets, RequiredAssets};
use state::{FoxAssetsLoadState, RequiredAssetsLoadState};

pub mod resource;
pub mod state;

pub struct MyAssetPlugin;

impl Plugin for MyAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<FoxAssetsLoadState>();
        app.init_state::<RequiredAssetsLoadState>();
        app.add_loading_state(
            LoadingState::new(FoxAssetsLoadState::Loading)
                .continue_to_state(FoxAssetsLoadState::LoadEnd)
                .load_collection::<FoxAssets>(),
        )
        .add_loading_state(
            LoadingState::new(RequiredAssetsLoadState::Loading)
                .continue_to_state(RequiredAssetsLoadState::LoadEnd)
                .load_collection::<RequiredAssets>(),
        );
    }
}
