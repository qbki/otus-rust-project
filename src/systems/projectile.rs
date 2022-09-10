use bevy::prelude::*;
use crate::components::*;

pub fn projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile, &mut Transform, &Speed, &MoveDirection)>,
) {
    for (entity, mut projectile, mut transform, Speed(speed), MoveDirection(direction)) in &mut query {
        let delta_time = time.delta_seconds();
        transform.translation += *direction * *speed * delta_time;
        projectile.life_time -= delta_time;
        if projectile.life_time <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

