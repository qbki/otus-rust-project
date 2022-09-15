use crate::components::*;
use crate::resources::Elapsed;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn enemy_system(
    time: Res<Elapsed>,
    mut query: Query<(&mut Enemy, &mut Transform, &RotationSpeed, &mut Weapons)>,
) {
    if time.is_paused {
        return;
    }

    for (mut enemy, mut transform, RotationSpeed(rotation_speed), mut weapons) in &mut query {
        enemy.angle += time.seconds * *rotation_speed;
        let mut new_transform = Transform::from_translation(transform.translation);
        new_transform.rotate_z(enemy.angle);
        *transform = new_transform;

        let length = weapons.0.len();
        for (i, mut weapon) in weapons.0.iter_mut().enumerate() {
            let angle = enemy.angle + (PI * 2.0 / (length as f32)) * (i as f32);
            weapon.direction = Vec3::new(f32::cos(angle), f32::sin(angle), 0.0);
        }
    }
}
