mod app;
mod asset;
mod games;
#[cfg(feature = "inspector")]
mod inspector;
mod shader;
mod ui;

pub fn main() {
    use app::MyAppPlugin;
    use bevy::{asset::AssetMetaCheck, prelude::*};
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#target".to_string()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(MyAppPlugin)
        .run();
}
