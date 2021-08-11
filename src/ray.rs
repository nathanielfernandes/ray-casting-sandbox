use crate::logic::main::get_distance;
use macroquad::prelude::*;
pub struct Ray {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub i: usize,
}

// TODO REMOVE RADIUS
impl Ray {
    pub fn spawn_rays(count: usize, x: f32, y: f32) -> Vec<Ray> {
        (0..count)
            .map(|i| Ray {
                x1: x,
                y1: y,
                x2: 0.0,
                y2: 0.0,
                i,
            })
            .collect()
    }

    pub fn start(&mut self, x1: f32, y1: f32) {
        self.x1 = x1;
        self.y1 = y1;
    }

    pub fn end(&mut self, x2: f32, y2: f32) {
        self.x2 = x2;
        self.y2 = y2;
    }

    pub fn get_start(&self) -> Vec2 {
        vec2(self.x1, self.y1)
    }
    pub fn get_end(&self) -> Vec2 {
        vec2(self.x2, self.y2)
    }

    fn update_pos_standard(&mut self, x: f32, y: f32, rot: f32, radius: f32, fov_ratio: f32) {
        self.start(x, y);
        let rads: f32 = ((self.i as f32) * fov_ratio + rot).to_radians();
        self.end(x + radius * rads.cos(), y + radius * rads.sin());
    }

    fn update_pos_fps(
        &mut self,
        x: f32,
        y: f32,
        new_x: f32,
        new_y: f32,
        segment_length: f32,
        angle_sin: f32,
        angle_cos: f32,
    ) {
        self.start(x, y);
        self.end(
            new_x + (self.i as f32 * segment_length) * -angle_sin,
            new_y + (self.i as f32 * segment_length) * angle_cos,
        )
    }

    pub fn update_rays(
        rays: &mut Vec<Ray>,
        x: f32,
        y: f32,
        rot: f32,
        rot_offset: f32,
        radius: f32,
        fov: f32,
        original: bool,
    ) {
        if original {
            let fov_ratio = fov / rays.len() as f32;
            rays.iter_mut()
                .for_each(|r| r.update_pos_standard(x, y, rot + rot_offset, radius, fov_ratio))
        } else {
            let b = radius * (fov / 2.0).to_radians().tan();
            let segment_length = (b * 2.0) / (rays.len() as f32 - 1.0);
            let angle = (rot - 90.0).to_radians();
            let angle_cos = angle.cos();
            let angle_sin = angle.sin();

            let new_x = (x + radius * angle_cos) - b * -angle_sin;
            let new_y = (y + radius * angle_sin) - b * angle_cos;

            rays.iter_mut().for_each(|r| {
                r.update_pos_fps(x, y, new_x, new_y, segment_length, angle_sin, angle_cos);
            });
        }
    }

    pub fn get_pts(&self) -> (f32, f32, f32, f32) {
        (self.x1, self.y1, self.x2, self.y2)
    }

    pub fn length(&self) -> f32 {
        get_distance(self.get_start(), self.get_end())
    }
}
