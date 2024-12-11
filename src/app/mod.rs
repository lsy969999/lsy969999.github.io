use crate::{
    asset::MyAssetPlugin,
    games::{default::MyDefaultGamePlugin, dungeon::MyDungeonGamePlugin},
    shader::MyShaderPlugin,
    ui::MyUiPlugin,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tweening::TweeningPlugin;
use state::MyAppState;
use system::{setup_camera_light, start_state};
pub mod component;
pub mod state;
mod system;

pub struct MyAppPlugin;

impl Plugin for MyAppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>();

        app.add_plugins(MeshPickingPlugin)
            .add_plugins(TweeningPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

        app.add_plugins(MyAssetPlugin)
            .add_plugins(MyUiPlugin)
            .add_plugins(MyShaderPlugin)
            .add_plugins(MyDefaultGamePlugin)
            .add_plugins(MyDungeonGamePlugin);

        app.add_systems(Startup, setup_camera_light);
        app.add_systems(Startup, start_state);
        #[cfg(feature = "inspector")]
        {
            use super::inspector::InspectorPlugin;
            app.add_plugins(InspectorPlugin)
                .add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}
