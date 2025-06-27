use bevy::prelude::*;
use std::time::Duration;

pub mod config;
pub mod systems;
pub mod components;
pub mod resources;

pub use config::*;
pub use systems::*;
pub use components::*;
pub use resources::*;

pub struct DancingGrandpaPlugin;

impl Plugin for DancingGrandpaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_system)
            .add_systems(Update, (
                animation_system,
                audio_system,
                fade_system,
                restart_system,
            ))
            .insert_resource(GameState::Loading)
            .insert_resource(AnimationTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating)))
            .insert_resource(FadeTimer(Timer::new(Duration::from_secs(3), TimerMode::Once)))
            .insert_resource(RestartTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)));
    }
}
