mod resources;
mod math;
mod components;
mod systems;
mod consts;

use std::f32::consts::PI;
use bevy::prelude::*;
use components::*;
use consts::*;
use systems::*;
use resources::*;

fn player_control_system(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform, &Speed, &mut Weapons)>,
    control: Res<Control>,
) {
    let (player, mut player_transform, Speed(speed), mut weapons) = query.single_mut();

    if let Some(hit_point) = player.plane.hit_test(&control.cursor_ray) {
        let angle_vector = hit_point - player_transform.translation;
        let angle = angle_vector.y.atan2(angle_vector.x);
        if angle.is_normal() {
            let mut matrix = Transform::from_translation(player_transform.translation);
            matrix.rotate_z(angle - PI * 0.5);
            *player_transform = matrix;
        }
        for mut weapon in weapons.0.iter_mut() {
            weapon.direction = angle_vector.normalize_or_zero();
            weapon.is_shooting = control.is_shooting;
        }
    }

    player_transform.translation += control.direction_normal * (time.delta_seconds() * *speed);
}

fn enemy_system(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform, &RotationSpeed, &mut Weapons)>,
) {
    let (mut enemy, mut transform, RotationSpeed(rotation_speed), mut weapons) = query.single_mut();
    enemy.angle += time.delta_seconds() * *rotation_speed;
    let mut new_transform = Transform::from_translation(transform.translation);
    new_transform.rotate_z(enemy.angle);
    *transform = new_transform;

    let length = weapons.0.len();
    for (i, mut weapon) in weapons.0.iter_mut().enumerate() {
        let angle = enemy.angle + (PI * 2.0 / (length as f32)) * (i as f32);
        weapon.direction = Vec3::new(
            f32::cos(angle),
            f32::sin(angle),
            0.0
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut handlers: ResMut<Handlers>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    });

    let projectile_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.1, 0.1),
        ..default()
    });
    handlers.materials.insert(PROJECTILE_MATERIAL.to_string(), projectile_material_handle);

    let ship_mesh = asset_server.load("ship.glb#Mesh0/Primitive0");
    let enemy_mesh = asset_server.load("enemy.glb#Mesh0/Primitive0");
    let projectile_mesh = asset_server.load("projectile.glb#Mesh0/Primitive0");
    handlers.meshes.insert(PROJECTILE_MESH.to_string(), projectile_mesh);

    commands
        .spawn_bundle(PbrBundle {
            mesh: ship_mesh,
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Player::new())
        .insert(Weapons(vec![Weapon {
            speed: 20.0,
            muzzle_distance: 1.2,
            ..default()
        }]))
        .insert(Speed(10.0));

    let enemy_weapon = Weapon {
        max_time: 0.5,
        is_shooting: true,
        speed: 4.0,
        muzzle_distance: 1.7,
        ..default()
    };
    let enemy_weapons = vec![
        Weapon { direction: Vec3::new(0.0, 1.0, 0.0), ..enemy_weapon },
        Weapon { direction: Vec3::new(0.0, -1.0, 0.0), ..enemy_weapon },
        Weapon { direction: Vec3::new(1.0, 0.0, 0.0), ..enemy_weapon },
        Weapon { direction: Vec3::new(-1.0, 0.0, 0.0), ..enemy_weapon },
    ];

    commands
        .spawn_bundle(PbrBundle {
            mesh: enemy_mesh,
            material: material_handle.clone(),
            transform: Transform::from_xyz(4.0, 0.0, 0.0),
            ..default()
        })
        .insert(Enemy::default())
        .insert(Weapons(enemy_weapons))
        .insert(RotationSpeed(0.5));

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
        .add_system(keyboard_input_system)
        .add_system(mouse_input_system)
        .add_system(player_control_system)
        .add_system(enemy_system)
        .add_system(projectile_system)
        .add_system(weapon_spawn_projectile)
        .insert_resource(Control::new())
        .insert_resource(Handlers::new())
        .run();
}
