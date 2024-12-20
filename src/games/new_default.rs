use crate::app::{
    component::{MyCamera2d, MyCamera3d},
    state::MyAppState,
};
use bevy::{
    color::palettes::css,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct MyNewDefaultGamePlugin;

impl Plugin for MyNewDefaultGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyAppState::DefaultScene), (setup));

        app.add_systems(
            FixedUpdate,
            (player_control,).run_if(in_state(MyAppState::DefaultScene)),
        );
        app.add_systems(
            Update,
            (player_look).run_if(in_state(MyAppState::DefaultScene)),
        );

        app.add_plugins(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_my_camera_3d: Query<Entity, With<MyCamera3d>>,
) {
    let Ok(camera_entity) = q_my_camera_3d.get_single() else {
        return;
    };
    /*
     * Stairs
     */
    let stair_len = 30;
    let stair_step = 0.2;
    for i in 1..=stair_len {
        let step = i as f32;
        let collider = Collider::cuboid(1.0, step * stair_step, 1.0);
        commands.spawn((
            Transform::from_xyz(40.0, step * stair_step, step * 2.0 - 20.0),
            Mesh3d(meshes.add(Cuboid::new(2., step * stair_step * 2., 2.))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(-40.0, step * stair_step, step * -2.0 + 20.0),
            Mesh3d(meshes.add(Cuboid::new(2., step * stair_step * 2., 2.))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(step * 2.0 - 20.0, step * stair_step, 40.0),
            Mesh3d(meshes.add(Cuboid::new(2., step * stair_step * 2., 2.))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(step * -2.0 + 20.0, step * stair_step, -40.0),
            Mesh3d(meshes.add(Cuboid::new(2., step * stair_step * 2., 2.))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
            collider.clone(),
        ));
    }
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(200., 1., 200.))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
        Transform::default(),
        RigidBody::Fixed,
        Collider::cuboid(100., 0.5, 100.),
    ));
    let input_map = InputMap::default()
        .with_dual_axis(
            Action::Move,
            // Define a virtual D-pad using four arbitrary buttons.
            VirtualDPad::wasd(),
        )
        .with_dual_axis(Action::Pan, MouseMove::default())
        .with(Action::Jump, KeyCode::Space)
        .with(Action::Run, KeyCode::ShiftLeft);
    let player = commands
        .spawn((
            Name::new("Player"),
            InputManagerBundle::with_map(input_map),
            Player,
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                custom_mass: Some(5.0),
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                slide: true,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(0.3),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: false,
                }),
                // Don’t allow climbing slopes larger than 45 degrees.
                max_slope_climb_angle: 45.0_f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30.0_f32.to_radians(),
                apply_impulse_to_dynamic_bodies: true,
                snap_to_ground: None,
                ..default()
            },
            PlayerStatus {
                is_jump: false,
                is_run: false,
                is_grounded: false,
            },
            // Collider::round_cylinder(0.9, 0.3, 0.2),
            // Ccd { enabled: true },
            Collider::capsule_y(0.5, 0.5),
            Mesh3d(meshes.add(Capsule3d::default())),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
            Transform::from_xyz(0., 2., 0.),
        ))
        .id();

    commands
        .entity(camera_entity)
        .insert(CameraRotation { pitch: 0., yaw: 0. })
        .insert(Transform::from_xyz(0., 1., 6.))
        .set_parent(player);
}
// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    #[actionlike(DualAxis)]
    Move,
    Jump,
    Run,
    #[actionlike(DualAxis)]
    Pan,
}

// Query for the `ActionState` component in your game logic systems!
fn player_control(
    time: Res<Time>,
    mut q_player: Query<
        (
            Entity,
            &mut PlayerStatus,
            &ActionState<Action>,
            &mut Transform,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        (With<Player>, Without<MyCamera3d>),
    >,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
) {
    let Ok((
        player_entity,
        mut player_status,
        action_state,
        mut player_transform,
        mut controller,
        output,
    )) = q_player.get_single_mut()
    else {
        return;
    };

    const GRAVITY: f32 = -9.81; // 중력 가속도
    let mut move_speed: f32 = 5.0; // 이동 속도
    const JUMP_FORCE: f32 = 5.0; // 점프 속도
    let mut movement = Vec3::ZERO;
    if action_state.just_pressed(&Action::Run) {
        player_status.is_run = true;
        // info!("run");
    }
    if action_state.just_released(&Action::Run) {
        player_status.is_run = false;
        // info!("run release");
    }

    if action_state.axis_pair(&Action::Move) != Vec2::ZERO {
        let axis_pair = action_state.axis_pair(&Action::Move);

        let camera_forward = player_transform.forward();
        let camera_right = player_transform.right();

        if player_status.is_run {
            move_speed *= 4.;
        }

        if axis_pair.x > 0. {
            // info!("righ");
            movement += *camera_right * move_speed;
        }

        if axis_pair.x < 0. {
            // info!("left");
            movement -= *camera_right * move_speed;
            //
        }

        if axis_pair.y > 0. {
            // info!("forwrad");
            movement += *camera_forward * move_speed;
        }

        if axis_pair.y < 0. {
            // info!("back");
            movement -= *camera_forward * move_speed;
            //
        }
    }
    let delta_time = time.delta_secs();
    let jump_speed = if action_state.just_pressed(&Action::Jump) {
        20.
    } else {
        0.
    };
    // info!("y: {}", movement.y);
    if output.map(|o| o.grounded).unwrap_or(false) {
        *grounded_timer = 0.5;
        *vertical_movement = 0.0;
    }
    if *grounded_timer > 0.0 {
        *grounded_timer -= delta_time;
        // If we jump we clear the grounded tolerance
        if jump_speed > 0.0 {
            *vertical_movement = jump_speed;
            *grounded_timer = 0.0;
        }
    }

    // if player_status.is_jump {
    //     movement.y = 13.;
    // }
    movement.y = *vertical_movement;
    *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
    info!("v {:?}", vertical_movement);

    controller.translation = Some(movement * delta_time);
}

fn player_look(
    time: Res<Time>,
    mut q_player: Query<
        (
            Entity,
            &mut PlayerStatus,
            &ActionState<Action>,
            &mut Transform,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        (With<Player>, Without<MyCamera3d>),
    >,
    mut q_my_camera_3d: Query<
        (Entity, &mut Transform, &mut CameraRotation),
        (With<MyCamera3d>, Without<Player>),
    >,
) {
    let Ok((
        player_entity,
        mut player_status,
        action_state,
        mut player_transform,
        mut controller,
        output,
    )) = q_player.get_single_mut()
    else {
        return;
    };
    let Ok((camera_entity, mut camera_transform, mut camera_rotation)) =
        q_my_camera_3d.get_single_mut()
    else {
        return;
    };
    if action_state.axis_pair(&Action::Pan) != Vec2::ZERO {
        // info!("pan");
        let sensitivity = 0.1; // 마우스 감도
        let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.01;

        let axis_pair = action_state.axis_pair(&Action::Pan);
        let delta_x = axis_pair.x;
        let delta_y = axis_pair.y;
        camera_rotation.yaw -= delta_x * sensitivity * time.delta_secs();
        camera_rotation.pitch -= delta_y * sensitivity * time.delta_secs();
        camera_rotation.pitch = camera_rotation.pitch.clamp(-pitch_limit, pitch_limit);
        camera_transform.rotation = Quat::from_axis_angle(Vec3::X, camera_rotation.pitch);
        // camera_transform.rotation = Quat::from_euler(
        //     EulerRot::YXZ, // 회전 순서: Yaw -> Pitch
        //     // camera_rotation.yaw,   // Yaw
        //     0.,
        //     camera_rotation.pitch, // Pitch
        //     0.0,                   // Roll은 항상 0
        // );

        player_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_rotation.yaw);
    }
}

#[derive(Component, Debug)]
pub struct PlayerStatus {
    pub is_jump: bool,
    pub is_run: bool,
    pub is_grounded: bool,
}

#[derive(Component)]
struct PlayerJumpTimer(Timer);

#[derive(Component)]
pub struct CameraRotation {
    pub yaw: f32,   // Y축 회전 (Yaw)
    pub pitch: f32, // X축 회전 (Pitch)
}

fn player_jump_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player_jump_timer: Query<(Entity, &mut PlayerJumpTimer)>,
    mut q_player_status: Query<&mut PlayerStatus>,
) {
    for (entity, mut timer) in &mut q_player_jump_timer {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            for mut ps in &mut q_player_status {
                ps.is_jump = false;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
