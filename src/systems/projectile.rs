use bevy::prelude::*;
use crate::components::*;

pub fn projectile_system(
    time: Res<Time>,
    mut query: Query<(&Projectile, &mut Transform, &Speed, &MoveDirection)>,
) {
    for (_, mut transform, Speed(speed), MoveDirection(direction)) in &mut query {
        transform.translation += *direction * *speed * time.delta_seconds();
    }
}

