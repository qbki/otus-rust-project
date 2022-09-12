use bevy::prelude::*;
use crate::resources::Elapsed;

pub fn elapsed_system(
    mut elapsed: ResMut<Elapsed>,
    time: Res<Time>,
) {
    elapsed.seconds = if elapsed.is_paused {
        0.0
    } else {
        time.delta_seconds()
    };
}
