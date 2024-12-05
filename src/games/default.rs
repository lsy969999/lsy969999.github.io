use bevy::{math::vec3, prelude::*};

use crate::asset::{
    resource::{FoxAssets, RequiredAssets},
    state::{FoxAssetsLoadState, RequiredAssetsLoadState},
};

pub struct MyDefaultGamePlugin;

impl Plugin for MyDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(OnEnter(FoxAssetsLoadState::LoadEnd), fox_spawn);
        app.add_systems(OnEnter(RequiredAssetsLoadState::LoadEnd), spawn_ui);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

fn fox_spawn(mut commands: Commands, my_assets: Res<FoxAssets>, gltf_assets: Res<Assets<Gltf>>) {
    let Some(gltf) = gltf_assets.get(my_assets.fox.id()) else {
        return;
    };
    info!("fox_spawn");
    let zz = gltf.scenes[0].clone();
    commands.spawn((
        SceneRoot(zz),
        Transform::from_xyz(2., 0., 0.).with_scale(vec3(0.01, 0.01, 0.01)),
    ));
}

fn spawn_ui(mut commands: Commands, my_assets: Res<RequiredAssets>) {
    info!("spawnui");
    commands.spawn((
        Name::new("text"),
        Text::new("text"),
        TextFont {
            font: my_assets.galmuri_mono11.clone(),
            ..default()
        },
    ));
}
