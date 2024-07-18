//! Pawsable real-time atmospheric stealth game. You play as a cat that must avoid the light while
//! exploring a house inhabited by strange light beings, using the items you find to create helpful
//! tools.
#![warn(missing_docs)]

use bevy::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.run();
}
