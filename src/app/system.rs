use bevy::prelude::*;

use super::state::MyAppState;

pub(super) fn setup_camera_light(mut commands: Commands) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub(super) fn start_state(mut next_state: ResMut<NextState<MyAppState>>) {
    next_state.set(MyAppState::BaseAssetLoading);
}
