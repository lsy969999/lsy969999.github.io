use crate::{
    app::{
        component::{MyCamera2d, MyCamera3d},
        state::MyAppState,
    },
    asset::resource::{BaseAssets, DefaultSceneAssets},
    shader::animated_shader::AnimatedShader,
};
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css,
    math::vec3,
    prelude::*,
    render::{
        mesh::{skinning::SkinnedMesh, MeshAabb, MeshVertexAttribute},
        render_resource::VertexFormat,
        view::{NoFrustumCulling, RenderLayers},
    },
};
use bevy_rapier3d::prelude::*;
use std::{collections::HashMap, f32::consts::PI, time::Duration};

pub struct MyDefaultGamePlugin;

impl Plugin for MyDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        // app.init_state::<PlayerAniState>();
        app.add_systems(
            OnEnter(MyAppState::DefaultScene),
            (new_setup, new_test_shader),
        );
        // app.add_systems(OnEnter(MyAppState::DefaultScene), fox_spawn.after(setup));
        app.add_systems(OnExit(MyAppState::DefaultScene), clear_scene);
        app.add_systems(
            Update,
            (
                draw_gizmo,
                draw_axes_helper_gizmo,
                player_movement,
                player_rotation,
                // camera_movement,
                added_default_scene,
                player_ani_changed,
                added_mesh3d_picking,
                disable_culling_for_skinned_meshes,
                monster_text,
                read_result_system,
                monster_bee_ani_changed,
                display_events,
                player_monster_attack_collision,
                // character_controller,
            )
                .run_if(in_state(MyAppState::DefaultScene)),
        );
        app.register_type::<PlayerAnimationState>();
        app.register_type::<MonsterBeeAnimationState>();
    }
}
/// System that automatically disables frustum culling for
/// all skinned meshes, as soon as they are added to the world.
fn disable_culling_for_skinned_meshes(
    mut commands: Commands,
    skinned: Query<Entity, Added<SkinnedMesh>>,
) {
    for entity in &skinned {
        commands.entity(entity).insert(NoFrustumCulling);
    }
}

fn new_test_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animated_materials: ResMut<Assets<AnimatedShader>>,
) {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(
        MeshVertexAttribute::new("Vertex_Position", 0, VertexFormat::Float32x3),
        vec![
            [-0.5, -0.5, 0.0], // 좌하단 정점
            [0.5, -0.5, 0.0],  // 우하단 정점
            [0.0, 0.5, 0.0],   // 상단 정점
        ],
    );
    mesh.insert_indices(bevy::render::mesh::Indices::U32(vec![0, 1, 2]));
    let handle = meshes.add(mesh);

    commands.spawn((
        Name::new("testshader"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(animated_materials.add(AnimatedShader {})),
        Transform::from_xyz(0., 2., 0.),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
    ));

    commands.spawn((
        Name::new("testshader2"),
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(animated_materials.add(AnimatedShader {})),
        Transform::from_xyz(0., 3., 0.),
    ));
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

    for (a, b) in &gltf.named_nodes {
        info!("node: {a:?}");
    }
    for (a, b) in &gltf.named_scenes {
        info!("scene: {a:?}");
    }

    let graph_handle = graphs.add(anmation_graph);
    commands.insert_resource(PlayerAnimations {
        animations: animatiton_map.clone(),
        graph: graph_handle.clone(),
    });
    commands.insert_resource(MonsterBeeAnimations {
        animations: animatiton_map,
        graph: graph_handle,
    });

    commands.spawn((
        SceneRoot(gltf.named_scenes["Scene"].clone()),
        DefaultScene,
        Name::new("DefaultScene"),
    ));
}

fn added_mesh3d_picking(
    mut commands: Commands,
    q_added_mesh: Query<(Entity, &Name), Added<Mesh3d>>,
) {
    for (entity, name) in &q_added_mesh {
        // info!("zzzff");
        if name.as_str() != "Cube.001" {
            commands.entity(entity).insert(PickingBehavior::IGNORE);
        }
    }
}
fn display_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }
}
fn player_monster_attack_collision(
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<&Player>,
    q_enemy: Query<&MonsterEnemyAttackSensor>,
    mut q_plyer_ani: Query<&mut PlayerAnimationState>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, collision_event_flags) => {
                if q_player.get(*entity1).is_ok() && q_enemy.get(*entity2).is_ok()
                    || q_player.get(*entity2).is_ok() && q_enemy.get(*entity1).is_ok()
                {
                    info!("player attack enemy start");
                    if let Ok(mut state) = q_plyer_ani.get_single_mut() {
                        info!("punch!!");
                        *state = PlayerAnimationState::Punch;
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, collision_event_flags) => {
                if q_player.get(*entity1).is_ok() && q_enemy.get(*entity2).is_ok()
                    || q_player.get(*entity2).is_ok() && q_enemy.get(*entity1).is_ok()
                {
                    info!("player attack enemy end");
                }
            }
        }
    }
}
fn added_default_scene(
    mut commands: Commands,
    q_added_name: Query<(Entity, &Children, &Parent, &Name, &Transform), Added<Name>>,
    q_mesh: Query<(&Mesh3d, &Name)>,
    q_entity: Query<Entity>,
    mut q_animation_player: Query<&mut AnimationPlayer>,
    meshes: Res<Assets<Mesh>>,
    player_animations: Res<PlayerAnimations>,
    bee_animations: Res<PlayerAnimations>,
    def_assets: Res<DefaultSceneAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut animated_materials: ResMut<Assets<AnimatedShader>>,
) {
    for (entity, children, parent, name, transform) in &q_added_name {
        match name.as_str() {
            "MonsterArmature_Enemy" => {
                let Ok(mut player) = q_animation_player.get_mut(entity) else {
                    return;
                };
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
                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(&mut player, animatiton_map["Enemy_Idle"], Duration::ZERO)
                    .repeat();
                commands
                    .entity(entity)
                    .insert(MonsterEnemy)
                    .insert(AnimationGraphHandle(graphs.add(anmation_graph)))
                    .insert(MonsterEmenyAnimationState::Idle)
                    .insert(Collider::ball(1.))
                    .insert(RigidBody::KinematicVelocityBased)
                    .with_children(|parent| {
                        parent
                            .spawn(Collider::ball(3.))
                            .insert(Transform::default())
                            .insert(RigidBody::Fixed)
                            .insert(Sensor)
                            .insert(MonsterEnemyAttackSensor)
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(ActiveCollisionTypes::KINEMATIC_STATIC)
                            // .insert(ActiveCollisionTypes::all())
                            ;
                    });
                info!("MonsterArmature_Enemy");
            }
            "EnemySpawnZone" => {
                //
                let Some(gltf) = gltf_assets.get(def_assets.default_scene.id()) else {
                    return;
                };

                // let tr = transform.clone();

                let mut tr = transform.clone();

                for _ in 1..=1 {
                    commands.spawn((
                        SceneRoot(gltf.named_scenes["MonsterEnemy"].clone()),
                        tr,
                        Name::new("MonsterEnemy"),
                    ));
                    tr.translation.x += 3.;
                }
            }
            "Floor" => {
                for entity in children {
                    let Ok((Mesh3d(mesh_handle), _)) = q_mesh.get(*entity) else {
                        break;
                    };
                    let mesh = meshes.get(mesh_handle.id()).unwrap();
                    let aabb = mesh.compute_aabb().unwrap();
                    let extent = aabb.half_extents;
                    commands
                        .entity(*entity)
                        // .insert(Collider::convex_hull_from_mesh(mesh).unwrap())
                        // .insert(
                        //     Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexHull)
                        //         .unwrap(),
                        // )
                        .insert(Collider::cuboid(extent.x, extent.y, extent.z))
                        .insert(RigidBody::Fixed)
                        .observe(base_pointer_down);
                }
            }
            "Icosphere" => {
                for child_entity in children {
                    let Ok((Mesh3d(mesh_handle), name)) = q_mesh.get(*child_entity) else {
                        break;
                    };
                    let Ok(mat_entity) = q_entity.get(*child_entity) else {
                        break;
                    };
                    let mesh = meshes.get(mesh_handle.id()).unwrap();
                    // let aabb = mesh.compute_aabb().unwrap();
                    // let extent = aabb.half_extents;
                    // info!("extent: {extent:?}");

                    commands.entity(entity).insert((
                        RigidBody::Dynamic,
                        // Collider::ball(extent.x),
                        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexHull).unwrap(),
                    ));
                    commands
                        .entity(mat_entity)
                        .remove::<MeshMaterial3d<StandardMaterial>>()
                        .insert(MeshMaterial3d(animated_materials.add(AnimatedShader {})));
                }
            }
            text if text.starts_with("Text") => {
                for child_entity in children {
                    let Ok((Mesh3d(mesh_handle), name)) = q_mesh.get(*child_entity) else {
                        break;
                    };
                    let mesh = meshes.get(mesh_handle.id()).unwrap();
                    let aabb = mesh.compute_aabb().unwrap();
                    let extent = aabb.half_extents;
                    // info!("extent: {extent:?}");

                    commands.entity(entity).insert((
                        RigidBody::Dynamic,
                        Collider::cuboid(extent.x, extent.y, extent.z),
                        // Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexHull).unwrap(),
                    ));
                }
            }
            coin if coin.starts_with("Coin") => {
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
            "Stair" => {
                for entity in children {
                    let Ok((Mesh3d(mesh_handle), _)) = q_mesh.get(*entity) else {
                        break;
                    };
                    let Ok(mat_entity) = q_entity.get(*entity) else {
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
                        .insert(RigidBody::Fixed);

                    commands
                        .entity(mat_entity)
                        .remove::<MeshMaterial3d<StandardMaterial>>()
                        .insert(MeshMaterial3d(animated_materials.add(AnimatedShader {})));
                }
            }
            "TestPlane" => {
                for entity in children {
                    let Ok((Mesh3d(mesh_handle), _)) = q_mesh.get(*entity) else {
                        break;
                    };
                    let Ok(mat_entity) = q_entity.get(*entity) else {
                        break;
                    };
                    let mesh = meshes.get(mesh_handle.id()).unwrap();

                    // commands
                    //     .entity(*entity)
                    //     // .insert(Collider::convex_hull_from_mesh(mesh).unwrap())
                    //     .insert(
                    //         Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexHull)
                    //             .unwrap(),
                    //     )
                    //     .insert(RigidBody::Fixed);

                    commands
                        .entity(mat_entity)
                        .remove::<MeshMaterial3d<StandardMaterial>>()
                        .insert(MeshMaterial3d(animated_materials.add(AnimatedShader {})));
                }
            }
            "MonsterArmature_Bee" => {
                let Ok(mut player) = q_animation_player.get_mut(entity) else {
                    return;
                };

                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(
                        &mut player,
                        bee_animations.animations["Bee_Flying"],
                        Duration::ZERO,
                    )
                    .repeat();
                commands
                    .entity(entity)
                    .insert(MonsterBee)
                    .insert(AnimationGraphHandle(bee_animations.graph.clone()))
                    .insert(MonsterBeeAnimationState::Flying);
            }
            "CharacterArmature" => {
                let Ok(mut player) = q_animation_player.get_mut(entity) else {
                    return;
                };

                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(
                        &mut player,
                        player_animations.animations["Idle"],
                        Duration::ZERO,
                    )
                    .repeat();
                commands
                    .entity(entity)
                    .insert(Transform::from_xyz(0., -1.5, 0.))
                    .insert(AnimationGraphHandle(player_animations.graph.clone()))
                    .insert(PlayerAnimationState::Idle)
                    .insert(transitions);

                let z = Transform {
                    translation: vec3(
                        transform.translation.x,
                        transform.translation.y + 2.5,
                        transform.translation.z,
                    ),
                    ..*transform
                };
                let new_rigid = commands
                    .spawn_empty()
                    .insert(Visibility::default())
                    .insert(Player)
                    .insert(z)
                    .insert(Velocity::zero())
                    .insert(Collider::capsule(vec3(0., -0.6, 0.), vec3(0., 1., 0.), 1.))
                    // .insert(Collider::cuboid(1., 2., 1.))
                    // .insert(RigidBody::Dynamic)
                    // .insert(LockedAxes::TRANSLATION_LOCKED)
                    // .insert(LockedAxes::ROTATION_LOCKED_Z)
                    .insert(RigidBody::KinematicPositionBased)
                    .insert(KinematicCharacterController {
                        // snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                        ..default()
                    })
                    // .insert(ActiveEvents::COLLISION_EVENTS)
                    .id();
                commands.entity(new_rigid).set_parent(**parent);
                commands.entity(entity).set_parent(new_rigid);
            }
            "Gun" => {
                commands.entity(entity).insert(Visibility::Hidden);
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct MonsterBee;
#[derive(Component)]
pub struct MonsterEnemy;
#[derive(Component)]
pub struct MonsterEnemyAttackSensor;

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

pub fn monster_text(
    mut commands: Commands,
    mut q_camera3d: Query<(Entity, &GlobalTransform, &Camera), (With<MyCamera3d>)>,
    mut q_camera2d: Query<(Entity, &GlobalTransform, &Camera), (With<MyCamera2d>)>,
    q_bee: Query<(Entity, &Transform), With<MonsterBee>>,
    mut q_bee_text: Query<
        (Entity, &mut Text2d, &mut Transform),
        (With<BeeText>, Without<MonsterBee>),
    >,
    base_asset: Res<BaseAssets>,
) {
    let Ok((camera_entity, camera3d_transform, camera3d)) = q_camera3d.get_single() else {
        return;
    };
    let Ok((camera_entity, camera2d_transform, camera2d)) = q_camera2d.get_single() else {
        return;
    };
    let Ok((bee_entity, bee_trsnform)) = q_bee.get_single() else {
        return;
    };
    let mut bee_trsnform = bee_trsnform.clone();
    bee_trsnform.translation.y += 2.;
    // todo: replate unwrap
    let Ok(viewport) = camera3d.world_to_viewport(camera3d_transform, bee_trsnform.translation)
    else {
        return;
    };
    let Ok(bee_2d_pos) = camera2d.viewport_to_world_2d(camera2d_transform, viewport) else {
        return;
    };

    if let Ok((_, text, mut tr)) = q_bee_text.get_single_mut() {
        tr.translation.x = bee_2d_pos.x;
        tr.translation.y = bee_2d_pos.y;
    } else {
        commands.spawn((
            BeeText,
            Text2d::new("안녕!"),
            TextFont {
                font: base_asset.font.clone(),
                ..default()
            },
            Transform::from_xyz(bee_2d_pos.x, bee_2d_pos.y, 0.),
            RenderLayers::layer(1),
        ));
    }

    // commands.entity(bee_entity).with_children(|parent| {
    //     parent
    //         .spawn(Text2d::new("text!!!"))
    //         .insert(Transform::from_xyz(zzzz.x, zzzz.y, 1.));
    // });
}

#[derive(Component)]
pub struct BeeText;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub animations: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}
#[derive(Resource)]
pub struct MonsterBeeAnimations {
    pub animations: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}
#[derive(Resource)]
pub struct MonsterEnemyAnimations {
    pub animations: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
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
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_def_scene: Query<Entity, With<DefaultScene>>,
    q_flag: Query<(Entity, &MoveFlag)>,
    q_player: Query<(Entity, &Transform), With<Player>>,
    mut q_player_ani_state: Query<(Entity, &mut PlayerAnimationState), With<PlayerAnimationState>>,
) {
    let Some(flag_pos) = down.hit.position else {
        return;
    };
    let Ok(def_scene) = q_def_scene.get_single() else {
        return;
    };

    for (entity, _) in &q_flag {
        commands.entity(entity).despawn_recursive();
    }

    commands.entity(def_scene).with_children(|parent| {
        parent.spawn((Name::new("MoveFlag"), MoveFlag(flag_pos)));
    });

    for (_entity, mut state) in &mut q_player_ani_state {
        if *state != PlayerAnimationState::Walking {
            *state = PlayerAnimationState::Walking;
        }
    }

    let Ok((player_entity, player_tr)) = q_player.get_single() else {
        return;
    };
    let speed = 1.;
    let sub = vec3(flag_pos.x, player_tr.translation.y, flag_pos.z) - player_tr.translation;
    let mut direction = (sub).normalize();

    // let up = Vec3::Y;
    // let z = Quat::from_rotation_arc(Vec3::Z, direction);

    // player_tr.rotation = player_tr.rotation.slerp(z, time.delta_secs() * 10.);

    // let tween: Tween<Transform> = Tween::new(
    //     EaseFunction::QuarticInOut,
    //     Duration::from_millis(250),
    //     TransformRotationLens {
    //         start: player_tr.rotation,
    //         end: z,
    //     },
    // );

    // commands.entity(player_entity).insert(Animator::new(tween));
    // let distance = (player_tr.translation - vec3(pos.x, pos.y + 1.0, pos.z)).length();
    // player_tr.rotation = Quat::from_rotation_arc(Vec3::Z, -direction);
    // let mut ttt = player_tr.clone();
    // let start = ttt.rotation.to_euler(EulerRot::XYZ).1;
    // ttt.rotation = Quat::from_rotation_arc(Vec3::Z, -direction);
    // let end = ttt.rotation.to_euler(EulerRot::XYZ).1;

    // info!(
    //     "start, end {} {} {}",
    //     start.to_degrees(),
    //     end.to_degrees(),
    //     end.to_degrees() - start.to_degrees()
    // );

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
fn read_result_system(
    mut commands: Commands,
    controllers: Query<(Entity, &KinematicCharacterControllerOutput)>,
    mut q_contoller: Query<&mut KinematicCharacterController>,
    time: Res<Time>,
) {
    for (entity, output) in controllers.iter() {
        // println!(
        //     "Entity {:?} moved by {:?} and touches the ground: {:?}",
        //     entity, output.effective_translation, output.grounded
        // );
        let Ok(mut controller) = q_contoller.get_mut(entity) else {
            return;
        };
        // info!("asdf");
        // if let Some(_) = controller.translation {
        //     info!(" some !");
        // } else {
        //     info!(" none !");
        // }
        // controller.translation = Some(vec3(0., -9.8, 0.) * time.delta_secs());
        // if let Some(mut trs) = controller.translation {
        //     info!("minus gracyt");

        //     trs.y -= 9.8 * time.delta_secs();
        // }
    }
}

fn character_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &mut Transform,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
) {
    const GRAVITY: f32 = -9.8; // 중력 가속도
    const MOVE_SPEED: f32 = 5.0; // 이동 속도
    const JUMP_FORCE: f32 = 5.0; // 점프 속도

    for (mut controller, mut transform, output) in query.iter_mut() {
        let mut movement = Vec3::ZERO;

        // WASD 키 입력 처리
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }

        // 방향 벡터 정규화
        if movement.length_squared() > 0.0 {
            movement = movement.normalize() * MOVE_SPEED * time.delta_secs();
        }

        // 중력 적용
        let mut velocity = Vec3::new(movement.x, 0.0, movement.z);
        if let Some(output) = output {
            if !output.grounded {
                // 바닥에 닿아있지 않을 때
                velocity.y += GRAVITY;
            }
        }

        // 점프 처리
        // if let Some(output) = output {
        if keyboard_input.just_pressed(KeyCode::Space) {
            velocity.y = JUMP_FORCE;
        }
        // }

        // 이동 벡터 설정
        controller.translation = Some(velocity);
    }
}

fn player_movement(
    time: Res<Time>,
    mut commands: Commands,
    q_flag: Query<(Entity, &MoveFlag)>,
    mut q_player: Query<
        (
            Entity,
            &mut Transform,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
    mut q_player_ani_state: Query<(Entity, &mut PlayerAnimationState), With<PlayerAnimationState>>,
) {
    //
    let Ok((player_entity, mut player_tr, mut controller, output)) = q_player.get_single_mut()
    else {
        return;
    };

    if let Ok((flag_entity, MoveFlag(flag_pos))) = q_flag.get_single() {
        let speed = 3.;

        let sub = vec3(flag_pos.x, player_tr.translation.y, flag_pos.z) - player_tr.translation;
        let mut direction = (sub).normalize();
        let distance = (sub).length();
        // let step = speed * time.delta_secs();
        direction = direction * speed * time.delta_secs();
        let mut velocity = Vec3::new(direction.x, 0.0, direction.z);
        if let Some(output) = output {
            if !output.grounded {
                // 바닥에 닿아있지 않을 때
                velocity.y += (-9.8) * time.delta_secs();
            }
        }
        // info!("driect {}", direction);
        controller.translation = Some(velocity);

        // commands
        //     .entity(player_entity)
        //     .insert(Velocity::linear(-direction * 2.));
        // velocity.linvel = direction * 2.;

        // tr.translation = tr.translation - (direction * step);
        // info!("tr.translation {}", tr.translation);
        // info!("distance {}", distance);
        if distance <= 0.1 {
            commands.entity(flag_entity).despawn_recursive();
            // commands.entity(player_entity).insert(Velocity::zero());
            // velocity.linvel = vec3(0., 0., 0.);
            // velocity.angvel = vec3(0., 0., 0.);
            // controller.translation = None;
            for (_entity, mut state) in &mut q_player_ani_state {
                *state = PlayerAnimationState::Idle;
            }
        }
    } else {
        let mut velocity = Vec3::new(-0., 0.0, 0.);
        if let Some(output) = output {
            if !output.grounded {
                // 바닥에 닿아있지 않을 때
                velocity.y += (-9.8) * time.delta_secs();
            }
        }
        controller.translation = Some(velocity);
    };
}
fn player_rotation(
    mut commands: Commands,
    q_flag: Query<(Entity, &MoveFlag)>,
    mut q_player: Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    //
    let Ok((flag_entity, MoveFlag(flag_pos))) = q_flag.get_single() else {
        return;
    };
    let Ok((player_entity, mut player_tr, mut velocity)) = q_player.get_single_mut() else {
        return;
    };
    let speed = 1.;
    let sub = vec3(flag_pos.x, player_tr.translation.y, flag_pos.z) - player_tr.translation;
    let direction = (sub).normalize();
    let distance = (sub).length();
    let step = speed * time.delta_secs();

    // let mut endrot = tr.clone();
    // endrot.rotation = Quat::from_rotation_arc(Vec3::Z, -direction);

    // info!(
    //     "player tr rot: {:?}, end rot; {:?}",
    //     tr.rotation, endrot.rotation
    // );
    // let e1 = tr.rotation.to_euler(EulerRot::XYX);
    // let e2: (f32, f32, f32) = endrot.rotation.to_euler(EulerRot::XYX);

    // info!("e1 e2 tr rot: {:?}, end rot; {:?}", e1.1, e2.1);
    // info!("e2 - e1: {}", e2.1 - e1.1);

    // let up = Vec3::Y;
    let z = Quat::from_rotation_arc(Vec3::Z, direction);

    player_tr.rotation = player_tr.rotation.slerp(z, time.delta_secs() * 10.);

    // let tween = Tween::new(
    //     EaseFunction::QuarticInOut,
    //     Duration::from_millis(250),
    //     TransformRotationLens {
    //         start: player_tr.rotation,
    //         end: z,
    //     },
    // );

    // commands.entity(player_entity).insert(Animator::new(tween));
    // player_tr.rotation = z;

    // let z = Quat::from_rotation_arc(Vec3::Z, -direction);

    // if !(e2.1 - e1.1 < 0.1) {
    //     velocity.angvel = vec3(0., 1., 0.);
    // }
    // velocity.angvel = vec3(0., 0.5, 0.);
    // tr.rotate_y(PI / 10.);

    // commands
    //     .entity(player_entity)
    //     .insert(Velocity::angular(vec3(0., 11., 0.)));

    // commands.entity(player_entity).insert(Rotation(z));
}
fn camera_movement(
    mut commands: Commands,
    mut q_camera3d: Query<(Entity, &mut Transform), (With<MyCamera3d>, Without<Player>)>,
    q_player: Query<(Entity, &Transform), With<Player>>,
) {
    let Ok((_, mut c_tr)) = q_camera3d.get_single_mut() else {
        return;
    };
    let Ok((_, p_tr)) = q_player.get_single() else {
        return;
    };

    //-12.5, 14.5, 19.0
    c_tr.translation = vec3(
        p_tr.translation.x - 12.5 * 1.,
        p_tr.translation.y + 14.5 * 1.,
        p_tr.translation.z + 19.0 * 1.,
    );
}

#[derive(Component, Debug, Reflect, PartialEq, Eq)]
pub enum PlayerAnimationState {
    Idle,
    Walking,
    Punch,
}

#[derive(Component, Debug, Reflect, PartialEq, Eq)]
pub enum MonsterBeeAnimationState {
    Flying,
    Walking,
}
#[derive(Component, Debug, Reflect, PartialEq, Eq)]
pub enum MonsterEmenyAnimationState {
    Idle,
    Walking,
}
pub fn player_ani_changed(
    mut commands: Commands,
    mut q_player_ani: Query<
        (Entity, &PlayerAnimationState, &mut AnimationTransitions),
        Changed<PlayerAnimationState>,
    >,
    animations: Res<PlayerAnimations>,
    mut q_animation_player: Query<&mut AnimationPlayer>,
) {
    for (entity, state, mut transitions) in &mut q_player_ani {
        info!("state:{:?}", state);
        let Ok(mut player) = q_animation_player.get_mut(entity) else {
            return;
        };
        let str = match state {
            PlayerAnimationState::Idle => "Idle",
            PlayerAnimationState::Punch => "Punch",
            _ => "Walk",
        };
        // player.stop_all();
        info!("str: {str}");
        // let mut transitions = AnimationTransitions::new();
        transitions
            .play(
                &mut player,
                animations.animations[str],
                Duration::from_millis(250),
            )
            .repeat();
    }
}
pub fn monster_bee_ani_changed(
    mut commands: Commands,
    mut q_player_ani: Query<
        (Entity, &MonsterBeeAnimationState, &mut AnimationTransitions),
        Changed<MonsterBeeAnimationState>,
    >,
    animations: Res<MonsterBeeAnimations>,
    mut q_animation_player: Query<&mut AnimationPlayer>,
) {
    for (entity, state, mut transitions) in &mut q_player_ani {
        info!("state:{:?}", state);
        let Ok(mut bee) = q_animation_player.get_mut(entity) else {
            return;
        };
        let str = match state {
            MonsterBeeAnimationState::Flying => "Bee_Flying",
            _ => "Walk",
        };
        // player.stop_all();
        info!("str: {str}");
        // let mut transitions = AnimationTransitions::new();
        transitions
            .play(
                &mut bee,
                animations.animations[str],
                Duration::from_millis(250),
            )
            .repeat();
    }
}
