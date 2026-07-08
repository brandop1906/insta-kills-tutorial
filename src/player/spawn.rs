use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::animation::AnimationPlayer2D;
use crate::{GameAssets, GameState};

use super::{Player, PLAYER_HITBOX_OFFSET, PLAYER_SPAWN_POS};

#[derive(Component)]
pub struct PlayerCollider;

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat(); // idle

    // Child entity: a small ball collider that emits collision events.
    let collider = commands
        .spawn((
            PlayerCollider,
            Collider::ball(4.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            Transform::from_translation(PLAYER_HITBOX_OFFSET),
        ))
        .id();
    // Root entity: sprite + animator + dynamic physics body
    commands
        .spawn((
            Player::new(collider),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED, // never spin from an impact
            Velocity::zero(),
            Ccd::enabled(),
            animator,
            Sprite {
                image: assets.player.clone(),
                texture_atlas: Some(TextureAtlas::from(assets.player_layout.clone())),
                ..default()
            },
            Transform::from_translation(PLAYER_SPAWN_POS).with_scale(Vec3::splat(2.0)),
        ))
        .add_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}