use avian3d::prelude::*;
use bevy::prelude::*;

use crate::camera::CameraState;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Player { speed: 5.0 },
        RigidBody::Dynamic,
        Collider::capsule(0.4, 0.9),
        LockedAxes::new().lock_rotation_x().lock_rotation_y().lock_rotation_z(),
        Mesh3d(meshes.add(Cylinder::new(0.4, 1.8))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.9, 0.0),
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    cam_state: Res<CameraState>,
    mut player_q: Query<(&Player, &mut LinearVelocity, &mut Transform)>,
) {
    let Ok((player, mut velocity, mut transform)) = player_q.single_mut() else { return };

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
        velocity.x = direction.x * player.speed;
        velocity.z = direction.z * player.speed;

        let target = Transform::IDENTITY.looking_to(-direction, Vec3::Y).rotation;
        transform.rotation = target;
    } else {
        velocity.x = 0.0;
        velocity.z = 0.0;
    }
}
