pub mod camera;
pub mod camera_shake;

pub use camera::MainCamera;   // so `crate::world::MainCamera` works (input.rs uses it)

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;   // RapierConfiguration, DefaultRapierContext

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, camera_shake::CameraShakePlugin))
            .add_systems(Startup, configure_physics);
    }
}

// Moved here from main.rs — physics config is a world-level concern.
fn configure_physics(
    mut q_rapier_config: Query<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    if let Ok(mut rapier_config) = q_rapier_config.single_mut() {
        rapier_config.gravity = Vec2::ZERO;
    }
}