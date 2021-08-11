use crate::Ray;
use crate::Wall;
use macroquad::prelude::*;

pub trait Shape {
    fn get_intersection<'shape>(&'shape self, ray: &Ray) -> Option<(&'shape Wall, f32, Vec2)>;

    fn draw(&self);

    fn check_collision(&self, pos: Vec2, radius: f32) -> bool;
}

pub struct Square {
    pub x: f32,
    pub y: f32,
    pub walls: Vec<Wall>,
    pub color: Vec4,
    pub texture: Texture2D,
}

impl Shape for Square {
    fn get_intersection<'shape>(&'shape self, ray: &Ray) -> Option<(&'shape Wall, f32, Vec2)> {
        Wall::get_closest_intersection(ray, &self.walls)
    }

    fn check_collision(&self, pos: Vec2, radius: f32) -> bool {
        let dx: f32 = (pos.x - self.x).abs() - 10.0;
        let dy: f32 = (pos.y - self.y).abs() - 10.0;

        if dx > radius || dy > radius {
            false
        } else if dx <= 0.0 || dy <= 0.0 {
            true
        } else {
            dx * dx + dy * dy <= radius * radius
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.x - 10.0,
            self.y - 10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(20.0, 20.0)),
                source: None,
                rotation: 0.0,
                pivot: None,
                flip_x: false,
                flip_y: false,
            },
        );
    }
}

impl Square {
    pub fn new(x: f32, y: f32, color: Vec4, texture: Texture2D) -> Square {
        Square {
            x,
            y,
            walls: Wall::polygon(x, y, 4, 14.14213562, 45.0, color, texture),
            color,
            texture,
        }
    }
}
