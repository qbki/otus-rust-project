use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn projectile_system(
    mut commands: Commands,
    time: Res<Elapsed>,
    mut query: Query<(Entity, &mut Projectile, &mut Transform, &Speed, &MoveDirection)>,
) {
    if time.is_paused {
        return;
    }

    for (entity, mut projectile, mut transform, Speed(speed), MoveDirection(direction)) in &mut query {
        transform.translation += *direction * *speed * time.seconds;
        projectile.life_time -= time.seconds;
        if projectile.life_time <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_hit_actor_system(
    mut commands: Commands,
    projectiles_query: Query<(Entity, &Transform), With<Projectile>>,
    mut agents_query: Query<(Entity, &mut Transform, &Collider, &mut Lives), Without<Projectile>>,
) {
    for (projectile_entity, projectile_transform) in &projectiles_query {
        for (agent_entity, mut agent_transform, collider, mut lives) in &mut agents_query{
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

pub fn projectile_hit_wall_system(
    mut commands: Commands,
    projectiles_query: Query<(Entity, &Transform), With<Projectile>>,
    walls_query: Query<(&Wall, &Transform, &Collider), Without<Projectile>>,
) {
    for (projectile_entity, projectile_transform) in &projectiles_query {
        for (_, wall_transform, collider) in &walls_query{
            let has_hit = collider.hit_test(&wall_transform.translation, &projectile_transform.translation);
            if has_hit {
                commands.entity(projectile_entity).despawn();
                return;
            }
        }
    }
}
