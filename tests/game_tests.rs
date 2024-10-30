use tiny_adventure::{game, Game, player};

#[test]
fn test_new_game_with_seed() {
    let seed = 1000;
    let game = Game::new(seed);
    assert_eq!(game.seed, seed);
}

#[test]
fn test_default_game_and_init() {
    let mut game = Game::default();
    game.init();

    let delta_time = game.world.get_unique::<&game::DeltaTime>();
    assert!(delta_time.is_ok());

    let player = game.world.get_unique::<&player::Player>();
    assert!(player.is_ok());
}
