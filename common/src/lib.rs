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
            .add_systems(Update, simple_animation_system);
    }
}
