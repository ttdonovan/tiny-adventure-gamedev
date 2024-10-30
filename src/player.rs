use macroquad::prelude::*;
use shipyard::{Unique, UniqueView, UniqueViewMut};

use crate::{game::DeltaTime, input, shared::Shape};

#[derive(Debug, Unique)]
pub struct Player {
    pub shape: Shape,
}

impl Player {
    const MOVEMENT_SPEED: f32 = 100.0;

    pub fn move_dir(&mut self, dir: Vec2, dt: f32) {
        let pos = &mut self.shape.0;
        pos.0 += dir * Self::MOVEMENT_SPEED * dt;
    }
}

// TODO: add test...
pub fn move_player(
    input: UniqueView<input::PlayerInput>,
    mut player: UniqueViewMut<Player>,
    dt: UniqueView<DeltaTime>,
) {
    let mut dir = Vec2::ZERO;

    if input.right {
        dir.x = 1.0;
    }

    if input.left {
        dir.x = -1.0;
    }

    if input.down {
        dir.y = 1.0;
    }

    if input.up {
        dir.y = -1.0;
    }

    player.move_dir(dir, dt.0);
}
