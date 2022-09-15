mod collider;
mod enemy;
mod player;
mod projectile;
mod weapon;

pub use collider::*;
pub use enemy::*;
pub use player::*;
pub use projectile::*;
pub use weapon::*;

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

#[derive(Component)]
pub struct StartScreenText;

#[derive(Component)]
pub struct LoseScreenText;

#[derive(Component)]
pub struct WinScreenText;
