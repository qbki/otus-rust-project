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

pub fn projectile_hit_system(
    mut commands: Commands,
    query_projectiles: Query<(Entity, &Transform), With<Projectile>>,
    mut query_agents: Query<(Entity, &mut Transform, &Collider, &mut Lives), Without<Projectile>>,
) {
    for (projectile_entity, projectile_transform) in &query_projectiles {
        for (agent_entity, mut agent_transform, collider, mut lives) in &mut query_agents{
            let has_hit = collider.hit_test(&agent_transform.translation, &projectile_transform.translation);
            if has_hit {
                commands.entity(projectile_entity).despawn();
                let new_translation = (agent_transform.translation - projectile_transform.translation).normalize() * 0.2;
                agent_transform.translation += new_translation;
                lives.0 -= 1;
                if lives.0 <= 0 {
                    commands.entity(agent_entity).despawn();
                }
                return;
            }
        }
    }
}
