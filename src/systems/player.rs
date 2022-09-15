use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn player_control_system(
    time: Res<Elapsed>,
    mut query: Query<(&Player, &mut Transform, &Speed, &mut Weapons)>,
    control: Res<Control>,
) {
    if time.is_paused {
        return;
    }

    for (player, mut player_transform, Speed(speed), mut weapons) in &mut query {
        if let Some(hit_point) = player.plane.hit_test(&control.cursor_ray) {
            let angle_vector = hit_point - player_transform.translation;
            let angle = angle_vector.y.atan2(angle_vector.x);
            if angle.is_normal() {
                let mut matrix = Transform::from_translation(player_transform.translation);
                matrix.rotate_z(angle - PI * 0.5);
                *player_transform = matrix;
            }
            for mut weapon in weapons.0.iter_mut() {
                weapon.direction = angle_vector.normalize_or_zero();
                weapon.is_shooting = control.is_shooting;
            }
        }
        player_transform.translation += control.direction_normal * (time.seconds * *speed);
    }
}

pub fn player_lose_system(
    query: Query<&Player>,
    mut event_writer: EventWriter<LoseGameEvent>,
    game_state: Res<GameState>,
) {
    if game_state.screen == ScreenEnum::Game && query.is_empty() {
        event_writer.send(LoseGameEvent);
    }
}

pub fn player_win_system(
    query: Query<&Enemy>,
    mut event_writer: EventWriter<WinGameEvent>,
    game_state: Res<GameState>,
) {
    if game_state.screen == ScreenEnum::Game && query.is_empty() {
        event_writer.send(WinGameEvent);
    }
}
