use bevy::prelude::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving,
    Dashing,
    Hooking,
    Sliding,
}