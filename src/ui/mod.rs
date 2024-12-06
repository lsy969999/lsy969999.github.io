use asset_loading::{enter_base_loading_ui, enter_default_scene_loading_ui};
use bevy::prelude::*;
use default_scene::on_enter_default_scene;
use system::{despawn_under_root_ui, setup_ui_root};

use crate::app::state::MyAppState;

pub mod asset_loading;
pub mod component;
pub mod default_scene;
pub mod system;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_root);

        app.add_systems(OnEnter(MyAppState::BaseAssetLoading), enter_base_loading_ui);
        app.add_systems(OnExit(MyAppState::BaseAssetLoading), despawn_under_root_ui);

        app.add_systems(
            OnEnter(MyAppState::DefaultSceneAssetLoading),
            enter_default_scene_loading_ui,
        );
        app.add_systems(
            OnExit(MyAppState::DefaultSceneAssetLoading),
            despawn_under_root_ui,
        );

        app.add_systems(OnEnter(MyAppState::DefaultScene), on_enter_default_scene);
    }
}
