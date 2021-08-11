use crate::logic::main::get_slope;
use crate::Ray;
use crate::Wall;
use macroquad::prelude::*;
use std::cmp::Ordering;
pub struct Sprite {
    pub pos: Vec2,
    pub radius: f32,
    // pub angle: f32,
    pub wall: Wall,
}

impl Sprite {
    pub fn new(
        x: f32,
        y: f32,
        z_offset: f32,
        radius: f32,
        color: Vec4,
        texture: Texture2D,
    ) -> Sprite {
        Sprite {
            pos: vec2(x, y),
            radius,
            wall: Wall {
                x1: x,
                y1: y,
                x2: x + radius,
                y2: y,
                color,
                texture,
                z_offset,
            },
        }
    }

    pub fn get_sprite_ray_collision<'sprites>(
        ray: &Ray,
        sprites: &'sprites Vec<Sprite>,
    ) -> Vec<(&'sprites Wall, f32, Vec2)> {
        let mut colls: Vec<(&'sprites Wall, f32, Vec2)> = sprites
            .iter()
            .filter_map(|s| Wall::get_intersection(ray, &s.wall))
            .collect();

        colls.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        colls
    }

    pub fn draw(&self) {
        self.wall.draw();
    }

    pub fn update_sprites_walls(sprites: &mut Vec<Sprite>, camera_position: Vec2) {
        sprites.iter_mut().for_each(|s| {
            let angle = -1.0 * get_slope(s.pos, camera_position).atan();
            let (angle_cos, angle_sin) = (angle.cos(), angle.sin());

            // if angle > 180.0 {
            if camera_position.x < s.pos.x {
                s.wall.x1 = s.pos.x - s.radius * angle_sin;
                s.wall.y1 = s.pos.y - s.radius * angle_cos;
                s.wall.x2 = s.pos.x + s.radius * angle_sin;
                s.wall.y2 = s.pos.y + s.radius * angle_cos;
            } else {
                s.wall.x1 = s.pos.x + s.radius * angle_sin;
                s.wall.y1 = s.pos.y + s.radius * angle_cos;
                s.wall.x2 = s.pos.x - s.radius * angle_sin;
                s.wall.y2 = s.pos.y - s.radius * angle_cos;
            }
        })
    }
}
