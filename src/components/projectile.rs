use bevy::prelude::Component;

#[derive(Component)]
pub struct Projectile {
    pub life_time: f32, // seconds
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            life_time: 7.0,
        }
    }
}
