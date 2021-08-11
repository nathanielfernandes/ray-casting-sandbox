pub mod main {
    use macroquad::prelude::*;

    pub fn get_polygon_lines(
        x: f32,
        y: f32,
        sides: u8,
        radius: f32,
        rotation: f32,
    ) -> Vec<(f32, f32, f32, f32)> {
        let mut last_x = 0.0;
        let mut last_y = 0.0;

        let mut lines: Vec<(f32, f32, f32, f32)> = (0..sides)
            .map(|i| {
                let rads = (360.0 / sides as f32 * (i as f32) + rotation).to_radians();
                let x2 = x + radius * rads.cos();
                let y2 = y + radius * rads.sin();

                let line = (last_x, last_y, x2, y2);

                last_x = x2;
                last_y = y2;

                line
            })
            .collect();

        lines[0] = (last_x, last_y, lines[0].2, lines[0].3);
        lines
    }

    pub fn get_intersection(
        line1: (f32, f32, f32, f32),
        line2: (f32, f32, f32, f32),
    ) -> Option<Vec2> {
        let (x1, y1, x2, y2) = line1;
        let (x3, y3, x4, y4) = line2;

        let den: f64 = ((x1 - x2) * (y3 - y4)) as f64 - ((y1 - y2) * (x3 - x4)) as f64;

        if den != 0.0 {
            let t: f64 = (((x1 - x3) * (y3 - y4)) - ((y1 - y3) * (x3 - x4))) as f64 / den;
            let u: f64 = -1.0 * (((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) as f64 / den);

            if (t > 0.0) && (t < 1.0) && (u > 0.0) && (u < 1.0) {
                let (px, py) = ((x1 + (t as f32 * (x2 - x1))), (y1 + (t as f32 * (y2 - y1))));
                return Some(vec2(px, py));
            }
        }
        None
    }

    pub fn get_slope(p1: Vec2, p2: Vec2) -> f32 {
        (p2.y - p1.y) / (p2.x - p1.x)
    }

    // pub fn get_closest_intersection(
    //     ray: (f32, f32, f32, f32),
    //     lines: Vec<(f32, f32, f32, f32)>,
    // ) -> Option<(f32, Vec2)> {
    //     let intersections: Vec<Vec2> = lines
    //         .iter()
    //         .filter_map(|line| get_intersection(ray, *line))
    //         .collect();

    //     if intersections.len() > 0 {
    //         return Some(get_closest_point(vec2(ray.0, ray.1), intersections));
    //     }

    //     None
    // }

    // skips sqrt, good enough when comparing distances
    pub fn get_distance_fast(p1: Vec2, p2: Vec2) -> f32 {
        f32::powf(p2.x - p1.x, 2.0).abs() + f32::powf(p2.y - p1.y, 2.0).abs()
    }

    pub fn get_distance(p1: Vec2, p2: Vec2) -> f32 {
        get_distance_fast(p1, p2).sqrt()
    }

    // pub fn get_closest_point(point: Vec2, points: Vec<Vec2>) -> (f32, Vec2) {
    //     let mut closest: (f32, Vec2) = (0.0, vec2(0.0, 0.0));
    //     let mut start: bool = true;
    //     points.iter().for_each(|p| {
    //         let distance = get_distance(point, *p);
    //         if start {
    //             closest = (distance, vec2(p.x, p.y));
    //             start = false;
    //         } else {
    //             if distance < closest.0 {
    //                 closest = (distance, vec2(p.x, p.y));
    //             }
    //         }
    //     });
    //     closest
    // }
}
