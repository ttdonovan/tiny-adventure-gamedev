use macroquad::prelude::*;
use shipyard::World;

use crate::player::Player;

pub(crate) fn render(world: &World) {
    let player = world.get_unique::<&Player>().unwrap();

    draw_circle(
        player.shape.center().x,
        player.shape.center().y,
        player.shape.size / 2.0,
        YELLOW,
    );
}
