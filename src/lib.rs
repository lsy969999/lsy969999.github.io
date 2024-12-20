use bevy::window::{WindowMode, WindowResolution};

mod app;
mod asset;
mod games;
#[cfg(feature = "inspector")]
mod inspector;
mod shader;
mod ui;

pub fn main() {
    use app::MyAppPlugin;
    #[cfg(any(target_os = "ios", target_os = "android"))]
    use bevy::window::WindowMode;
    use bevy::winit::WinitSettings;
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
                        resizable: false,
                        // present_mode: bevy::window::PresentMode::AutoNoVsync,
                        // resolution: WindowResolution::default().with_scale_factor_override(2.),
                        // resolution: WindowResolution::new(412., 883.)
                        // .with_scale_factor_override(0.5),
                        canvas: Some("#target".to_string()),
                        fit_canvas_to_parent: true,
                        #[cfg(any(target_os = "ios", target_os = "android"))]
                        mode: WindowMode::SizedFullscreen(MonitorSelection::Primary),
                        #[cfg(any(target_os = "ios", target_os = "android"))]
                        recognize_rotation_gesture: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(MyAppPlugin)
        .insert_resource(WinitSettings::mobile())
        .run();
}
