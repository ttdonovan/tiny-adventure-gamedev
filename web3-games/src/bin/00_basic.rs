use macroquad::prelude::*;

#[macroquad::main("Basic")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        let dt = get_frame_time();

        draw_text(&format!("Delta Time: {:.4}", dt), 20.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
