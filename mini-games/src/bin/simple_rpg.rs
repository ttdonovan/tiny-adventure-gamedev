use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use macroquad_tiled as tiled;
use shipyard::{
    Component, EntitiesViewMut, IntoIter, Unique, UniqueView, UniqueViewMut, View, ViewMut, World,
};

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;

mod animations {
    use macroquad::experimental::animation::{AnimatedSprite, Animation};

    pub fn player_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            48,
            48,
            &[
                Animation {
                    name: "idle_front".to_owned(),
                    row: 0,
                    frames: 6,
                    fps: 6,
                },
                Animation {
                    name: "idle_side".to_owned(),
                    row: 1,
                    frames: 6,
                    fps: 6,
                },
                Animation {
                    name: "idle_back".to_owned(),
                    row: 2,
                    frames: 6,
                    fps: 6,
                },
                Animation {
                    name: "move_front".to_owned(),
                    row: 3,
                    frames: 6,
                    fps: 6,
                },
                Animation {
                    name: "move_side".to_owned(),
                    row: 4,
                    frames: 6,
                    fps: 6,
                },
                Animation {
                    name: "move_back".to_owned(),
                    row: 5,
                    frames: 6,
                    fps: 6,
                },
                Animation {
                    name: "attack_front".to_owned(),
                    row: 6,
                    frames: 4,
                    fps: 8,
                },
                Animation {
                    name: "attack_side".to_owned(),
                    row: 7,
                    frames: 4,
                    fps: 8,
                },
                Animation {
                    name: "attack_back".to_owned(),
                    row: 8,
                    frames: 4,
                    fps: 8,
                },
                Animation {
                    name: "death".to_owned(),
                    row: 9,
                    frames: 3,
                    fps: 3,
                },
            ],
            true,
        )
    }

    pub fn slime_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            32,
            32,
            &[
                Animation {
                    name: "idle_side".to_owned(),
                    row: 1,
                    frames: 4,
                    fps: 4,
                },
                Animation {
                    name: "move_side".to_owned(),
                    row: 4,
                    frames: 6,
                    fps: 6,
                },
                Animation {
                    name: "attack_front".to_owned(),
                    row: 6,
                    frames: 7,
                    fps: 7,
                },
            ],
            true,
        )
    }
}

mod timers {
    pub struct Timer {
        elapsed: f64,
        wait: f64,
    }

    impl Timer {
        pub fn new(wait: f64) -> Self {
            Self { elapsed: 0.0, wait }
        }

        pub fn update(&mut self, elapsed_time: f64) -> bool {
            self.elapsed += elapsed_time;
            self.is_done()
        }

        pub fn is_done(&self) -> bool {
            self.wait <= self.elapsed
        }

        pub fn reset(&mut self) {
            while self.is_done() {
                self.elapsed = 0.0;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

trait Actor {
    fn pos(&self) -> Vec2;
    fn body(&self) -> &Body;
    fn hitbox_r(&self) -> f32;

    fn shape(&self) -> Circle {
        let pos = self.pos();

        Circle {
            x: pos.x,
            y: pos.y,
            r: self.body().0,
        }
    }

    fn hitbox(&self) -> Circle {
        let pos = self.pos();

        Circle {
            x: pos.x,
            y: pos.y,
            r: self.hitbox_r(),
        }
    }
}

struct Body(f32); // radius
struct Health(u32); // hit points
struct Hitbox(f32); // radius
struct Sensor(f32); // radius

#[derive(Unique)]
struct Player {
    dir: Dir,
    pos: Vec2,
    body: Body,
    hitbox: Hitbox,
    attack_cooldown: timers::Timer,
    sprite: AnimatedSprite,
    sprite_flip: bool,
    health: Health,
    is_alive: bool,
    is_attacking: bool,
}

impl Actor for Player {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn body(&self) -> &Body {
        &self.body
    }

    fn hitbox_r(&self) -> f32 {
        self.hitbox.0
    }
}

#[derive(Component)]
struct PlayerAttack((Dir, Vec2));

#[derive(Component)]
struct Enemy {
    pos: Vec2,
    body: Body,
    hitbox: Hitbox,
    sensor: Sensor,
    attack_cooldown: timers::Timer,
    sprite: AnimatedSprite,
    sprite_flip: bool,
}

impl Enemy {
    fn sensor(&self) -> Circle {
        Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: self.sensor.0,
        }
    }
}

impl Actor for Enemy {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn body(&self) -> &Body {
        &self.body
    }

    fn hitbox_r(&self) -> f32 {
        self.hitbox.0
    }
}

fn spawn_enemies(world: &mut World, player_pos: Vec2) {
    for _ in 0..5 {
        let angle = rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
        let distance: f32 = rand::gen_range(100.0, 200.0);
        let offset_x = distance * angle.cos();
        let offset_y = distance * angle.sin();

        let enemy = Enemy {
            pos: vec2(player_pos.x + offset_x, player_pos.y + offset_y),
            body: Body(8.0),
            hitbox: Hitbox(12.0),
            sensor: Sensor(128.0),
            attack_cooldown: timers::Timer::new(0.5),
            sprite: animations::slime_sprite(),
            sprite_flip: false,
        };

        world.add_entity(enemy);
    }
}

#[derive(Clone, Component)]
struct HitPlayer(Vec2);

#[macroquad::main("Simple RPG")]
async fn main() -> Result<(), macroquad::Error> {
    // seed the random number generator
    rand::srand(miniquad::date::now() as u64);

    // load all textures
    set_pc_assets_folder("assets");
    let player_texture = load_texture("mystic_woods/characters/player.png").await?;
    player_texture.set_filter(FilterMode::Nearest);

    let slime_texture = load_texture("mystic_woods/characters/slime.png").await?;
    slime_texture.set_filter(FilterMode::Nearest);

    let grass_tileset = load_texture("mystic_woods/tilesets/grass.png").await?;
    grass_tileset.set_filter(FilterMode::Nearest);

    let plains_tileset = load_texture("mystic_woods/tilesets/plains.png").await?;
    plains_tileset.set_filter(FilterMode::Nearest);

    let objects_tileset = load_texture("mystic_woods/objects/objects.png").await?;
    objects_tileset.set_filter(FilterMode::Nearest);

    build_textures_atlas();

    let tiled_map_json = load_string("tiled/rpg/Level_1.json").await?;
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[
            ("grass.png", grass_tileset),
            ("plains.png", plains_tileset),
            ("objects.png", objects_tileset),
        ],
        &[],
    )
    .unwrap();

    // build the world
    let mut world = World::new();

    let player_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let player = Player {
        dir: Dir::Down,
        pos: player_pos,
        body: Body(8.0),
        hitbox: Hitbox(12.0),
        attack_cooldown: timers::Timer::new(0.5),
        sprite: animations::player_sprite(),
        sprite_flip: false,
        health: Health(100),
        is_alive: true,
        is_attacking: false,
    };

    world.add_unique(player);

    // spawn a few enemies
    spawn_enemies(&mut world, player_pos);

    // run the game loop
    loop {
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        // get the delta time (used by timers)
        let delta_time = get_frame_time() as f64;

        // rendering target
        let render_target: RenderTarget =
            render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);

        // rendering target's camera
        let aspect = VIRTUAL_WIDTH / VIRTUAL_HEIGHT;
        let fovy = 300.0; // adjust this value to zoom in/out
        let mut render_target_camera = Camera3D {
            up: vec3(0.0, -1.0, 0.0),
            fovy,
            aspect: Some(aspect),
            projection: Projection::Orthographics,
            render_target: Some(render_target.clone()),
            ..Default::default()
        };

        // update the camera position to follow the player
        world.run(|player: UniqueView<Player>| {
            render_target_camera.position = vec3(player.pos.x, player.pos.y, -100.0);
            render_target_camera.target = vec3(player.pos.x, player.pos.y, 0.0);
        });

        // set the camera to the render target's camera and then draw...
        set_camera(&render_target_camera);
        // set_default_camera();
        ["Ground", "Ysort"].iter().for_each(|layer| {
            tiled_map.draw_tiles(layer, Rect::new(0.0, 0.0, 1920.0, 1280.0), None);
        });

        // update player direction and position
        world.run(|mut player: UniqueViewMut<Player>| {
            // reset the player sprite animation back to idle, if not attacking
            if !player.is_attacking {
                match player.dir {
                    Dir::Up => player.sprite.set_animation(2),
                    Dir::Down => player.sprite.set_animation(0),
                    Dir::Left => player.sprite.set_animation(1),
                    Dir::Right => player.sprite.set_animation(1),
                }
            }

            let player_speed = if player.is_alive { 1.0 } else { 0.3 };

            if is_key_down(KeyCode::Up) {
                player.dir = Dir::Up;
                player.sprite.set_animation(5);
                player.sprite_flip = false;
                player.pos.y -= player_speed;
            }

            if is_key_down(KeyCode::Down) {
                player.dir = Dir::Down;
                player.sprite.set_animation(3);
                player.sprite_flip = false;
                player.pos.y += player_speed;
            }

            if is_key_down(KeyCode::Left) {
                player.dir = Dir::Left;
                player.sprite.set_animation(4);
                player.sprite_flip = true;
                player.pos.x -= player_speed;
            }

            if is_key_down(KeyCode::Right) {
                player.dir = Dir::Right;
                player.sprite.set_animation(4);
                player.sprite_flip = false;
                player.pos.x += player_speed;
            }
        });

        // update player attack direction and animation
        world.run(
            |mut player: UniqueViewMut<Player>,
             mut entities: EntitiesViewMut,
             mut vm_player_attack: ViewMut<PlayerAttack>| {
                // second wind...
                if !player.is_alive {
                    if is_key_down(KeyCode::R) {
                        player.health.0 = 100;
                        player.is_alive = true;
                    }
                }

                // player attack...
                player.attack_cooldown.update(delta_time);

                if player.attack_cooldown.is_done() && is_key_down(KeyCode::A) {
                    entities.add_entity(
                        &mut vm_player_attack,
                        PlayerAttack((player.dir, player.pos)),
                    );

                    match player.dir {
                        Dir::Up => player.sprite.set_animation(8),
                        Dir::Down => player.sprite.set_animation(6),
                        Dir::Left => player.sprite.set_animation(7),
                        Dir::Right => player.sprite.set_animation(7),
                    }

                    player.attack_cooldown.reset();
                }

                match player.sprite.current_animation() {
                    6 | 7 | 8 => player.is_attacking = !player.sprite.is_last_frame(),
                    _ => player.is_attacking = false,
                }
            },
        );

        // TODO: look into better way to handle player attacks
        // apply player attacks
        world.run(
            |player: UniqueView<Player>,
             mut vm_player_attacks: ViewMut<PlayerAttack>,
             mut vm_enemies: ViewMut<Enemy>| {
                for attack in vm_player_attacks.iter() {
                    let (dir, pos) = attack.0;
                    let angle = match dir {
                        Dir::Up => -std::f32::consts::PI / 2.0,
                        Dir::Down => std::f32::consts::PI / 2.0,
                        Dir::Left => -std::f32::consts::PI,
                        Dir::Right => 0.0,
                    };
                    let distance = 10.0; // reach of player attack
                    let offset_x = distance * angle.cos();
                    let offset_y = distance * angle.sin();

                    let circle_r = 10.0; // size of player attack
                    let center_pos = vec2(pos.x + offset_x, pos.y + offset_y);
                    let circle = Circle::new(center_pos.x, center_pos.y, circle_r);
                    draw_circle_lines(circle.x, circle.y, circle_r, 1.0, WHITE);

                    // check for enemy hits
                    for enemy in (&mut vm_enemies).iter() {
                        let enemy_hitbox = enemy.hitbox();

                        if circle.overlaps(&enemy_hitbox) {
                            // enemy hit by player attack add knockback
                            let knockback_dir = (enemy.pos - center_pos).normalize();
                            let knockback_str = 15.0;

                            enemy.pos = enemy.pos + knockback_dir * knockback_str;
                        }
                    }
                }

                // only clear the attacks if the player is no longer attacking
                if !player.is_attacking {
                    vm_player_attacks.clear();
                }
            },
        );

        // update enemies
        world.run(
            |player: UniqueView<Player>,
             mut vm_enemies: ViewMut<Enemy>,
             mut entities: EntitiesViewMut,
             mut vm_hit_player: ViewMut<HitPlayer>| {
                let mut atks = vec![None; vm_enemies.len()];
                let mut dirs = vec![Vec2::ZERO; vm_enemies.len()];
                let player_shape = player.shape();
                let player_hitbox = player.hitbox();

                // update "all" enemy timers
                for enemy in (&mut vm_enemies).iter() {
                    enemy.attack_cooldown.update(delta_time);

                    // if enemy.attack_cooldown.is_done() {
                    //     enemy.sprite.set_animation(0);
                    // }
                }

                // build a list of attackes and directions for each enemy
                for ((enemy, atk), dir) in vm_enemies.iter().zip(&mut atks).zip(&mut dirs) {
                    // check for enemy aggro (sensor)
                    let enemy_sensor = enemy.sensor();

                    if !enemy_sensor.overlaps(&player_shape) {
                        // no aggro, maybe move randomly?
                        continue;
                    }

                    // check if the enemy can attack the player
                    if enemy.attack_cooldown.is_done() {
                        if enemy.hitbox().overlaps(&player_hitbox) {
                            *atk = Some(HitPlayer(enemy.pos));
                        };
                    }

                    let player_dir = player.pos - enemy.pos;
                    *dir = player_dir.normalize();

                    let mut neighbor_dir = Vec2::ZERO;

                    // check for nearby "neighbors" to avoid
                    for neighbor in vm_enemies.iter() {
                        if enemy.pos.distance_squared(neighbor.pos)
                            < enemy.body.0 * neighbor.body.0 / 1.5
                        {
                            neighbor_dir += enemy.pos - neighbor.pos;
                        }
                    }

                    *dir *= 0.2; // the enemy's speed (slower than player)
                    *dir += neighbor_dir * 1.5; // neighbor avoidance
                }

                // attack the player
                for (enemy, hit) in (&mut vm_enemies).iter().zip(atks) {
                    // FIXME: might be a bug here with attack animations...
                    if let Some(hit) = hit {
                        entities.add_entity(&mut vm_hit_player, hit);
                        enemy.sprite.set_animation(2);
                        // reset the attack cooldown
                        enemy.attack_cooldown.reset();
                    } else {
                        enemy.sprite.set_animation(1);
                        enemy.sprite_flip = player.pos.x - enemy.pos.x < 0.0;
                    }
                }

                // move the enemies
                for (enemy, dir) in (&mut vm_enemies).iter().zip(dirs) {
                    if dir == Vec2::ZERO {
                        enemy.sprite.set_animation(0);
                        continue;
                    }

                    enemy.pos += dir;
                }
            },
        );

        // check for enemy player hits and update player health
        world.run(
            |mut player: UniqueViewMut<Player>, mut vm_hit_players: ViewMut<HitPlayer>| {
                for hit in vm_hit_players.iter() {
                    if player.health.0 != 0 {
                        player.health.0 -= 10;
                        println!("Player is hit! HP: {}", player.health.0);

                        // knockback player
                        let knockback_dir = (player.pos - hit.0).normalize();
                        let knockback_str = 10.0;

                        player.pos = player.pos + knockback_dir * knockback_str;
                    }
                }

                if player.health.0 == 0 {
                    player.is_alive = false;
                    player.sprite.set_animation(9);
                }

                vm_hit_players.clear();
            },
        );

        // update animations for player and enemies
        world.run(
            |mut player: UniqueViewMut<Player>, mut vm_enemies: ViewMut<Enemy>| {
                player.sprite.update();

                for enemy in (&mut vm_enemies).iter() {
                    enemy.sprite.update();
                }
            },
        );

        // draw enemies
        world.run(|v_enemies: View<Enemy>| {
            for enemy in v_enemies.iter() {
                let enemy_frame = enemy.sprite.frame();

                draw_texture_ex(
                    &slime_texture,
                    enemy.pos.x - 16.0,
                    enemy.pos.y - 16.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(enemy_frame.dest_size),
                        source: Some(enemy_frame.source_rect),
                        flip_x: enemy.sprite_flip,
                        ..Default::default()
                    },
                );

                let shape = enemy.shape();
                draw_circle_lines(shape.x, shape.y, shape.r, 1.0, RED);

                let hitbox = enemy.hitbox();
                draw_circle_lines(hitbox.x, hitbox.y, hitbox.r, 1.0, BLUE);

                let sensor = enemy.sensor();
                draw_circle_lines(sensor.x, sensor.y, sensor.r, 1.0, ORANGE);
            }
        });

        // draw player
        world.run(|player: UniqueView<Player>| {
            let player_frame = player.sprite.frame();

            draw_texture_ex(
                &player_texture,
                player.pos.x - 24.0,
                player.pos.y - 32.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(player_frame.dest_size),
                    source: Some(player_frame.source_rect),
                    flip_x: player.sprite_flip,
                    ..Default::default()
                },
            );

            let hitbox = player.hitbox();
            draw_circle_lines(hitbox.x, hitbox.y, hitbox.r, 1.0, BLUE);

            let shape = player.shape();
            draw_circle_lines(shape.x, shape.y, shape.r, 1.0, YELLOW);
        });

        // restore regular drawing
        set_default_camera();

        // draw the render target properly scaled
        let scale = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );
        // dbg!(scale);

        // let scale = 1.0;

        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5,
            (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true, // must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
