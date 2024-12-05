mod app;
mod asset;
mod games;
#[cfg(feature = "inspector")]
mod inspector;

pub fn main() {
    use app::MyAppPlugin;
    use bevy::{asset::AssetMetaCheck, prelude::*};
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(MyAppPlugin)
        .run();
}
