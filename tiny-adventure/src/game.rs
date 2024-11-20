use macroquad::prelude::*;
use shipyard::{Unique, UniqueViewMut, World};

use crate::{input, player, renderer::render, shared::Shape};

#[derive(Debug, Unique)]
pub struct Boundry(pub f32, pub f32);

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

        self.init(screen_width(), screen_height());

        loop {
            clear_background(DARKPURPLE);

            let dt = get_frame_time();
            self.world.run(|mut delta_time: UniqueViewMut<DeltaTime>| {
                delta_time.0 = dt;
            });

            self.world.run(input::player_input);
            self.world.run(player::move_player);

            render(&self.world);

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
    pub fn init(&mut self, screen_width: f32, screen_height: f32) {
        self.add_unique_boundry(screen_width, screen_height);
        self.add_unique_delta_time();
        self.add_unique_player(screen_width, screen_height);
        self.add_unique_player_input();
    }

    fn add_unique_boundry(&mut self, screen_width: f32, screen_height: f32) {
        self.world.add_unique(Boundry(screen_width, screen_height));
    }

    fn add_unique_delta_time(&mut self) {
        self.world.add_unique(DeltaTime(0.0));
    }

    fn add_unique_player(&mut self, screen_width: f32, screen_height: f32) {
        let pos = Vec2::new(screen_width / 2.0, screen_height / 2.0);
        let shape = Shape::new(pos, 16.0);
        self.world.add_unique(player::Player { shape });
    }

    pub fn add_unique_player_input(&mut self) {
        self.world.add_unique(input::PlayerInput::default());
    }
}
