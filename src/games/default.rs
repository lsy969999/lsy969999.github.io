use crate::{
    app::{
        component::{MyCamera2d, MyCamera3d},
        state::MyAppState,
    },
    asset::resource::{BaseAssets, DefaultSceneAssets},
};
use bevy::{
    color::palettes::css,
    math::vec3,
    prelude::*,
    render::{
        mesh::{skinning::SkinnedMesh, MeshAabb},
        view::{NoFrustumCulling, RenderLayers},
    },
};
use bevy_rapier3d::prelude::*;
use std::{collections::HashMap, time::Duration};

pub struct MyDefaultGamePlugin;

impl Plugin for MyDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        // app.init_state::<PlayerAniState>();
        app.add_systems(OnEnter(MyAppState::DefaultScene), new_setup);
        // app.add_systems(OnEnter(MyAppState::DefaultScene), fox_spawn.after(setup));
        app.add_systems(OnExit(MyAppState::DefaultScene), clear_scene);
        app.add_systems(
            Update,
            (
                draw_gizmo,
                draw_axes_helper_gizmo,
                player_movement,
                player_rotation,
                camera_movement,
                added_default_scene,
                player_ani_changed,
                added_mesh3d_picking,
                disable_culling_for_skinned_meshes,
                monster_text,
            )
                .run_if(in_state(MyAppState::DefaultScene)),
        );
        app.register_type::<PlayerAnimationState>();
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
            "Icosphere" => {
                for child_entity in children {
                    let Ok((Mesh3d(mesh_handle), name)) = q_mesh.get(*child_entity) else {
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
            "MonsterArmature_Bee" => {
                commands.entity(entity).insert(MonsterBee);
            }
            "CharacterArmature" => {
                let Ok(mut player) = q_animation_player.get_mut(entity) else {
                    return;
                };

                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(&mut player, animations.animations["Idle"], Duration::ZERO)
                    .repeat();
                commands
                    .entity(entity)
                    .insert(Transform::from_xyz(0., -1.5, 0.))
                    .insert(AnimationGraphHandle(animations.graph.clone()))
                    .insert(PlayerAnimationState::Idle)
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
                    .insert(Collider::cylinder(1.5, 1.))
                    // .insert(Collider::cuboid(1., 2., 1.))
                    .insert(RigidBody::Dynamic)
                    .insert(LockedAxes::ROTATION_LOCKED_X)
                    .insert(LockedAxes::ROTATION_LOCKED_Z)
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
pub struct Animations {
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
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_def_scene: Query<Entity, With<DefaultScene>>,
    q_flag: Query<(Entity, &MoveFlag)>,
    q_player: Query<(Entity, &Transform), With<Player>>,
    mut q_player_ani_state: Query<(Entity, &mut PlayerAnimationState), With<PlayerAnimationState>>,
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

    for (_entity, mut state) in &mut q_player_ani_state {
        *state = PlayerAnimationState::Walking;
    }

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
    mut q_player_ani_state: Query<(Entity, &mut PlayerAnimationState), With<PlayerAnimationState>>,
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
    // info!("distance {}", distance);
    if distance <= 0.1 {
        commands.entity(flag_entity).despawn_recursive();
        commands.entity(player_entity).insert(Velocity::zero());
        for (_entity, mut state) in &mut q_player_ani_state {
            *state = PlayerAnimationState::Idle;
        }
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
        p_tr.translation.x - 12.5,
        p_tr.translation.y + 14.5,
        p_tr.translation.z + 19.0,
    );
}

#[derive(Component, Debug, Reflect)]
pub enum PlayerAnimationState {
    Idle,
    Walking,
}
pub fn player_ani_changed(
    mut commands: Commands,
    q_player_ani: Query<(Entity, &PlayerAnimationState), Changed<PlayerAnimationState>>,
    animations: Res<Animations>,
    mut q_animation_player: Query<&mut AnimationPlayer>,
) {
    for (entity, state) in &q_player_ani {
        info!("state:{:?}", state);
        let Ok(mut player) = q_animation_player.get_mut(entity) else {
            return;
        };
        let str = match state {
            PlayerAnimationState::Idle => "Idle",
            _ => "Walk",
        };
        player.stop_all();
        info!("str: {str}");
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(&mut player, animations.animations[str], Duration::ZERO)
            .repeat();
        commands
            .entity(entity)
            // .insert(Transform::from_xyz(0., -1.5, 0.))
            // .insert(AnimationGraphHandle(animations.graph.clone()))
            // .insert(PlayerAnimationState::Idle)
            .insert(transitions);
    }
}
