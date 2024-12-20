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
            Update,
            (player_control, player_jump_timer).run_if(in_state(MyAppState::DefaultScene)),
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

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(20., 1., 20.))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
        Transform::default(),
        RigidBody::Fixed,
        Collider::cuboid(10., 0.5, 10.),
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
                offset: CharacterLength::Relative(0.01),
                ..default()
            },
            PlayerStatus {
                is_jump: false,
                is_run: false,
            },
            Ccd { enabled: true },
            Collider::capsule_y(0.5, 0.5),
            Mesh3d(meshes.add(Capsule3d::default())),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(css::GRAY))),
            Transform::from_xyz(0., 3., 0.),
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
    mut commands: Commands,
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
    const GRAVITY: f32 = -9.8; // 중력 가속도
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
            movement += *camera_right * move_speed * time.delta_secs();
        }

        if axis_pair.x < 0. {
            // info!("left");
            movement -= *camera_right * move_speed * time.delta_secs();
            //
        }

        if axis_pair.y > 0. {
            // info!("forwrad");
            movement += *camera_forward * move_speed * time.delta_secs();
        }

        if axis_pair.y < 0. {
            // info!("back");
            movement -= *camera_forward * move_speed * time.delta_secs();
            //
        }
    }
    if action_state.just_pressed(&Action::Jump) {
        if let Some(output) = output {
            if output.grounded {
                player_status.is_jump = true;
                commands.spawn(PlayerJumpTimer(Timer::from_seconds(1., TimerMode::Once)));
                // info!("jump!");
            }
        }
    }

    if player_status.is_jump {
        movement.y = 13. * time.delta_secs();
    }

    movement.y += GRAVITY * time.delta_secs();

    controller.translation = Some(movement);

    if action_state.axis_pair(&Action::Pan) != Vec2::ZERO {
        // info!("pan");
        let sensitivity = 0.1; // 마우스 감도
        let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.01;

        let axis_pair = action_state.axis_pair(&Action::Pan);
        let mut delta_x = axis_pair.x;
        let mut delta_y = axis_pair.y;
        camera_rotation.yaw -= delta_x * sensitivity * time.delta_secs();
        camera_rotation.pitch -= delta_y * sensitivity * time.delta_secs();
        camera_rotation.pitch = camera_rotation.pitch.clamp(-pitch_limit, pitch_limit);
        camera_transform.rotation = Quat::from_euler(
            EulerRot::YXZ, // 회전 순서: Yaw -> Pitch
            // camera_rotation.yaw,   // Yaw
            0.,
            camera_rotation.pitch, // Pitch
            0.0,                   // Roll은 항상 0
        );

        player_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_rotation.yaw);
    }
}
#[derive(Component, Debug)]
pub struct PlayerStatus {
    pub is_jump: bool,
    pub is_run: bool,
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
