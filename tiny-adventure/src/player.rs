use macroquad::prelude::*;
use shipyard::{Unique, UniqueView, UniqueViewMut};

use crate::{game, input, shared::Shape};

#[derive(Debug, Unique)]
pub struct Player {
    pub shape: Shape,
}

impl Player {
    const MOVEMENT_SPEED: f32 = 100.0;

    pub fn move_dir(&mut self, dir: Vec2, dt: f32, max_x: f32, max_y: f32) {
        self.shape.pos += dir * Self::MOVEMENT_SPEED * dt;

        let size = self.shape.size / 2.0;
        self.shape.pos.x = (self.shape.pos.x + dir.x).clamp(0.0 - size, max_x - size);
        self.shape.pos.y = (self.shape.pos.y + dir.y).clamp(0.0 - size, max_y - size);
    }
}

// TODO: add test...
pub fn move_player(
    input: UniqueView<input::PlayerInput>,
    mut player: UniqueViewMut<Player>,
    dt: UniqueView<game::DeltaTime>,
    boundry: UniqueView<game::Boundry>,
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

    player.move_dir(dir, dt.0, boundry.0, boundry.1);
}
