use macroquad::prelude::*;
use shipyard::{Unique, UniqueViewMut, World};

#[derive(Debug, Unique)]
pub struct DeltaTime(pub f32);

#[derive(Debug)]
pub struct Game {
    pub seed: u64,
    pub world: World,
}

impl Game {
    pub fn new(seed: u64) -> Self {
        let world = World::new();
        Game { seed, world }
    }

    pub async fn run(&mut self) {
        rand::srand(self.seed);

        self.add_unique_delta_time();

        loop {
            clear_background(DARKPURPLE);

            let dt = get_frame_time();
            self.world.run(|mut delta_time: UniqueViewMut<DeltaTime>| {
                delta_time.0 = dt;
            });

            next_frame().await;
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        let seed = miniquad::date::now() as u64;
        let game = Game::new(seed);
        game
    }
}

impl Game {
    pub fn add_unique_delta_time(&mut self) {
        self.world.add_unique(DeltaTime(0.0));
    }
}
