#[derive(PartialEq)]
pub enum ScreenEnum {
    Start,
    Game,
    Lose,
    Win,
}

pub struct GameState {
    pub screen: ScreenEnum,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            screen: ScreenEnum::Start,
        }
    }
}
