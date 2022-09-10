mod player;
mod weapon;

use bevy::prelude::{Component, Vec3};

pub use player::Player;
pub use weapon::*;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct MoveDirection(pub Vec3);

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Enemy;
