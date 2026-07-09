use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};
use bevy_asset_loader::prelude::*;

pub use assets::GameAssets;
use bevy_rapier2d::plugin::{RapierPhysicsPlugin, NoUserData};

use crate::animation::{Animation2DPlugin,};
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

mod assets;
mod animation;
mod player;
mod world;

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
        .add_plugins(Animation2DPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(WorldPlugin)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::Gaming)
            .load_collection::<GameAssets>(),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    GameOver,
    Restart,
}