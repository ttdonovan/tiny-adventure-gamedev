use tiny_adventure::{game, input, player, Game};

#[test]
fn test_new_game_with_seed() {
    let seed = 1000;
    let game = Game::new(seed);
    assert_eq!(game.seed, seed);
}

#[test]
fn test_default_game_and_init() {
    let mut game = Game::default();
    game.init(1280.0, 720.0);

    let boundry = game.world.get_unique::<&game::Boundry>();
    assert!(boundry.is_ok());

    let delta_time = game.world.get_unique::<&game::DeltaTime>();
    assert!(delta_time.is_ok());

    let player = game.world.get_unique::<&player::Player>();
    assert!(player.is_ok());

    let player_input = game.world.get_unique::<&input::PlayerInput>();
    assert!(player_input.is_ok());
}
