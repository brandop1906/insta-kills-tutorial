use bevy::prelude::*;
use bevy_rapier2d::prelude::*;               // Velocity
use crate::GameState;
use super::input::PlayerInput;
use super::{Player, PlayerState, MOVE_SPEED};   // constants live in mod.rs

fn player_movement(
    mut q_player: Query<(&mut Velocity, &mut Player)>,
    player_input: Res<PlayerInput>,
) {
    let (mut velocity, mut player) = match q_player.single_mut() {
        Ok(r) => r,
        Err(_) => return,          // no player yet — nothing to do
    };

    // Abilities (dash, hook) own their own states; movement only drives Idling/Moving.
    if player.state != PlayerState::Moving && player.state != PlayerState::Idling {
        return;
    }

    let direction = player_input.move_direction;
    if direction == Vec2::ZERO {
        player.state = PlayerState::Idling;
        velocity.linvel = Vec2::ZERO;
        return;
    }

    player.state = PlayerState::Moving;
    player.current_direction = direction;
    velocity.linvel = direction * MOVE_SPEED;   // physics does the rest
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement.run_if(in_state(GameState::Gaming)));
    }
}