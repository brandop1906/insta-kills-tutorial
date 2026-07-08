pub mod input;
pub mod state;
pub mod spawn;
pub mod movement;

use bevy::prelude::*;
use state::PlayerState;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(100.0, 100.0, 0.0);
const MOVE_SPEED: f32 = 400.0;
const PLAYER_HITBOX_OFFSET: Vec3 = Vec3::new(0.0, -10.0, 0.0);

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
    pub hook_target_pos: Vec2,
    pub collider_entity: Entity,
    pub disabled: bool,
}

impl Player {
    // Private on purpose - only spawn.rs (a child module of 'player') constructs one.
    fn new(collider_entity: Entity) -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::ZERO,
            hook_target_pos: Vec2::ZERO,
            collider_entity,
            disabled: false,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            spawn::PlayerSpawnPlugin,
            movement::PlayerMovementPlugin,
        ));
    }
}