use bevy::prelude::*;

#[derive(Component)]
pub struct Weapons(pub Vec<Weapon>);

pub struct Weapon {
    pub time_left: f32, // seconds
    pub max_time: f32, // seconds
    pub is_shooting: bool,
    pub direction: Vec3,
    pub position: Vec3,
}

impl Weapon {
    pub fn new() -> Self {
        Weapon {
            time_left: 0.0,
            max_time: 0.1,
            is_shooting: false,
            direction: Vec3::ONE,
            position: Vec3::ZERO,
        }
    }
}
