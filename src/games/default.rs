use std::time::Duration;

use bevy::{math::vec3, prelude::*};

use crate::{
    app::state::MyAppState,
    asset::resource::{BaseAssets, DefaultSceneAssets},
    shader::animated_shader::AnimatedShader,
};

pub struct MyDefaultGamePlugin;

impl Plugin for MyDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::DefaultScene), setup);
        app.add_systems(OnEnter(MyAppState::DefaultScene), fox_spawn.after(setup));
        app.add_systems(Update, ani_player_added);
        app.add_systems(OnExit(MyAppState::DefaultScene), clear_scene);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ani_materials: ResMut<Assets<AnimatedShader>>,
    base_asset: Res<BaseAssets>,
) {
    info!("setup");
    // circular base
    commands
        .spawn((Visibility::default(), Transform::default(), DefaultScene))
        .with_children(|parent| {
            parent
                .spawn((
                    Mesh3d(meshes.add(Circle::new(4.0))),
                    MeshMaterial3d(materials.add(Color::WHITE)),
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                ))
                .observe(|out: Trigger<Pointer<Click>>| {
                    info!("base click depth: {:?}", out.hit.depth);
                });
            // cube
            parent
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                    Transform::from_xyz(2.0, 0.5, 0.0),
                    PickingBehavior {
                        is_hoverable: true,
                        should_block_lower: false,
                    },
                ))
                .observe(|out: Trigger<Pointer<Click>>| {
                    info!("cube click depth: {:?}", out.hit.depth);
                });

            parent
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(ani_materials.add(AnimatedShader {})),
                    Transform::from_xyz(-2.0, 0.5, 0.0),
                ))
                .observe(|out: Trigger<Pointer<Click>>| {
                    info!("cube click depth: {:?}", out.hit.depth);
                });
        });

    // let Some(CustomAsset { value }) = assetcustom.get(base_asset.custom.id()) else {
    //     return;
    // };
    // info!("customasset: {value:?}");
    // let Some(TestAsset { test }) = assettest.get(base_asset.test.id()) else {
    //     return;
    // };
    // info!("testasset: {test:?}");
}

fn fox_spawn(
    mut commands: Commands,
    my_assets: Res<DefaultSceneAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    q_def_scene: Query<Entity, With<DefaultScene>>,
) {
    let Some(gltf) = gltf_assets.get(my_assets.fox.id()) else {
        return;
    };

    for (name, clip) in gltf.named_animations.iter() {
        info!("ani name: {name:?}");
    }
    for (name, clip) in gltf.named_nodes.iter() {
        info!("node name: {name:?}");
    }
    let animations = &gltf.animations;

    let Ok(entity) = q_def_scene.get_single() else {
        return;
    };
    // let (graph, node_indices) = AnimationGraph::from_clips(animations.clone());
    // let graph_handle = graphs.add(graph);
    // commands.insert_resource(Animations {
    //     animations: node_indices,
    //     graph: graph_handle,
    // });
    commands.entity(entity).with_children(|parent| {
        info!("fox_spawn");
        parent.spawn((
            SceneRoot(gltf.scenes[0].clone()),
            Transform::from_xyz(0., 0., 0.).with_scale(vec3(0.01, 0.01, 0.01)),
        ));
    });
}

fn ani_player_added(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    animations: Res<Animations>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}
#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(Component)]
struct DefaultScene;

fn clear_scene(
    mut commands: Commands,
    q_def_scene: Query<(Entity, &Children), With<DefaultScene>>,
) {
    let Ok((entity, children)) = q_def_scene.get_single() else {
        return;
    };
    for entity in children {
        commands.entity(*entity).despawn_recursive();
    }
    commands.entity(entity).despawn_recursive();
}
