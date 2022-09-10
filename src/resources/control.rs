use bevy::math::Vec3;
use crate::math::Ray;

pub struct Control {
    pub cursor_ray: Ray,
    pub direction_normal: Vec3,
    pub is_shooting: bool,
}

impl Control {
    pub fn new() -> Self {
        Control {
            cursor_ray: Ray {
                origin: Vec3::ZERO,
                normal: Vec3::new(0.0, 0.0, -1.0),
            },
            direction_normal: Vec3::ZERO,
            is_shooting: false,
        }
    }
}

