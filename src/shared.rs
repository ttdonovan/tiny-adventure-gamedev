use macroquad::prelude::*;

#[derive(Debug)]
pub struct Pos(Vec2);

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos(vec2(x, y))
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }
}

#[derive(Debug)]
pub struct Shape(Pos, f32);

impl Shape {
    pub fn new(pos: Pos, size: f32) -> Self {
        Shape(pos, size)
    }

    pub fn center(&self) -> Vec2 {
        vec2(self.0.x() + self.1 / 2.0, self.0.y() + self.1 / 2.0)
    }

    pub fn size(&self) -> f32 {
        self.1
    }
}
