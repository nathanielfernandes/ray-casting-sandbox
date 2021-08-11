use crate::Shape;
use macroquad::prelude::*;
pub struct Player {
    pub pos: Vec2,
    pub rot: f32,
    pub vel: Vec2,
}

impl Player {
    pub fn update(&mut self, shapes: &Vec<Box<dyn Shape>>) {
        let rotation = self.rot.to_radians();
        let mut acc = -self.vel / 2.0;

        if is_key_down(KeyCode::W) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / 2.;
        } else if is_key_down(KeyCode::S) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / -2.;
        }

        if is_key_down(KeyCode::D) {
            self.rot += 1.8;
        } else if is_key_down(KeyCode::A) {
            self.rot -= 1.8;
        }

        self.vel += acc;
        if self.vel.length() > 1.5 {
            self.vel = self.vel.normalize() * 1.5;
        }

        let new_pos = self.pos + self.vel;

        if !shapes
            .iter()
            .any(|shape| shape.check_collision(new_pos, 2.0))
        {
            self.pos = new_pos
        } else {
            self.vel.x = 0.0;
            self.vel.y = 0.0;
        }
    }

    pub fn draw(&self) {
        draw_poly(self.pos.x, self.pos.y, 3, 6.0, self.rot + 30.0, RED);
    }
}
