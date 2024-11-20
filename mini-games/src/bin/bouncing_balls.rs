use macroquad::prelude::*;

struct Ball {
    radius: f32,
    pos: Vec2,
}

#[macroquad::main("Bouncing Balls!")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let ball = Ball {
        radius: 6.0,
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
    };

    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        // draw the ball
        draw_circle(ball.pos.x, ball.pos.y, ball.radius, YELLOW);

        next_frame().await
    }
}
