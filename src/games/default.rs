use bevy::{math::vec3, prelude::*};

use crate::asset::{
    event::{FontLoaded, FoxLoaded},
    resource::MyAssets,
};

pub struct MyDefaultGamePlugin;

impl Plugin for MyDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(Update, fox_spawn);
        app.add_systems(Update, spawn_ui);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("setup");
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

fn fox_spawn(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    mut er: EventReader<FoxLoaded>,
) {
    for _ in er.read() {
        // let MyAssetLoadedEvent::FoxLoaded = evt else {
        //     return;
        // };
        let Some(gltf) = gltf_assets.get(my_assets.fox.id()) else {
            return;
        };
        info!("fox_spawn");
        commands.spawn((
            SceneRoot(gltf.scenes[0].clone()),
            Transform::from_xyz(2., 0., 0.).with_scale(vec3(0.01, 0.01, 0.01)),
        ));
    }
}

fn spawn_ui(mut commands: Commands, my_assets: Res<MyAssets>, mut er: EventReader<FontLoaded>) {
    for _ in er.read() {
        // let MyAssetLoadedEvent::FontLoaded = evt else {
        //     return;
        // };
        info!("spawnui");
        commands.spawn((
            Name::new("text"),
            Text::new("text"),
            TextFont {
                font: my_assets.font.clone(),
                ..default()
            },
        ));
    }
}
