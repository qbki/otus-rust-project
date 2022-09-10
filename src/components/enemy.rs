use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub angle: f32, // radians
}

impl Default for Enemy {
    fn default() -> Self {
        Self { angle: 0.0 }
    }
}
