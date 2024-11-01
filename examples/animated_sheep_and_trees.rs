use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use shipyard::{Component, World};

mod animations {
    use macroquad::experimental::animation::{AnimatedSprite, Animation};

    pub fn sheep_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            128,
            128,
            &[
                Animation {
                    name: "idle".to_owned(),
                    row: 1,
                    frames: 8,
                    fps: 8,
                },
                Animation {
                    name: "bouncing".to_owned(),
                    row: 2,
                    frames: 6,
                    fps: 6,
                },
            ],
            true,
        )
    }

    pub fn tree_sprite() -> AnimatedSprite {
        AnimatedSprite::new(
            192,
            192,
            &[
                Animation {
                    name: "original".to_owned(),
                    row: 0,
                    frames: 1,
                    fps: 1,
                },
                Animation {
                    name: "idle".to_owned(),
                    row: 1,
                    frames: 4,
                    fps: 1,
                },
            ],
            true,
        )
    }
}

#[derive(Component)]
struct Sheep {
    pos: Vec2,
    sprite: AnimatedSprite,
}

#[macroquad::main("Sheep and Trees!")]
async fn main() -> Result<(), macroquad::Error> {
    rand::srand(miniquad::date::now() as u64);

    set_pc_assets_folder("assets");
    let sheep_texture = load_texture("tiny_swords/resources/sheep/happy_sheep.png").await?;
    sheep_texture.set_filter(FilterMode::Nearest);

    let tree_texture = load_texture("tiny_swords/resources/trees/tree.png").await?;
    tree_texture.set_filter(FilterMode::Nearest);

    build_textures_atlas();

    let mut tree_sprite = animations::tree_sprite();

    let mut world = World::new();

    for _ in 0..20 {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());

        let pos = vec2(x, y);
        dbg!(&pos);

        world.add_entity(Sheep {
            pos,
            sprite: animations::sheep_sprite(),
        });
    }

    loop {
        clear_background(DARKGREEN);

        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        if is_mouse_button_down(MouseButton::Left) {
            for sheep in &mut world.iter::<&mut Sheep>() {
                sheep.sprite.set_animation(1);
            }

            tree_sprite.set_animation(1);
        } else {
            for sheep in &mut world.iter::<&mut Sheep>() {
                sheep.sprite.set_animation(0);
            }

            tree_sprite.set_animation(0);
        }

        for sheep in &mut world.iter::<&Sheep>() {
            let sheep_frame = sheep.sprite.frame();

            draw_texture_ex(
                &sheep_texture,
                sheep.pos.x,
                sheep.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(sheep_frame.dest_size),
                    source: Some(sheep_frame.source_rect),
                    ..Default::default()
                },
            );
        }

        let tree_frame = tree_sprite.frame();
        draw_texture_ex(
            &tree_texture,
            150.0,
            150.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(tree_frame.dest_size),
                source: Some(tree_frame.source_rect),
                ..Default::default()
            },
        );

        for sheep in &mut world.iter::<&mut Sheep>() {
            sheep.sprite.update();
        }

        tree_sprite.update();
        next_frame().await
    }
}
