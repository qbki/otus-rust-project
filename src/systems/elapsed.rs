use crate::resources::*;
use bevy::prelude::*;

pub fn elapsed_system(mut elapsed: ResMut<Elapsed>, time: Res<Time>, game_state: Res<GameState>) {
    match game_state.screen {
        ScreenEnum::Game => {
            elapsed.seconds = time.delta_seconds();
            elapsed.is_paused = false;
        }
        _ => {
            elapsed.seconds = 0.0;
            elapsed.is_paused = true;
        }
    }
}
