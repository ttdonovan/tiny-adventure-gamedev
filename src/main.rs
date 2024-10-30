use macroquad::prelude::*;

#[macroquad::main("Tiny Adventure")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);

        next_frame().await;
    }
}
