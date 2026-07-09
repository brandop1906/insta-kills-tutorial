use bevy::prelude::*;
use chrono::Utc;
use bevy_rapier2d::plugin::RapierTransformPropagateSet;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::world::camera::MainCamera; // re-exported from world::camera at the module root

const NOISE_STRENGTH: f32 = 10.0;
const TRANSLATION_SHAKE_STRENGTH: f32 = 15.0;
const ROTATION_SHAKE_STRENGTH: f32 = 2.5;

#[derive(Resource, Default, Reflect)]
pub struct CameraShake {
    trauma: f32,
    seed: f32,
    target: Vec2,
}

impl CameraShake {
    pub fn add_trauma(&mut self, trauma: f32) {
        if self.trauma == 0.0 {
            self.seed = (Utc::now().timestamp_millis() & 0xFFFF) as f32;
        }
        self.trauma = (self.trauma + trauma.abs()).min(1.0);   // clamp to 1
    }

    pub fn update_target(&mut self, target: Vec2) {
        self.target = target;
    }

    fn reduce_trauma(&mut self, delta: f32) {
        self.trauma = (self.trauma - delta.abs()).max(0.0);    // never below 0
    }

    // One noise sample per axis; `stack` shifts the seed so x/y/rotation decorrelate.

    fn noise_value(&self, stack: u32) -> f32 {
        simplex_noise_2d_seeded(
            Vec2::new(self.trauma * NOISE_STRENGTH, 0.0),  // Use bevy's Vec2 directly
            self.seed + stack as f32,
        )
    }
}

fn decay_shake_trauma(time: Res<Time>, mut shake: ResMut<CameraShake>) {
    shake.reduce_trauma(time.delta_secs());
}

pub fn update_camera(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    shake: ResMut<CameraShake>,
) {
    let mut transform = match q_camera.single_mut() { Ok(t) => t, Err(_) => return };

    let translation_offset = Vec3::new(shake.noise_value(0), shake.noise_value(1), 0.0)
        * shake.trauma.powi(2) * TRANSLATION_SHAKE_STRENGTH;      // trauma² feel
    let rotation_offset = Quat::from_rotation_z(
        (shake.noise_value(2) * shake.trauma.powi(2) * ROTATION_SHAKE_STRENGTH).to_radians(),
    );

    transform.translation = shake.target.extend(transform.translation.z) + translation_offset;
    transform.rotation = rotation_offset;
}

pub struct CameraShakePlugin;

impl Plugin for CameraShakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraShake>()
            .add_systems(Update, decay_shake_trauma)
            .add_systems(PostUpdate, update_camera.before(RapierTransformPropagateSet));
    }
}