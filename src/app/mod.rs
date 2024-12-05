use crate::{asset::MyAssetPlugin, games::default::MyDefaultGamePlugin};
use bevy::prelude::*;
use system::setup_camera_light;

mod system;

pub struct MyAppPlugin;

impl Plugin for MyAppPlugin {
    fn build(&self, app: &mut App) {
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
