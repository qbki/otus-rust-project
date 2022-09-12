mod collider;
mod enemy;
mod player;
mod weapon;
mod projectile;

pub use collider::*;
pub use enemy::*;
pub use player::*;
pub use weapon::*;
pub use projectile::*;

use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct RotationSpeed(pub f32);

#[derive(Component)]
pub struct Lives(pub i32);

#[derive(Component)]
pub struct MoveDirection(pub Vec3);

#[derive(Component)]
pub struct Wall;
