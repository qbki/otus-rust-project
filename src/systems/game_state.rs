use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;

fn set_visibility<T: Component>(
    query: &mut Query<(&T, &mut Style)>,
    value: bool,
) {
    for (_, mut style) in query {
        style.display = match value {
            true => Display::Flex,
            false => Display::None,
        }
    }
}

pub fn game_state_system(
    mut start_game_event: EventReader<StartGameEvent>,
    mut lose_game_event: EventReader<LoseGameEvent>,
    mut win_game_event: EventReader<WinGameEvent>,
    mut game_state: ResMut<GameState>,
    mut text_set: ParamSet<(
        Query<(&StartScreenText, &mut Style)>,
        Query<(&WinScreenText, &mut Style)>,
        Query<(&LoseScreenText, &mut Style)>,
    )>,
) {
    match game_state.screen {
        ScreenEnum::Start => {
            for _ in start_game_event.iter() {
                set_visibility(&mut text_set.p0(), false);
                game_state.screen = ScreenEnum::Game;
            }
        }
        ScreenEnum::Game => {
            for _ in win_game_event.iter() {
                game_state.screen = ScreenEnum::Win;
                set_visibility(&mut text_set.p1(), true);
            }
            for _ in lose_game_event.iter() {
                game_state.screen = ScreenEnum::Lose;
                set_visibility(&mut text_set.p2(), true);
            }
        }
        _ => {}
    }
}
