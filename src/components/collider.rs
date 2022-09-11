use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn hit_test(&self, owner_translation: &Vec3, point: &Vec3) -> bool {
        (*point - *owner_translation).length() <= self.radius
    }
}
