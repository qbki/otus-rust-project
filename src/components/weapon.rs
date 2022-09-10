use bevy::prelude::*;

#[derive(Component)]
pub struct Weapons(pub Vec<Weapon>);

#[derive(Copy, Clone)]
pub struct Weapon {
    pub time_left: f32, // seconds
    pub max_time: f32, // seconds
    pub is_shooting: bool,
    pub direction: Vec3,
    pub muzzle_distance: f32,
    pub speed: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            time_left: 0.0,
            max_time: 0.1,
            is_shooting: false,
            direction: Vec3::ONE,
            muzzle_distance: 0.0,
            speed: 1.0,
        }
    }
}
