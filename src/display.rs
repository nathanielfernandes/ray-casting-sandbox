use crate::logic::main::{get_distance, get_slope};
use crate::Wall;
use macroquad::prelude::*;
use std::collections::BTreeMap;
pub struct Display {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Display {
    pub fn draw(
        &self,
        slits: Vec<Option<(f32, Vec2, &Wall)>>,
        sprite_slits: Vec<Vec<(f32, Vec2, &Wall)>>,
        camera_position: Vec2,
        use_new: bool,
    ) {
        let cy = self.h / 2.0;

        draw_rectangle(self.x, self.y, self.w, cy, DARKGRAY);
        draw_rectangle(self.x, self.y + cy, self.w, cy, GRAY);

        let slit_count = slits.len() as f32;
        let slit_width: f32 = self.w / slit_count;

        slits.iter().enumerate().for_each(|(i, slit)| {
            if let Some((distance_ratio, point, wall)) = slit {
                let slit_height = self.h * distance_ratio;

                let wall_length = wall.get_length();

                let (m1, m2) = (get_slope(camera_position, *point), wall.slope());
                let ray_angle = m1.atan();

                let brightness = 0.2 + (ray_angle - m2.atan()).abs();

                let start_d = get_distance(*point, wall.get_start());

                let wall_len_ratio = start_d / wall_length;
                let texture_width = wall.texture.width();

                let texture_slit_width = texture_width / wall_length / slit_count;

                let dest_size = vec2(slit_width, slit_height);
                let source = Rect::new(
                    wall_len_ratio * texture_width + i as f32 * texture_slit_width,
                    0.0,
                    texture_slit_width,
                    texture_width,
                );

                draw_texture_ex(
                    wall.texture,
                    self.x + (i as f32 * slit_width),
                    self.y + cy - slit_height / 2.0,
                    Color::from_vec(vec4(brightness, brightness, brightness, 1.0)),
                    DrawTextureParams {
                        dest_size: Some(dest_size),
                        source: Some(source),
                        rotation: 0.0,
                        pivot: None,
                        flip_x: false,
                        flip_y: false,
                    },
                )
            }
        });

        if use_new {
            //let mut re_ordered_sprite_slits: Vec<Vec<(f32, Vec2, &Wall)>> = Vec::new();
            let mut ordered_sprite_slits: BTreeMap<usize, Vec<(usize, f32, Vec2, &Wall)>> =
                BTreeMap::new();

            sprite_slits.iter().enumerate().for_each(|(j, slits)| {
                slits.iter().enumerate().for_each(|(i, slit)| {
                    let (a, b, c) = *slit;
                    if let Some(inner_slits) = ordered_sprite_slits.get_mut(&i) {
                        inner_slits.push((j, a, b, c));
                    } else {
                        ordered_sprite_slits.insert(i, vec![(j, a, b, c)]);
                    }
                });
            });

            ordered_sprite_slits.values().for_each(|slits| {
                slits
                    .iter()
                    .for_each(|(i, distance_ratio_s, point_s, wall_s)| {
                        let i = *i as f32;
                        let slit_height = self.h * distance_ratio_s;

                        let wall_length = wall_s.get_length();

                        let wall_start = wall_s.get_start();

                        let start_d = get_distance(*point_s, wall_start);

                        let wall_len_ratio = start_d / wall_length;
                        let texture_width = wall_s.texture.width();

                        let texture_slit_width = texture_width / wall_length / slit_count;

                        let dest_size = vec2(slit_width, slit_height);
                        let source = Rect::new(
                            wall_len_ratio * texture_width + i * texture_slit_width,
                            0.0,
                            texture_slit_width,
                            texture_width,
                        );
                        draw_texture_ex(
                            wall_s.texture,
                            self.x + (i * slit_width),
                            self.y + cy - slit_height / 2.0,
                            //WHITE,
                            Color::from_vec(vec4(1.0, 1.0, 1.0, 2.0 - distance_ratio_s)),
                            DrawTextureParams {
                                dest_size: Some(dest_size),
                                source: Some(source),
                                rotation: 0.0,
                                pivot: None,
                                flip_x: false,
                                flip_y: false,
                            },
                        )
                    });
            });
        } else {
            sprite_slits.iter().enumerate().for_each(|(i, slits)| {
                slits
                    .iter()
                    .for_each(|(distance_ratio_s, point_s, wall_s)| {
                        let slit_height = self.h * distance_ratio_s;

                        let wall_length = wall_s.get_length();

                        let wall_start = wall_s.get_start();

                        let start_d = get_distance(*point_s, wall_start);

                        let wall_len_ratio = start_d / wall_length;
                        let texture_width = wall_s.texture.width();

                        let texture_slit_width = texture_width / wall_length / slit_count;

                        let dest_size = vec2(slit_width, slit_height);
                        let source = Rect::new(
                            wall_len_ratio * texture_width + i as f32 * texture_slit_width,
                            0.0,
                            texture_slit_width,
                            texture_width,
                        );

                        draw_texture_ex(
                            wall_s.texture,
                            self.x + (i as f32 * slit_width),
                            self.y + cy - slit_height / 2.0,
                            //WHITE,
                            Color::from_vec(vec4(1.0, 1.0, 1.0, 2.0 - distance_ratio_s)),
                            DrawTextureParams {
                                dest_size: Some(dest_size),
                                source: Some(source),
                                rotation: 0.0,
                                pivot: None,
                                flip_x: false,
                                flip_y: false,
                            },
                        )
                    });
            });
        }
    }
}
