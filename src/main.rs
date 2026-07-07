use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};
use bevy_asset_loader::prelude::*;

pub use assets::GameAssets;

use crate::animation::{Animation2DPlugin, AnimationPlayer2D};
mod assets;
mod animation;

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
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
            .continue_to_state(GameState::Gaming)
            .load_collection::<GameAssets>(),
        )
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Gaming), (setup, setup_player))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat(); // 0 = idle

    commands.spawn((
        Sprite {
            image: assets.player.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.player_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        animator,
    ));
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    GameOver,
    Restart,
}