use crate::logic::main::{get_distance, get_intersection, get_polygon_lines};
use crate::Ray;
use crate::Shape;

use macroquad::prelude::*;
const WALL_THICKNESS: f32 = 1.0;
pub struct Wall {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: Vec4,
    pub texture: Texture2D,
    pub z_offset: f32,
}

impl Wall {
    pub fn draw(&self) {
        draw_line(
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            WALL_THICKNESS,
            Color::from_vec(self.color),
        );
    }

    pub fn get_pts(&self) -> (f32, f32, f32, f32) {
        (self.x1, self.y1, self.x2, self.y2)
    }

    pub fn slope(&self) -> f32 {
        (self.y2 - self.y1) / (self.x2 - self.x1)
    }

    pub fn get_start(&self) -> Vec2 {
        vec2(self.x1, self.y1)
    }
    pub fn get_end(&self) -> Vec2 {
        vec2(self.x2, self.y2)
    }

    pub fn polygon(
        x: f32,
        y: f32,
        sides: u8,
        radius: f32,
        rotation: f32,
        color: Vec4,
        texture: Texture2D,
    ) -> Vec<Wall> {
        get_polygon_lines(x, y, sides, radius, rotation)
            .iter()
            .map(|(x1, y1, x2, y2)| Wall {
                x1: *x1,
                y1: *y1,
                x2: *x2,
                y2: *y2,
                color,
                texture,
                z_offset: 0.0,
            })
            .collect()
    }

    pub fn check_intersection(&self, ray: &Ray) -> Option<Vec2> {
        return get_intersection(ray.get_pts(), self.get_pts());
    }

    pub fn get_length(&self) -> f32 {
        get_distance(self.get_start(), self.get_end())
    }

    pub fn get_closest_intersection_shapes<'shape>(
        ray: &Ray,
        shapes: &'shape Vec<Box<dyn Shape>>,
    ) -> Option<(&'shape Wall, f32, Vec2)> {
        let mut closest: Option<(&Wall, f32, Vec2)> = None;

        shapes.into_iter().for_each(|shape| {
            if let Some(intersection_data) = shape.get_intersection(ray) {
                match closest {
                    Some(c) => {
                        if intersection_data.1 < c.1 {
                            closest = Some(intersection_data);
                        }
                    }
                    None => closest = Some(intersection_data),
                }
            }
        });
        closest
    }

    pub fn get_closest_intersection<'shape>(
        ray: &Ray,
        walls: &'shape Vec<Wall>,
    ) -> Option<(&'shape Wall, f32, Vec2)> {
        let mut closest: Option<(&'shape Wall, f32, Vec2)> = None;
        let ray_s = ray.get_start();

        walls.iter().for_each(|wall| {
            if let Some(point) = wall.check_intersection(ray) {
                let d = get_distance(ray_s, point);
                match closest {
                    Some(c) => {
                        if d < c.1 {
                            closest = Some((wall, d, point))
                        }
                    }
                    None => closest = Some((wall, d, point)),
                }
            }
        });

        closest
    }

    pub fn get_intersection<'shape>(
        ray: &Ray,
        wall: &'shape Wall,
    ) -> Option<(&'shape Wall, f32, Vec2)> {
        match wall.check_intersection(ray) {
            Some(point) => Some((wall, get_distance(ray.get_start(), point), point)),
            None => None,
        }
    }
}
