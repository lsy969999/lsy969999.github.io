use bevy::prelude::*;

use super::{component::MyCamera, state::MyAppState};

pub(super) fn setup_camera_light(mut commands: Commands) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 6.0, 6.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        MyCamera,
        Transform::from_xyz(-12.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub(super) fn start_state(mut next_state: ResMut<NextState<MyAppState>>) {
    next_state.set(MyAppState::BaseAssetLoading);
}
