use crate::components::*;
use crate::consts::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn spawn_projectiles_system(
    time: Res<Elapsed>,
    mut query: Query<(&mut Weapons, &Transform)>,
    mut commands: Commands,
    mut material_handlers: ResMut<Handlers>,
) {
    if time.is_paused {
        return;
    }

    for (mut weapons, transform) in &mut query {
        for weapon in weapons.0.iter_mut() {
            weapon.time_left -= time.seconds;
            if weapon.time_left <= 0.0 && weapon.is_shooting {
                make_projectile(&mut commands, &mut material_handlers, weapon, transform);
                weapon.time_left = weapon.max_time;
            }
        }
    }
}

fn make_projectile(
    commands: &mut Commands,
    material_handlers: &mut ResMut<Handlers>,
    weapon: &Weapon,
    transform: &Transform,
) {
    let mesh = material_handlers.meshes.get(PROJECTILE_MESH).unwrap();
    let material = material_handlers
        .materials
        .get(PROJECTILE_MATERIAL)
        .unwrap();

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(
                transform.translation + weapon.muzzle_distance * weapon.direction,
            ),
            ..default()
        })
        .insert(Projectile::default())
        .insert(MoveDirection(weapon.direction))
        .insert(Speed(weapon.speed));
}
