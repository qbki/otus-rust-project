mod enemy;
mod player;
mod weapon;
mod projectile;

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
pub struct MoveDirection(pub Vec3);
