use bevy::{math::vec3, prelude::*};

use crate::{
    app::state::MyAppState,
    asset::resource::{BaseAssets, DefaultSceneAssets},
};

pub struct MyDefaultGamePlugin;

impl Plugin for MyDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::DefaultScene), setup);
        app.add_systems(OnEnter(MyAppState::DefaultScene), fox_spawn);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("setup");
    // circular base
    commands
        .spawn((
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ))
        .observe(|out: Trigger<Pointer<Click>>| {
            info!("base click depth: {:?}", out.hit.depth);
        });
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

fn fox_spawn(
    mut commands: Commands,
    my_assets: Res<DefaultSceneAssets>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    let Some(gltf) = gltf_assets.get(my_assets.fox.id()) else {
        return;
    };
    info!("fox_spawn");
    commands.spawn((
        SceneRoot(gltf.scenes[0].clone()),
        Transform::from_xyz(2., 0., 0.).with_scale(vec3(0.01, 0.01, 0.01)),
    ));
}
