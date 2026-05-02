use avian3d::prelude::*;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_reference_cubes, spawn_lighting));
    }
}

fn spawn_ground(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(20.0, 0.5),
        Mesh3d(meshes.add(Cylinder::new(20.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
}

fn spawn_reference_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    for (x, z) in [(-5.0f32, -5.0f32), (5.0, -5.0), (-5.0, 5.0), (5.0, 5.0)] {
        commands.spawn((
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(180, 120, 80))),
            Transform::from_xyz(x, 0.5, z),
        ));
    }
}

fn spawn_lighting(mut commands: Commands) {
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
}
