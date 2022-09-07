use bevy::math::Vec3;
use super::ray::Ray;

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn hit_test(&self, ray: &Ray) -> Option<Vec3> {
        let dot_product = self.normal.dot(ray.normal);
        if dot_product.abs() > f32::EPSILON {
            let t = (self.origin - ray.origin).dot(self.normal) / dot_product;
            if t > 0.0 {
                Some(ray.origin + ray.normal * t)
            } else {
                None
            }
        } else {
            None
        }
    }
}

