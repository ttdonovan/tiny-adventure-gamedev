use tiny_adventure::Game;
#[macroquad::main("Tiny Adventure")]
async fn main() {
    let mut game = Game::default();
    game.run().await;
}
