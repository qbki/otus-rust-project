use bevy::prelude::*;
use crate::components::*;
use crate::consts::*;
use crate::resources::*;

pub fn weapon_spawn_projectile(
    time: Res<Time>,
    mut query: Query<&mut Weapons>,
    mut commands: Commands,
    mut material_handlers: ResMut<Handlers>,
) {
    let elapsed_time = time.delta_seconds();
    for mut weapons in &mut query {
        for weapon in weapons.0.iter_mut() {
            weapon.time_left -= elapsed_time;
            if weapon.time_left <= 0.0 && weapon.is_shooting {
                make_projectile(&mut commands, &mut material_handlers, &weapon);
                weapon.time_left = weapon.max_time;
            }
        }
    }
}

fn make_projectile(
    commands: &mut Commands,
    material_handlers: &mut ResMut<Handlers>,
    weapon: &Weapon,
) {
    let mesh = material_handlers.meshes.get(PROJECTILE_MESH).unwrap();
    let material = material_handlers.materials.get(PROJECTILE_MATERIAL).unwrap();

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(weapon.position),
            ..default()
        })
        .insert(Projectile)
        .insert(MoveDirection(weapon.direction))
        .insert(Speed(20.0));
}
