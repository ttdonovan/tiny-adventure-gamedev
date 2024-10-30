use macroquad::prelude::*;
use shipyard::{Unique, UniqueViewMut};

#[derive(Debug, Default, Unique)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

pub fn player_input(mut input: UniqueViewMut<PlayerInput>) {
    input.up = is_key_down(KeyCode::W);
    input.down = is_key_down(KeyCode::S);
    input.left = is_key_down(KeyCode::A);
    input.right = is_key_down(KeyCode::D);
}
