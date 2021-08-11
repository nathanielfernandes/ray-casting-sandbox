pub mod display;
pub mod logic;
pub mod player;
pub mod ray;
pub mod shapes;
pub mod sprite;
pub mod wall;

use crate::display::Display;
use crate::player::Player;
use crate::ray::Ray;
use crate::shapes::{Shape, Square};
use crate::sprite::Sprite;
use crate::wall::Wall;

use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
};

const INITIAL_FOV: u32 = 60;

const INITIAL_RAY_COUNT: u32 = 500;
const INITIAL_RAY_RADIUS: f32 = 3000.0;

const RAY_THICKNESS: f32 = 0.5;

const BACKGROUND_COLOR: macroquad::color::Color = BLACK;

fn window_conf() -> Conf {
    Conf {
        window_title: "Nathans Game".to_owned(),
        window_height: 1080,
        window_width: 1920,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // screen coords
    let (sw, sh) = (screen_width(), screen_height());
    let (cx, cy) = (sw / 2.0, sh / 2.0);

    let mut player = Player {
        pos: Vec2::new(cx - cx / 2.0, cy),
        rot: 0.0,
        vel: Vec2::new(0.0, 0.0),
    };

    let texture_stonewall: Texture2D = load_texture("./textures/stonewall.png").await.unwrap();
    let texture_oakplanks: Texture2D = load_texture("./textures/woodenplanks.png").await.unwrap();
    let texture_stonebricks: Texture2D = load_texture("./textures/stonebricks.png").await.unwrap();
    let texture_oli: Texture2D = load_texture("./textures/oil.png").await.unwrap();

    let texture_sprite: Texture2D = load_texture("./textures/sprite.png").await.unwrap();
    let texture_barrel: Texture2D = load_texture("./textures/barrel.png").await.unwrap();
    let texture_lamp: Texture2D = load_texture("./textures/lamp.png").await.unwrap();
    let texture_armor: Texture2D = load_texture("./textures/armor.png").await.unwrap();

    let mut current_texture = texture_stonewall;
    let mut current_sprite_texture = texture_barrel;
    // walls
    // let mut walls: Vec<Wall> = Vec::new();
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    let mut sprites: Vec<Sprite> = Vec::new();

    let display = Display {
        x: cx,
        y: 0.0,
        w: cx,
        h: sh,
    };

    let mut debug: bool = false;

    let mut ray_count: f32 = INITIAL_RAY_COUNT as f32;
    let mut fov: f32 = INITIAL_FOV as f32;
    let mut radius: f32 = INITIAL_RAY_RADIUS;

    let mut last_ray_count: f32 = 0.0;
    let mut last_fov: f32 = 0.0;
    let mut last_radius: f32 = 0.0;

    let mut fov_ratio = 0.0;
    let mut rot_offset = 0.0;

    let mut rays: Vec<Ray> = Vec::new();

    let mut use_eulidean = false;

    let mut wall_height = cx;

    let mut original = false;

    let mut placed_coords: Vec<(f32, f32)> = Vec::new();
    let mut placed_sprites: Vec<(f32, f32)> = Vec::new();

    let ray_color = Color::from_vec(vec4(1.0, 1.0, 1.0, 0.1));

    let mut use_new: bool = true;

    loop {
        // spawing rays and setting contants
        if ray_count != last_ray_count || fov != last_fov || radius != last_radius {
            fov_ratio = ray_count / fov as f32;
            rot_offset = -90.0 - (fov as f32 / 2.0); //-0.5 * FOV as f32 - 90.0;
                                                     // spawn rays
            rays = Ray::spawn_rays((ray_count).round() as usize, cx, cy);

            last_ray_count = ray_count;
            last_fov = fov;
            last_radius = radius;
        }

        if is_key_pressed(KeyCode::Space) {
            debug = if debug { false } else { true };
        }

        if is_key_pressed(KeyCode::N) {
            use_new = if use_new { false } else { true };
        }

        let (mx, my) = mouse_position();
        let pts = (20.0 * (mx / 20.0).round(), 20.0 * (my / 20.0).round());
        let spts = (10.0 * (mx / 10.0).round(), 10.0 * (my / 10.0).round());

        if !debug && is_mouse_button_pressed(MouseButton::Left) {
            if is_key_down(KeyCode::LeftShift) {
                if !placed_sprites.contains(&spts) {
                    sprites.push(Sprite::new(
                        spts.0,
                        spts.1,
                        0.0,
                        10.0,
                        vec4(1.0, 1.0, 1.0, 1.0),
                        current_sprite_texture,
                    ));
                    placed_sprites.push(spts);
                } else {
                    let i = placed_sprites.iter().position(|&pt| pt == spts).unwrap();
                    placed_sprites.remove(i);
                    sprites.remove(i);
                }
            } else {
                if !placed_coords.contains(&pts) {
                    shapes.push(Box::new(Square::new(
                        pts.0,
                        pts.1,
                        vec4(0.5, 0.5, 0.5, 1.0),
                        current_texture,
                    )));
                    placed_coords.push(pts);
                } else {
                    let i = placed_coords.iter().position(|&pt| pt == pts).unwrap();
                    placed_coords.remove(i);
                    shapes.remove(i);
                }
            }
        }

        // player logic
        player.update(&shapes);
        Sprite::update_sprites_walls(&mut sprites, player.pos);

        // START DRAWING HERE
        clear_background(BACKGROUND_COLOR);

        if !debug {
            if is_key_down(KeyCode::LeftShift) {
                let scolor = if placed_sprites.contains(&spts) {
                    RED
                } else {
                    WHITE
                };
                draw_circle(spts.0, spts.1, 4.0, scolor);
            } else {
                let color = if placed_coords.contains(&pts) {
                    RED
                } else {
                    WHITE
                };
                draw_poly_lines(pts.0, pts.1, 4, 14.14214, 45.0, 1.0, color);
            }
        }

        //draw_poly(1100.0, 500.0, 4, 710.0, 45.0, BLACK);

        Ray::update_rays(
            &mut rays,
            player.pos.x,
            player.pos.y,
            player.rot,
            rot_offset,
            radius,
            fov,
            original,
        );
        // TODO two loops make screen object

        let mut wall_slits: Vec<Option<(f32, Vec2, &Wall)>> = Vec::new();
        let mut sprite_slits: Vec<Vec<(f32, Vec2, &Wall)>> = Vec::new();

        rays.iter().for_each(|ray| {
            let raw_angle_cos = ((ray.i as f32) / fov_ratio + rot_offset + 90.0)
                .to_radians()
                .cos();
            let camera_angle = ((ray.i as f32) / fov_ratio + player.rot + rot_offset).to_radians();
            let wall_height_fov = wall_height / fov;

            let mut ray_draw_distance: f32 = radius;
            let mut end_pt: Vec2 = ray.get_end();
            let wall_collision = match Wall::get_closest_intersection_shapes(ray, &shapes) {
                Some((wall, euclidean_distance, point)) => {
                    let mut distance = euclidean_distance;
                    if !use_eulidean {
                        if original {
                            distance *= raw_angle_cos;
                        } else {
                            distance *= radius / ray.length();
                        }
                    }
                    end_pt = point;
                    ray_draw_distance = distance;
                    if debug {
                        draw_circle(
                            ray.x1 + distance * camera_angle.cos(),
                            ray.y1 + distance * camera_angle.sin(),
                            2.0,
                            GREEN,
                        );
                    }

                    Some(((wall_height_fov / distance).abs(), point, wall))
                }
                None => None,
            };

            draw_line(ray.x1, ray.y1, end_pt.x, end_pt.y, RAY_THICKNESS, ray_color);

            wall_slits.push(wall_collision);

            let collisions: Vec<(f32, Vec2, &Wall)> =
                Sprite::get_sprite_ray_collision(ray, &sprites)
                    .into_iter()
                    .filter_map(|(wall, euclidean_distance, point)| {
                        let mut distance = euclidean_distance;
                        if !use_eulidean {
                            if original {
                                distance *= raw_angle_cos;
                            } else {
                                distance *= radius / ray.length();
                            }
                        }
                        let slit = if distance < ray_draw_distance {
                            Some(((wall_height_fov / distance).abs(), point, wall))
                        } else {
                            None
                        };
                        if debug {
                            if let Some(_) = slit {
                                draw_circle(
                                    ray.x1 + distance * camera_angle.cos(),
                                    ray.y1 + distance * camera_angle.sin(),
                                    2.0,
                                    BLUE,
                                );
                            }
                        }
                        slit
                    })
                    .collect();

            sprite_slits.push(collisions)
        });

        display.draw(wall_slits, sprite_slits, player.pos, use_new);

        shapes.iter().for_each(|s| s.draw());
        sprites.iter().for_each(|s| s.draw());
        player.draw();

        // settings menu
        if debug {
            if original {
                draw_poly_lines(player.pos.x, player.pos.y, 64, radius, 0.0, 1.0, GREEN);
            }

            let fps = get_fps();
            draw_text(&format!("fps: {}", fps), 2.0, 20.0, 30.0, GREEN);
            draw_text(
                &format!(
                    "rays: {} radius: {} shapes: {} sprites: {}",
                    rays.len(),
                    radius as u32,
                    shapes.len(),
                    sprites.len()
                ),
                130.0,
                20.0,
                30.0,
                GREEN,
            );
            draw_text(
                &format!(
                    "player: x={} y={} r={} fov={}",
                    player.pos.x as i32, player.pos.y as i32, player.rot as i32, fov as u32
                ),
                2.0,
                50.0,
                30.0,
                GREEN,
            );
            draw_text(
                &format!(
                    "mouse: x={} y={}  using_new:{}",
                    mx as i32, my as i32, use_new
                ),
                2.0,
                100.0,
                30.0,
                GREEN,
            );

            widgets::Window::new(hash!(), vec2(0.0, cy), vec2(cx / 2.0, 200.0))
                .label("Camera Settings")
                .ui(&mut *root_ui(), |ui| {
                    ui.slider(hash!(), "FOV", 1.0f32..360f32, &mut fov);
                    ui.slider(hash!(), "RAY COUNT", 1.0f32..1000.0f32, &mut ray_count);
                    ui.slider(hash!(), "RADIUS", 0.1f32..5000.0f32, &mut radius);
                    ui.slider(hash!(), "WALL HEIGHT", 0.1f32..10000.0f32, &mut wall_height);
                    ui.label(
                        None,
                        "Corrected Raycasting removes distortion but has a max FOV of 180",
                    );
                    let usage = if original {
                        "Using Natural Ray Casting"
                    } else {
                        "Using Corrected Raycasting"
                    };
                    if ui.button(None, usage) {
                        original = if original { false } else { true };
                    }

                    ui.label(
                        None,
                        "Use Euclidean Distance to Fix Distortion At FOVs above 360",
                    );
                    let usage = if use_eulidean {
                        "Using Euclidean Distance"
                    } else {
                        "Using Corrected Distance"
                    };
                    if ui.button(None, usage) {
                        use_eulidean = if use_eulidean { false } else { true };
                    }
                    ui.separator();
                });

            widgets::Window::new(hash!(), vec2(0.0, cy + cy / 2.0), vec2(cx / 2.0, 200.0))
                .label("Textures")
                .ui(&mut *root_ui(), |ui| {
                    ui.label(None, "Shape Textures");
                    if ui.button(None, "Stone Wall") {
                        current_texture = texture_stonewall
                    }
                    if ui.button(None, "Oak Planks") {
                        current_texture = texture_oakplanks
                    }
                    if ui.button(None, "Stone Bricks") {
                        current_texture = texture_stonebricks
                    }
                    if ui.button(None, "Oli") {
                        current_texture = texture_oli
                    }
                    ui.separator();
                    ui.label(None, "Sprite Textures");
                    if ui.button(None, "Barrel") {
                        current_sprite_texture = texture_barrel
                    }
                    if ui.button(None, "Lamp") {
                        current_sprite_texture = texture_lamp
                    }
                    if ui.button(None, "Sprite") {
                        current_sprite_texture = texture_sprite
                    }
                    if ui.button(None, "Armor") {
                        current_sprite_texture = texture_armor
                    }
                });

            draw_texture_ex(
                current_texture,
                cx - 70.0,
                8.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    source: None,
                    rotation: 0.0,
                    pivot: None,
                    flip_x: false,
                    flip_y: false,
                },
            );

            draw_texture_ex(
                current_sprite_texture,
                cx - 70.0,
                80.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    source: None,
                    rotation: 0.0,
                    pivot: None,
                    flip_x: false,
                    flip_y: false,
                },
            );
        }

        next_frame().await
    }
}
