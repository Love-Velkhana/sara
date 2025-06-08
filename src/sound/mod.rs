use bevy::prelude::*;

pub struct SoundManager;
impl SoundManager {
    fn init() {}
}
impl Plugin for SoundManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::init);
    }
}
