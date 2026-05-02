use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::player::Player;

const PITCH_MIN: f32 = -0.1;
const PITCH_MAX: f32 = 1.2;
const SENSITIVITY: f32 = 0.003;
const PLAYER_EYE_HEIGHT: f32 = 1.2;

#[derive(Resource)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
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

#[derive(Component)]
pub struct FollowCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_orbit);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        FollowCamera,
        Camera3d::default(),
        Transform::IDENTITY,
    ));
    commands.insert_resource(CameraState::default());
}

fn camera_orbit(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut cam_state: ResMut<CameraState>,
    player_q: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut cam_q: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
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
