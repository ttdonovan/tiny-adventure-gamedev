use macroquad::prelude::*;

type Pos = Vec2;

#[derive(Debug)]
pub struct Shape {
    pub pos: Pos,
    pub size: f32,
}

impl Shape {
    pub fn new(pos: Pos, size: f32) -> Self {
        Shape { pos, size }
    }

    pub fn center(&self) -> Vec2 {
        vec2(self.pos.x + self.size / 2.0, self.pos.y + self.size / 2.0)
    }
}
