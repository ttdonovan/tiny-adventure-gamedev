use tiny_adventure::{game::DeltaTime, Game};

#[test]
fn test_game_new_with_seed() {
    let seed = 1000;
    let game = Game::new(seed);
    assert_eq!(game.seed, seed);
}

#[test]
fn test_game_world_and_delta_time() {
    let mut game = Game::default();
    game.add_unique_delta_time();

    let delta_time = game.world.get_unique::<&DeltaTime>().unwrap();
    assert_eq!(0.0, delta_time.0);
}
