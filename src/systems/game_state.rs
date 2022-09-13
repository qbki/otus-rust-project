use bevy::prelude::*;
use crate::resources::*;
use crate::events::*;

pub fn game_state_system(
    mut start_game_event: EventReader<StartGameEvent>,
    mut lose_game_event: EventReader<LoseGameEvent>,
    mut win_game_event: EventReader<WinGameEvent>,
    mut game_state: ResMut<GameState>,
) {
    match game_state.screen {
        ScreenEnum::Start => {
            for _ in start_game_event.iter() {
                game_state.screen = ScreenEnum::Game;
            }
        }
        ScreenEnum::Game => {
            for _ in lose_game_event.iter() {
                game_state.screen = ScreenEnum::Lose;
            }
            for _ in win_game_event.iter() {
                game_state.screen = ScreenEnum::Win;
            }
        }
        _ => {}
    }
}
