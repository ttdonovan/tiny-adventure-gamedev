use macroquad::{color, prelude::*};
use shipyard::{
    AllStoragesViewMut, Component, Delete, EntitiesViewMut, IntoIter, IntoWithId, SparseSet, View,
    ViewMut, World,
};

#[derive(Component)]
struct Bubble {
    pos: Vec2,
    size: f32,
}

impl Bubble {
    fn random() -> Self {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());

        let s = rand::gen_range(0.0, 1.0);
        let size = 4.0 + s * 16.0;

        Bubble {
            pos: vec2(x, y),
            size,
        }
    }
}

#[derive(Component)]
struct Explosion {
    pos: Vec2,
    radius: f32,
}

#[derive(Component)]
struct ToDelete;

fn setup(world: &mut World) {
    world.bulk_add_entity((0..1000).map(|_| Bubble::random()));
}

fn draw_shapes(v_bubbles: View<Bubble>) {
    for bubble in v_bubbles.iter() {
        let h = (bubble.size - 4.0) / 16.0;
        let color = color::hsl_to_rgb(h, 1.0, 0.8);
        draw_circle_lines(bubble.pos.x, bubble.pos.y, bubble.size, 1.0, color);
    }
}

#[macroquad::main("Bubble Pop!")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut world = World::new();
    setup(&mut world);

    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = mouse_position();

            world.add_entity(Explosion {
                pos: vec2(pos.0, pos.1),
                radius: 1.0,
            });
        }

        world.run(draw_shapes);

        world.run(
            |mut entities: EntitiesViewMut,
             v_bubbles: View<Bubble>,
             mut vm_explosions: ViewMut<Explosion>,
             mut vm_to_delete: ViewMut<ToDelete>| {
                let ids_to_delete_and_cascade_explosions: Vec<_> = vm_explosions
                    .iter()
                    .with_id()
                    .map(|(explosion_id, explode)| {
                        let mut next_explosion: Option<Explosion> = None;

                        for (bubble_id, bubble) in v_bubbles.iter().with_id() {
                            if bubble.pos.distance(explode.pos) < bubble.size + explode.radius {
                                next_explosion = Some(Explosion {
                                    pos: bubble.pos,
                                    radius: bubble.size,
                                });
                                entities.add_component(bubble_id, &mut vm_to_delete, ToDelete);
                            }
                        }

                        (explosion_id, next_explosion)
                    })
                    .collect();

                for (id, next_explosion) in ids_to_delete_and_cascade_explosions {
                    (&mut vm_explosions).delete(id);

                    if let Some(explosion) = next_explosion {
                        entities.add_entity(&mut vm_explosions, explosion);
                    }
                }
            },
        );

        world.run(|mut all_storages: AllStoragesViewMut| {
            all_storages.delete_any::<SparseSet<ToDelete>>();
        });

        next_frame().await
    }
}
