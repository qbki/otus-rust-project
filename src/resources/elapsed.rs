pub struct Elapsed {
    pub seconds: f32,
    pub is_paused: bool,
}

impl Elapsed {
    pub fn new() -> Self {
        Self {
            seconds: 0.0,
            is_paused: true,
        }
    }
}
