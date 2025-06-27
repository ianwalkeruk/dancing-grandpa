use bevy::prelude::*;
use dancing_grandpa_common::DancingGrandpaPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    
    // Configure plugins with WebGL compatibility
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Dancing Grandpa - Baby Entertainment System".into(),
            resolution: (800.0, 600.0).into(),
            canvas: Some("#bevy".to_owned()),
            ..default()
        }),
        ..default()
    });
    
    // Add plugins and run
    app.add_plugins(default_plugins)
       .add_plugins(DancingGrandpaPlugin)
       .run();
}
