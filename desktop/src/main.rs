use bevy::prelude::*;
use dancing_grandpa_common::DancingGrandpaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dancing Grandpa - Baby Entertainment System".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DancingGrandpaPlugin)
        .run();
}
