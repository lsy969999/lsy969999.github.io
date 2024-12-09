use crate::{
    app::{component::MyCamera, state::MyAppState},
    asset::resource::{BaseAssets, DefaultSceneAssets},
    shader::animated_shader::AnimatedShader,
};
use bevy::{
    color::palettes::css,
    gltf::{GltfMaterialName, GltfMesh, GltfNode, GltfSkin},
    math::{bounding::Aabb3d, vec3},
    prelude::*,
    render::{
        mesh::{skinning::SkinnedMesh, MeshAabb},
        primitives::Aabb,
    },
};
use bevy_rapier3d::prelude::*;
use std::{collections::HashMap, f32::consts::PI, time::Duration};

pub struct MyDefaultGamePlugin;

impl Plugin for MyDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::DefaultScene), new_setup);
        // app.add_systems(OnEnter(MyAppState::DefaultScene), fox_spawn.after(setup));
        app.add_systems(OnExit(MyAppState::DefaultScene), clear_scene);
        app.add_systems(
            Update,
            (
                // ani_player_added,
                draw_gizmo,
                draw_axes_helper_gizmo,
                player_movement,
                player_rotation,
                // camera_movement,
                added_default_scene,
            )
                .run_if(in_state(MyAppState::DefaultScene)),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ani_materials: ResMut<Assets<AnimatedShader>>,
    mut ani_graphs: ResMut<Assets<AnimationGraph>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    assets_gltfnode: Res<Assets<GltfNode>>,
    assets_gltfskin: Res<Assets<GltfSkin>>,
    def_assets: Res<DefaultSceneAssets>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    let def_secne = commands
        .spawn((
            Visibility::default(),
            Transform::default(),
            DefaultScene,
            Name::new("DefaultScene"),
        ))
        .id();
    // circular base

    commands.entity(def_secne).with_children(|parent| {
        // base
        // parent
        //     .spawn((
        //         Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
        //         MeshMaterial3d(materials.add(Color::WHITE)),
        //         RigidBody::Static,
        //         Collider::cylinder(4., 0.1),
        //     ))
        //     .observe(base_pointer_down);

        // // cube
        // parent.spawn((
        //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        //     Transform::from_xyz(2.0, 0.5, 0.0),
        //     PickingBehavior {
        //         is_hoverable: true,
        //         should_block_lower: false,
        //     },
        // ));

        // parent.spawn((
        //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        //     MeshMaterial3d(ani_materials.add(AnimatedShader {})),
        //     Transform::from_xyz(-2.0, 0.5, 0.0),
        // ));

        // parent.spawn((
        //     Mesh3d(meshes.add(Plane3d::default())),
        //     MeshMaterial3d(ani_materials.add(AnimatedShader {})),
        //     Transform::from_xyz(-4.0, 0.5, 0.0),
        // ));

        let Some(gltf) = gltf_assets.get(def_assets.fox.id()) else {
            return;
        };
        // parent
        //     .spawn((
        //         Visibility::default(),
        //         Transform::from_xyz(0., 1.0, 0.),
        //         Name::new("player"),
        //         Player,
        //         RigidBody::Kinematic,
        //         Collider::cuboid(1., 1., 1.),
        //         LockedAxes::new()
        //             .lock_rotation_x()
        //             .lock_rotation_y()
        //             .lock_rotation_z(),
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn((
        //             SceneRoot(gltf.scenes[0].clone()),
        //             Transform::from_xyz(0., -0.5, 0.).with_scale(vec3(0.01, 0.01, 0.01)),
        //         ));
        //     });

        let Some(gltf) = gltf_assets.get(def_assets.default_scene.id()) else {
            return;
        };
        for (node_name, handle) in &gltf.named_nodes {
            info!("default_scene node: {}", node_name);
        }
        for (node_name, handle) in &gltf.named_animations {
            info!("default_scene ani: {}", node_name);
        }

        let Some(floor_node) = assets_gltfnode.get(&gltf.named_nodes["Floor"]) else {
            return;
        };
        let Some(floor_mesh_handle) = &floor_node.mesh else {
            return;
        };
        let Some(floor_gltf_mesh) = assets_gltfmesh.get(floor_mesh_handle.id()) else {
            return;
        };

        let mesh = meshes.get(floor_gltf_mesh.primitives[0].mesh.id()).unwrap();
        parent
            .spawn((
                Mesh3d(floor_gltf_mesh.primitives[0].mesh.clone()),
                MeshMaterial3d(materials.add(Color::WHITE)),
                RigidBody::Fixed,
                floor_node.transform,
                // Collider::convex_hull_from_mesh(mesh).unwrap(),
            ))
            .observe(base_pointer_down);

        let Some(ico_node) = assets_gltfnode.get(&gltf.named_nodes["Icosphere"]) else {
            return;
        };
        let Some(ico_mesh_handle) = &ico_node.mesh else {
            return;
        };
        let Some(ico_gltf_mesh) = assets_gltfmesh.get(ico_mesh_handle.id()) else {
            return;
        };
        let mesh = meshes.get(ico_gltf_mesh.primitives[0].mesh.id()).unwrap();
        parent.spawn((
            Mesh3d(ico_gltf_mesh.primitives[0].mesh.clone()),
            MeshMaterial3d(materials.add(Color::WHITE)),
            ico_node.transform,
            RigidBody::Dynamic,
            // Collider::convex_hull_from_mesh(mesh).unwrap(),
        ));

        let Some(tree_node) = assets_gltfnode.get(&gltf.named_nodes["Tree"]) else {
            return;
        };
        for child in &tree_node.children {
            info!("child {:?}", child);
        }
        let Some(tree_mesh_handle) = &tree_node.mesh else {
            return;
        };
        let Some(tree_gltf_mesh) = assets_gltfmesh.get(tree_mesh_handle.id()) else {
            return;
        };

        for primitive in &tree_gltf_mesh.primitives {
            // let mesh = meshes.get(primitive.mesh.id()).unwrap();
            parent.spawn((
                Mesh3d(primitive.mesh.clone()),
                MeshMaterial3d(primitive.material.clone().unwrap()),
                tree_node.transform,
            ));
        }

        let Some(coin_node) = assets_gltfnode.get(&gltf.named_nodes["Coin"]) else {
            return;
        };
        for child in &coin_node.children {
            info!("child {:?}", child);
        }
        let Some(coin_mesh_handle) = &coin_node.mesh else {
            return;
        };
        let Some(coin_gltf_mesh) = assets_gltfmesh.get(coin_mesh_handle.id()) else {
            return;
        };

        for primitive in &coin_gltf_mesh.primitives {
            // let mesh = meshes.get(primitive.mesh.id()).unwrap();
            parent.spawn((
                Mesh3d(primitive.mesh.clone()),
                MeshMaterial3d(primitive.material.clone().unwrap()),
                coin_node.transform,
            ));
        }

        // let Some(character_mesh_handle) = &character_node.mesh else {
        //     return;
        // };
        // let Some(character_gltf_mesh) = assets_gltfmesh.get(character_mesh_handle.id()) else {
        //     return;
        // };

        // parent.spawn((SceneRoot(gltf.scenes[0].clone()),));
        // parent.spawn((
        //     SceneRoot(gltf.scenes[0].clone()),
        //     Transform::from_xyz(0., 0., 0.).with_scale(vec3(0.01, 0.01, 0.01)),
        //     Player,
        //     Collider::cuboid(100., 100., 100.),
        //     Name::new("player"),
        // ));
    });

    let Some(gltf) = gltf_assets.get(def_assets.default_scene.id()) else {
        return;
    };
    let Some(character_node) = assets_gltfnode.get(&gltf.named_nodes["CharacterArmature"]) else {
        return;
    };
    info!(
        "character_node_tr {:?}, mesh {:?} skin {:?}",
        character_node.transform, character_node.mesh, character_node.skin
    );

    let tar = commands
        .spawn((
            Name::new("target"),
            character_node.transform,
            Visibility::default(),
        ))
        .id();

    let mut tar_child = vec![];
    for child in &character_node.children {
        let Some(gltf_node) = assets_gltfnode.get(child.id()) else {
            info!("1");
            break;
        };

        info!(
            "child name: {:?}, child_len: {:?}, mesh: {:?}, skin: {:?}",
            gltf_node.name,
            gltf_node.children.len(),
            gltf_node.mesh,
            gltf_node.skin
        );
        let Some(gltf_skin_handle) = &gltf_node.skin else {
            break;
        };
        let Some(gltf_skin) = assets_gltfskin.get(gltf_skin_handle.id()) else {
            break;
        };

        let mut spawned_joints: Vec<Entity> = Vec::new();
        let mut joint_map = HashMap::new();
        for (idx, joint) in gltf_skin.joints.iter().enumerate() {
            let Some(gltf_node2) = assets_gltfnode.get(joint.id()) else {
                info!("j1");
                break;
            };

            info!(
                "skin {:?}, mesh: {:?}, children_len {:?}  tr {:?}",
                gltf_node2.name,
                gltf_node2.mesh,
                gltf_node2.children.len(),
                gltf_node2.transform,
            );
            let asset_id = joint.id();
            let joint_entity = commands
                .spawn((Name::new("skin"), gltf_node2.transform))
                .id();
            spawned_joints.push(joint_entity);
            joint_map.insert(asset_id, joint_entity);
            //
        }

        for joint in gltf_skin.joints.iter() {
            let joint_entity = joint_map.get(&joint.id()).unwrap();
            let joint = assets_gltfnode.get(joint).unwrap();

            let mut child_entities = Vec::new();
            joint.children.iter().for_each(|child| {
                if let Some(child_entity) = joint_map.get(&child.id()) {
                    child_entities.push(*child_entity);
                }
            });
            commands.entity(*joint_entity).add_children(&child_entities);
        }

        let Some(gltf_mesh_handle) = &gltf_node.mesh else {
            break;
        };
        let Some(gltf_mesh) = assets_gltfmesh.get(gltf_mesh_handle.id()) else {
            break;
        };
        for primitive in &gltf_mesh.primitives {
            info!("primitive: {:?}", primitive.name);
            let mesh = meshes.get(primitive.mesh.id()).unwrap();
            let e = commands
                .spawn((
                    Name::new("assss"),
                    Mesh3d(primitive.mesh.clone()),
                    MeshMaterial3d(primitive.material.clone().unwrap()),
                    gltf_node.transform,
                    SkinnedMesh {
                        joints: spawned_joints.clone(),
                        inverse_bindposes: gltf_skin.inverse_bind_matrices.clone(),
                    },
                ))
                .id();
            tar_child.push(e);
        }
        // break;
        info!("-----");
    }

    info!("tar_child {:?}", tar_child);
    // commands.entity(tar).add_children(&tar_child);
    // let Some(gltf) = gltf_assets.get(def_assets.character.id()) else {
    //     return;
    // };
    // commands.spawn((
    //     SceneRoot(gltf.scenes[0].clone()),
    //     Name::new("chharacter"),
    //     Transform::from_xyz(2., 2., 2.),
    // ));
    // let Some(CustomAsset { value }) = assetcustom.get(base_asset.custom.id()) else {
    //     return;
    // };
    // info!("customasset: {value:?}");
    // let Some(TestAsset { test }) = assettest.get(base_asset.test.id()) else {
    //     return;
    // };
    // info!("testasset: {test:?}");
}

fn new_setup(
    mut commands: Commands,
    def_assets: Res<DefaultSceneAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let Some(gltf) = gltf_assets.get(def_assets.default_scene.id()) else {
        return;
    };

    let mut animatiton_map = HashMap::<String, AnimationNodeIndex>::new();
    let mut anmation_graph = AnimationGraph::new();
    for (name, clip_handle) in &gltf.named_animations {
        info!("ani: {name}");
        let idx = anmation_graph.add_clip(clip_handle.clone(), 1., anmation_graph.root);
        animatiton_map.insert(name.to_string(), idx);
    }

    let graph_handle = graphs.add(anmation_graph);
    commands.insert_resource(Animations {
        animations: animatiton_map,
        graph: graph_handle,
    });

    commands.spawn((
        SceneRoot(gltf.scenes[0].clone()),
        DefaultScene,
        Name::new("DefaultScene"),
    ));
}

fn added_default_scene(
    mut commands: Commands,
    q_added_name: Query<(Entity, &Children, &Parent, &Name, &Transform), Added<Name>>,
    q_mesh: Query<(&Mesh3d, &Name)>,
    mut q_animation_player: Query<&mut AnimationPlayer>,
    meshes: Res<Assets<Mesh>>,
    animations: Res<Animations>,
) {
    for (entity, children, parent, name, transform) in &q_added_name {
        match name.as_str() {
            "Floor" => {
                for entity in children {
                    let Ok((Mesh3d(mesh_handle), _)) = q_mesh.get(*entity) else {
                        break;
                    };
                    let mesh = meshes.get(mesh_handle.id()).unwrap();
                    commands
                        .entity(*entity)
                        // .insert(Collider::convex_hull_from_mesh(mesh).unwrap())
                        .insert(
                            Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexHull)
                                .unwrap(),
                        )
                        .insert(RigidBody::Fixed)
                        .observe(base_pointer_down);
                }
            }

            coin if coin.contains("Coin") => {
                for child_entity in children {
                    let Ok((Mesh3d(mesh_handle), name)) = q_mesh.get(*child_entity) else {
                        break;
                    };
                    // == "Cylinder.001.0"
                    if name.as_str().ends_with(".0") {
                        let mesh = meshes.get(mesh_handle.id()).unwrap();
                        let aabb = mesh.compute_aabb().unwrap();
                        let extent = aabb.half_extents;
                        info!("extent: {extent:?}");

                        commands.entity(entity).insert((
                            RigidBody::KinematicVelocityBased,
                            Collider::cuboid(extent.x, extent.y, extent.z),
                            Velocity::angular(Vec3::new(0., 1., 0.)),
                        ));
                    }
                }
            }
            "CharacterArmature" => {
                let Ok(mut player) = q_animation_player.get_mut(entity) else {
                    return;
                };
                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(&mut player, animations.animations["Walk"], Duration::ZERO)
                    .repeat();
                commands
                    .entity(entity)
                    .insert(Transform::from_xyz(0., -1.5, 0.))
                    .insert(AnimationGraphHandle(animations.graph.clone()))
                    .insert(transitions);

                let z = Transform {
                    translation: vec3(
                        transform.translation.x,
                        transform.translation.y + 1.5,
                        transform.translation.z,
                    ),
                    ..*transform
                };
                let new_rigid = commands
                    .spawn_empty()
                    .insert(Visibility::default())
                    .insert(Player)
                    .insert(z)
                    .insert(Collider::cuboid(1., 2., 1.))
                    .insert(RigidBody::Dynamic)
                    .insert(LockedAxes::ROTATION_LOCKED_X)
                    .insert(LockedAxes::ROTATION_LOCKED_Z)
                    .id();
                commands.entity(new_rigid).set_parent(**parent);
                commands.entity(entity).set_parent(new_rigid);
            }
            _ => {}
        }
    }
}

fn draw_axes_helper_gizmo(mut gizmos: Gizmos) {
    let transform = Transform::from_xyz(0., 10., 0.);
    gizmos.axes(transform, 3.);
}

fn draw_gizmo(mut gizmos: Gizmos, q_flag: Query<&MoveFlag>) {
    let Ok(MoveFlag(pos)) = q_flag.get_single() else {
        return;
    };
    // gizmos.cross(*pos, 1., css::BLACK);
    gizmos.arrow(
        vec3(pos.x, pos.y + 0.5, pos.z),
        vec3(pos.x, pos.y + 0.1, pos.z),
        css::BLACK,
    );
}

#[derive(Resource)]
struct Animations {
    animations: HashMap<String, AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(Component)]
struct DefaultScene;

#[derive(Component, Debug)]
pub struct MoveFlag(pub Vec3);

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

fn base_pointer_down(
    down: Trigger<Pointer<Down>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_def_scene: Query<Entity, With<DefaultScene>>,
    q_flag: Query<(Entity, &MoveFlag)>,
    q_player: Query<(Entity, &Transform), With<Player>>,
) {
    let Some(pos) = down.hit.position else {
        return;
    };
    let Ok(def_scene) = q_def_scene.get_single() else {
        return;
    };

    for (entity, _) in &q_flag {
        commands.entity(entity).despawn_recursive();
    }

    commands.entity(def_scene).with_children(|parent| {
        parent.spawn((Name::new("MoveFlag"), MoveFlag(pos)));
    });
    let Ok((player_entity, player_tr)) = q_player.get_single() else {
        return;
    };
    let direction =
        (player_tr.translation - vec3(pos.x, player_tr.translation.y, pos.z)).normalize();
    // commands
    //     .entity(player_entity)
    //     .insert(Velocity::linear(-direction));
    // let Ok((entity, transform)) = q_player.get_single() else {
    //     return;
    // };
    // let direction = (pos - transform.translation).normalize();
    // commands.entity(entity).insert(LinearVelocity(direction));
}

#[derive(Component)]
pub struct Player;

fn player_movement(
    mut commands: Commands,
    q_flag: Query<(Entity, &MoveFlag)>,
    mut q_player: Query<(Entity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    //
    let Ok((flag_entity, MoveFlag(pos))) = q_flag.get_single() else {
        return;
    };
    let Ok((player_entity, mut tr)) = q_player.get_single_mut() else {
        return;
    };

    let speed = 1.;
    let direction = (tr.translation - vec3(pos.x, tr.translation.y, pos.z)).normalize();
    let distance = (tr.translation - vec3(pos.x, tr.translation.y, pos.z)).length();
    // let step = speed * time.delta_secs();

    commands
        .entity(player_entity)
        .insert(Velocity::linear(-direction * 2.));

    // tr.translation = tr.translation - (direction * step);
    // info!("tr.translation {}", tr.translation);
    info!("distance {}", distance);
    if distance <= 0.1 {
        commands.entity(flag_entity).despawn_recursive();
        commands.entity(player_entity).insert(Velocity::zero());
    }
}
fn player_rotation(
    mut commands: Commands,
    q_flag: Query<(Entity, &MoveFlag)>,
    mut q_player: Query<(Entity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    //
    let Ok((flag_entity, MoveFlag(pos))) = q_flag.get_single() else {
        return;
    };
    let Ok((player_entity, mut tr)) = q_player.get_single_mut() else {
        return;
    };
    let speed = 1.;
    let direction = (tr.translation - vec3(pos.x, tr.translation.y, pos.z)).normalize();
    let distance = (tr.translation - vec3(pos.x, pos.y + 1.0, pos.z)).length();
    let step = speed * time.delta_secs();

    // let up = Vec3::Y;
    tr.rotation = Quat::from_rotation_arc(Vec3::Z, -direction);

    // let z = Quat::from_rotation_arc(Vec3::Z, -direction);

    // commands.entity(player_entity).insert(Rotation(z));
}
fn camera_movement(
    mut commands: Commands,
    mut q_camera: Query<(Entity, &mut Transform), (With<MyCamera>, Without<Player>)>,
    q_player: Query<(Entity, &Transform), With<Player>>,
) {
    let Ok((_, mut c_tr)) = q_camera.get_single_mut() else {
        return;
    };
    let Ok((_, p_tr)) = q_player.get_single() else {
        return;
    };
    c_tr.translation = vec3(
        p_tr.translation.x - 2.5,
        p_tr.translation.y + 4.5,
        p_tr.translation.z + 9.0,
    );
}
