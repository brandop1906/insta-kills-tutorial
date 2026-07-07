use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};

const BACKGROUND_COLOR: Color = Color::srgb(0.75, 0.6, 0.5);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,   // vsync
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: true,        // matters for the web build
                        ..default()
                    }),
                    ..default()
                })
                // Pixel art: sample textures with nearest-neighbor, not blurry linear.
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}