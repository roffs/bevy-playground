use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, camera_orbit, cursor_grab))
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Resource)]
struct CameraState {
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.4,
            distance: 10.0,
        }
    }
}

const PITCH_MIN: f32 = -0.1;
const PITCH_MAX: f32 = 1.2;
const SENSITIVITY: f32 = 0.003;
const PLAYER_EYE_HEIGHT: f32 = 1.2;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // Reference cubes
    for (x, z) in [(-5.0f32, -5.0f32), (5.0, -5.0), (-5.0, 5.0), (5.0, 5.0)] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(180, 120, 80))),
            Transform::from_xyz(x, 0.5, z),
        ));
    }

    // Player cylinder
    commands.spawn((
        Player { speed: 5.0 },
        Mesh3d(meshes.add(Cylinder::new(0.4, 1.8))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.9, 0.0),
    ));

    // Camera (position set by camera_orbit each frame)
    commands.spawn((
        Camera3d::default(),
        Transform::IDENTITY,
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));

    commands.spawn(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        affects_lightmapped_meshes: true,
    });

    commands.insert_resource(CameraState::default());
}

fn camera_orbit(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut cam_state: ResMut<CameraState>,
    player_q: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        delta += ev.delta;
    }

    if delta != Vec2::ZERO {
        cam_state.yaw -= delta.x * SENSITIVITY;
        cam_state.pitch = (cam_state.pitch - delta.y * SENSITIVITY).clamp(PITCH_MIN, PITCH_MAX);
    }

    let Ok(player_tf) = player_q.single() else { return };
    let Ok(mut cam_tf) = cam_q.single_mut() else { return };

    let player_pos = player_tf.translation;

    // Direction from player to camera (spherical coordinates)
    let dir = Vec3::new(
        cam_state.yaw.sin() * cam_state.pitch.cos(),
        cam_state.pitch.sin(),
        cam_state.yaw.cos() * cam_state.pitch.cos(),
    );

    let cam_pos = player_pos + dir * cam_state.distance;

    *cam_tf = Transform::from_translation(cam_pos).looking_at(
        player_pos + Vec3::Y * PLAYER_EYE_HEIGHT,
        Vec3::Y,
    );
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    cam_state: Res<CameraState>,
    mut player_q: Query<(&Player, &mut Transform)>,
) {
    let Ok((player, mut transform)) = player_q.single_mut() else { return };

    let yaw_rot = Quat::from_rotation_y(cam_state.yaw);
    let forward = yaw_rot * Vec3::NEG_Z;
    let right = yaw_rot * Vec3::X;

    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp)    { direction += forward; }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown)  { direction -= forward; }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { direction += right;   }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft)  { direction -= right;   }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * player.speed * time.delta_secs();

        let target = Transform::IDENTITY.looking_to(-direction, Vec3::Y).rotation;
        transform.rotation = transform.rotation.slerp(target, 12.0 * time.delta_secs());
    }
}

fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}
