use crate::{asset::MyAssetPlugin, games::default::MyDefaultGamePlugin};
use bevy::prelude::*;
use state::MyAppState;
use system::setup_camera_light;

mod state;
mod system;

pub struct MyAppPlugin;

impl Plugin for MyAppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MyAppState>();
        app.add_plugins(MyAssetPlugin)
            .add_plugins(MyDefaultGamePlugin);
        app.add_systems(Startup, setup_camera_light);
        #[cfg(feature = "inspector")]
        {
            use super::inspector::InspectorPlugin;
            app.add_plugins(InspectorPlugin);
        }
    }
}
