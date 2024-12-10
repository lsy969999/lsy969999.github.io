use std::f32::consts::PI;

use bevy::{color::palettes::css, prelude::*, render::view::RenderLayers};

use super::{
    component::{MyCamera2d, MyCamera3d},
    state::MyAppState,
};

pub(super) fn setup_camera_light(mut commands: Commands) {
    // light
    // commands.spawn((
    //     PointLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 6.0, 6.0),
    // ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT / 2.,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 22.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        // CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }
        // .build(),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            // clear_color: ClearColorConfig::Custom(css::WHITE.into()),
            ..default()
        },
        MyCamera3d,
        Transform::from_xyz(-12.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera2d::default(),
        MyCamera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::from_layers(&[1]),
    ));

    // commands.spawn((Text2d::new("wtf"), Name::new("test")));
}

pub(super) fn start_state(mut next_state: ResMut<NextState<MyAppState>>) {
    next_state.set(MyAppState::BaseAssetLoading);
}
