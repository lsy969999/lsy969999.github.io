use std::f32::consts::PI;

use bevy::{
    color::palettes::css,
    core_pipeline::{
        experimental::taa::TemporalAntiAliasing,
        fxaa::Fxaa,
        prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass},
    },
    pbr::{Cascade, CascadeShadowConfigBuilder, ClusterConfig, DirectionalLightShadowMap},
    prelude::*,
    render::view::RenderLayers,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_atmosphere::plugin::AtmosphereCamera;

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
        //     num_cascades: 1,               // 기본적으로 3개의 cascade 사용
        //     minimum_distance: 0.1,         // 카메라와 가까운 0.1 유닛부터 그림자 시작
        //     maximum_distance: 100.0,       // 100 유닛까지 그림자 렌더링
        //     first_cascade_far_bound: 10.0, // 첫 번째 cascade는 카메라로부터 10 유닛까지 적용
        //     overlap_proportion: 0.1,       // cascade 간 10%의 오버랩
        // }
        // .build(),
    ));

    // commands.insert_resource(DirectionalLightShadowMap { size: 1024 });

    // camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            // hdr: false,
            // clear_color: ClearColorConfig::Custom(css::WHITE.into()),
            ..default()
        }, // MSAA needs to be off for Deferred rendering
        // Msaa::Off,
        // TemporalAntiAliasing::default(),
        // DepthPrepass,
        // MotionVectorPrepass,
        // DeferredPrepass,
        // Fxaa::default(),
        MyCamera3d,
        AtmosphereCamera::default(),
        Transform::from_xyz(-12.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
        // ClusterConfig::Single,
    ));

    commands.spawn((
        Camera2d::default(),
        // Msaa::Off,
        // DepthPrepass,
        // DeferredPrepass,
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
pub fn toggle_cursor_grab_with_esc(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor_options.visible = !primary_window.cursor_options.visible;
        primary_window.cursor_options.grab_mode = if primary_window.cursor_options.visible {
            CursorGrabMode::None
        } else {
            CursorGrabMode::Locked
        };
    }
}
