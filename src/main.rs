mod io;
mod math;
mod components;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::prelude::shape::UVSphere;
use components::{Player, Speed};
use components::Enemy;

fn player_control_system(
    time: Res<Time>,
    mut player: Query<(&Player, &mut Transform, &Speed)>,
    control: Res<io::Control>,
) {
    let (player, mut player_transform, Speed(speed)) = player.single_mut();

    if let Some(hit_point) = player.plane.hit_test(&control.cursor_ray) {
        let angle_vector = hit_point - player_transform.translation;
        let angle = angle_vector.y.atan2(angle_vector.x);
        if angle.is_normal() {
            let mut matrix = Transform::from_translation(player_transform.translation);
            matrix.rotate_z(angle - PI * 0.5);
            *player_transform = matrix;
        }
    }

    player_transform.translation += control.direction_normal * (time.delta_seconds() * *speed);
}

fn enemy_system(
    time: Res<Time>,
    mut player: Query<(&Enemy, &mut Transform, &Speed)>,
) {
    let (_, mut transform, Speed(speed)) = player.single_mut();
    transform.rotate_z(time.delta_seconds() * *speed);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    });

    let ship_mesh = asset_server.load("ship.glb#Mesh0/Primitive0");
    let enemy_mesh = asset_server.load("enemy.glb#Mesh0/Primitive0");

    UVSphere

    commands
        .spawn_bundle(PbrBundle {
            mesh: ship_mesh,
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Player::new())
        .insert(Speed(10.0));

    commands
        .spawn_bundle(PbrBundle {
            mesh: enemy_mesh,
            material: material_handle.clone(),
            transform: Transform::from_xyz(4.0, 0.0, 0.0),
            ..default()
        })
        .insert(Enemy)
        .insert(Speed(0.5));

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 4.0),
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(io::keyboard_input_system)
        .add_system(io::mouse_input_system)
        .add_system(player_control_system)
        .add_system(enemy_system)
        .insert_resource(io::Control::new())
        .run();
}
