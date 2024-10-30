use tiny_adventure::{game::DeltaTime, Game};

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

    let delta_time = game.world.get_unique::<&DeltaTime>().unwrap();
    assert_eq!(0.0, delta_time.0);
}
