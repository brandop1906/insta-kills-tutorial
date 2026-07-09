use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player::Player;
use crate::GameState;
use super::camera_shake::{update_camera, CameraShake};

const RAPIER_TIMESTEP: f32 = 60.0;
pub const TRANSLATION_TO_PIXEL: f32 = 0.0001;

fn update_camera_target(
    mut shake: ResMut<CameraShake>,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
) {
    let (player_pos, player_vel) = match q_player.single() {
        Ok(p) => (p.0.translation, p.1),
        Err(_) => return,
    };

    // Aim one physics-step ahead of where the player is right now.
    shake.update_target(Vec2::new(
        player_pos.x + player_vel.linvel.x / RAPIER_TIMESTEP,
        player_pos.y + player_vel.linvel.y / RAPIER_TIMESTEP,
    ));
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));
}

#[derive(Component)]
pub struct YSort(pub f32);   // the base layer; children pass e.g. -1.0 or 1.0

pub fn apply_y_sort(mut q: Query<(&mut Transform, &GlobalTransform, &YSort)>) {
    for (mut transform, global, ysort) in &mut q {
        transform.translation.z = ysort.0 - global.translation().y * TRANSLATION_TO_PIXEL;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn_camera)
            .add_systems(Update, apply_y_sort)
            .add_systems(PostUpdate, update_camera_target.before(update_camera));
    }
}