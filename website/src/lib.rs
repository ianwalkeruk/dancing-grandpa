use bevy::prelude::*;
use common::DancingGrandpaPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dancing Grandpa - Baby Entertainment System".into(),
                resolution: (800.0, 600.0).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DancingGrandpaPlugin)
        .run();
}
