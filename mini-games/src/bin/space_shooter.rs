use macroquad::prelude::*;
use shipyard::{
    AddComponent, AllStoragesViewMut, Component, EntitiesViewMut, IntoIter, IntoWithId, SparseSet,
    Unique, UniqueView, UniqueViewMut, View, ViewMut, World,
};

#[derive(Debug)]
struct Shape {
    x: f32,
    y: f32,
    size: f32,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[derive(Unique)]
struct Player {
    pos: Vec2,
    shape: Shape,
}

#[derive(Debug, Component)]
struct Enemy {
    pos: Vec2,
    vel: Vec2,
    shape: Shape,
}

#[derive(Debug, Component)]
struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shape: Shape,
}

#[derive(Debug, Component)]
struct ToDelete;

#[macroquad::main("Space Shooter")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let shape = Shape {
        x: pos.x,
        y: pos.y,
        size: 32.0,
    };
    let player = Player { pos, shape };

    let mut world = World::new();
    world.add_unique(player);

    loop {
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        let delta_time = get_frame_time();

        // update player's velocity and position (and shape)
        world.run(|mut player: UniqueViewMut<Player>| {
            let mut dir = Vec2::ZERO;

            if is_key_down(KeyCode::Right) {
                dir.x = 1.0;
            }

            if is_key_down(KeyCode::Left) {
                dir.x = -1.0;
            }

            if is_key_down(KeyCode::Down) {
                dir.y = 1.0;
            }

            if is_key_down(KeyCode::Up) {
                dir.y = -1.0;
            }

            if dir != Vec2::ZERO {
                let mov = (dir * 200.0 * delta_time).normalize();
                player.pos += mov;

                // clamp pos x and y to be within the screen
                player.pos.x = clamp(player.pos.x, 0.0, screen_width());
                player.pos.y = clamp(player.pos.y, 0.0, screen_height());

                player.shape.x = player.pos.x;
                player.shape.y = player.pos.y;
            }
        });

        // player shoots a bullet
        world.run(
            |mut entities: EntitiesViewMut,
             player: UniqueView<Player>,
             mut vm_bullets: ViewMut<Bullet>| {
                if is_key_pressed(KeyCode::Space) {
                    let bullet = Bullet {
                        pos: player.pos,
                        vel: vec2(0.0, -300.0),
                        shape: Shape {
                            x: player.pos.x,
                            y: player.pos.y,
                            size: 8.0,
                        },
                    };

                    entities.add_entity(&mut vm_bullets, bullet);
                }
            },
        );

        // generate new enemies
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            let x = rand::gen_range(size / 2.0, screen_width() - size / 2.0);
            let y = 0.0;
            let speed = rand::gen_range(50.0, 150.0);

            let pos = vec2(x, y);
            let vel = vec2(0.0, speed);
            let shape = Shape { x, y, size };
            world.add_entity(Enemy { pos, vel, shape });
        }

        // move enemies
        world.run(
            |mut vm_enemies: ViewMut<Enemy>, mut vm_to_delete: ViewMut<ToDelete>| {
                for (entity_id, enemy) in (&mut vm_enemies).iter().with_id() {
                    enemy.pos += enemy.vel * delta_time;

                    enemy.shape.x = enemy.pos.x;
                    enemy.shape.y = enemy.pos.y;

                    if enemy.pos.y > screen_height() + enemy.shape.size {
                        vm_to_delete.add_component_unchecked(entity_id, ToDelete);
                    }
                }
            },
        );

        // move bullets
        world.run(
            |mut vm_bullets: ViewMut<Bullet>, mut vm_to_delete: ViewMut<ToDelete>| {
                for (entity_id, bullet) in (&mut vm_bullets).iter().with_id() {
                    bullet.pos += bullet.vel * delta_time;

                    bullet.shape.x = bullet.pos.x;
                    bullet.shape.y = bullet.pos.y;

                    if bullet.pos.y < 0.0 - bullet.shape.size / 2.0 {
                        vm_to_delete.add_component_unchecked(entity_id, ToDelete);
                    }
                }
            },
        );

        // check for collisions
        world.run(
            |v_bullets: View<Bullet>,
             v_enemies: View<Enemy>,
             mut vm_to_delete: ViewMut<ToDelete>| {
                for (bullet_id, bullet) in v_bullets.iter().with_id() {
                    for (enemy_id, enemy) in v_enemies.iter().with_id() {
                        if bullet.shape.collides_with(&enemy.shape) {
                            // mark bullet and enemy for deletion
                            vm_to_delete.add_component_unchecked(bullet_id, ToDelete);
                            vm_to_delete.add_component_unchecked(enemy_id, ToDelete);
                        }
                    }
                }
            },
        );

        // clean up entities
        world.run(|mut all_storages: AllStoragesViewMut| {
            all_storages.delete_any::<SparseSet<ToDelete>>();
        });

        // draw bullets
        world.run(|v_bullets: View<Bullet>| {
            for bullet in v_bullets.iter() {
                let shape = &bullet.shape;
                draw_circle(shape.x, shape.y, shape.size / 2.0, BLUE);

                let pos = &bullet.pos;
                draw_circle(pos.x, pos.y, 2.0, RED)
            }
        });

        // draw player
        world.run(|player: UniqueView<Player>| {
            let shape = &player.shape;
            draw_circle(shape.x, shape.y, shape.size / 2.0, YELLOW);

            let pos = &player.pos;
            draw_circle(pos.x, pos.y, 2.0, RED);
        });

        // draw enemies
        world.run(|v_enemies: View<Enemy>| {
            for enemy in v_enemies.iter() {
                let shape = &enemy.shape;
                draw_rectangle(
                    shape.x - shape.size / 2.0,
                    shape.y - shape.size / 2.0,
                    shape.size,
                    shape.size,
                    GREEN,
                );

                let pos = &enemy.pos;
                draw_circle(pos.x, pos.y, 2.0, RED);
            }
        });

        next_frame().await
    }
}
