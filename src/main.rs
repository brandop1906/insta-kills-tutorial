use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};
use bevy_asset_loader::prelude::*;

pub use assets::GameAssets;
use bevy_rapier2d::plugin::{RapierPhysicsPlugin, RapierConfiguration, DefaultRapierContext, NoUserData};

use crate::animation::{Animation2DPlugin, AnimationPlayer2D};
use crate::player::PlayerPlugin;
use crate::player::input::InputPlugin;
use crate::player::spawn::PlayerSpawnPlugin;

mod assets;
mod animation;
mod player;

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
        .add_systems(Startup, configure_physics)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::Gaming)
            .load_collection::<GameAssets>(),
        )
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Gaming), setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn configure_physics(
    mut q_rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if let Ok(mut rapier_config) = q_rapier_config.single_mut() {
        rapier_config.gravity = Vec2::ZERO;
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    GameOver,
    Restart,
}