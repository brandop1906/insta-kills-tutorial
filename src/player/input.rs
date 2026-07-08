use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

use crate::player;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub attack: bool,
    pub dash: bool,
    pub hook: bool,
    pub scroll: f32,
    pub escape: bool,
    pub restart: bool,
}

fn reset_player_input(mut player_input: ResMut<PlayerInput>) {
    *player_input = PlayerInput::default();
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    let mut direction = Vec2::default();
    if keys.pressed(KeyCode::KeyW) { direction.y += 1.0 }
    if keys.pressed(KeyCode::KeyS) { direction.y -= 1.0 }
    if keys.pressed(KeyCode::KeyD) { direction.x += 1.0 }
    if keys.pressed(KeyCode::KeyA) { direction.x -= 1.0 }
    input.move_direction = direction.normalize_or_zero();
}

fn attack(mouse: Res<ButtonInput<MouseButton>>, mut input: ResMut<PlayerInput>) {
    input.attack = mouse.just_pressed(MouseButton::Left);
}

fn dash(key: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    input.dash = key.just_pressed(KeyCode::Space);
}

fn hook(key: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    input.hook = key.just_pressed(KeyCode::ShiftLeft);
}

fn input_escape(key: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    input.escape = key.just_pressed(KeyCode::Escape);
}

fn input_restart(key: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    input.restart = key.just_pressed(KeyCode::KeyR);
}

fn fetch_scroll_events(mut scroll_evr: MessageReader<MouseWheel>, mut input: ResMut<PlayerInput>) {
    for ev in scroll_evr.read() {
        input.scroll = if ev.y > 0.0 { -1.0 } else { 1.0 };
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(PreUpdate, reset_player_input.before(InputSystems))
            .add_systems(
                PreUpdate,
                (player_movement, attack, dash, hook, input_escape, input_restart, fetch_scroll_events)
                .after(InputSystems)
            );
    }
}