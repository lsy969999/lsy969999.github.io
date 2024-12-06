use bevy::prelude::*;
// use bevy_asset_loader::loading_state::{
//     config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
// };
use event::{FontLoaded, FoxLoaded};

use system::{asset_load, on_asset_font_load_state, on_asset_fox_load_state};

pub mod event;
pub mod resource;
pub mod state;
pub mod system;
pub struct MyAssetPlugin;

impl Plugin for MyAssetPlugin {
    fn build(&self, app: &mut App) {
        // app.init_state::<FoxAssetsLoadState>();
        // app.init_state::<RequiredAssetsLoadState>();
        // app.add_loading_state(
        //     LoadingState::new(FoxAssetsLoadState::Loading)
        //         .continue_to_state(FoxAssetsLoadState::LoadEnd)
        //         .load_collection::<FoxAssets>(),
        // )
        // .add_loading_state(
        //     LoadingState::new(RequiredAssetsLoadState::Loading)
        //         .continue_to_state(RequiredAssetsLoadState::LoadEnd)
        //         .load_collection::<RequiredAssets>(),
        // );

        app.add_event::<FontLoaded>();
        app.add_event::<FoxLoaded>();
        app.add_systems(Startup, asset_load);
        app.add_systems(Update, (on_asset_fox_load_state, on_asset_font_load_state));
    }
}
