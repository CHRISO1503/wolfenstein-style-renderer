use std::f32::consts::PI;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::math::line_2d::Line2D;
use crate::math::rotation::Quaternion;
use crate::player::Player;
use crate::{level, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render(canvas: &mut WindowCanvas, player: &Player) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    const FOV: f32 = PI / 2.0;
    let cartesian_forward = Quaternion::new(0.0, 0.0, 1.0, 0.0)
        .rotate(player.rotation)
        .to_euler();

    for i in 0..WINDOW_WIDTH {
        // cast ray
        let ray_direction = cartesian_forward.y - FOV / 2.0 + FOV * i as f32 / WINDOW_WIDTH as f32;
        let m = ray_direction.tan();
        let c = player.pos.2 - m * player.pos.0;
        let ray_line = Line2D::new(m, c);

        let xdir;
        if ray_direction % 2.0 * PI < PI {
            xdir = 1.0;
        } else {
            xdir = -1.0;
        }

        let ydir;
        if ray_direction % 2.0 * PI < 3.0 * PI / 2.0 && ray_direction > PI / 2.0 {
            ydir = -1.0;
        } else {
            ydir = 1.0;
        }

        let mut ray_pos = (player.pos.0, player.pos.2);
        if xdir > 0.0 {
            ray_pos.0 = player.pos.0.ceil();
        } else {
            ray_pos.0 = player.pos.0.floor();
        }

        let mut hit_x: f32 = 1e30;
        let mut hit_value: u8 = 0;
        loop {
            if ray_pos.0 > level::MAP[0].len() as f32
                || ray_pos.1 > level::MAP.len() as f32
                || ray_pos.0 < 0.0
                || ray_pos.1 < 0.0
            {
                break;
            }

            ray_pos.1 = ray_line.y_from_x(ray_pos.0);

            let hit = level::MAP[ray_pos.1.floor() as usize]
                [level::MAP[0].len() - ray_pos.0.round() as usize];
            if hit > 0 {
                hit_x = ray_pos.0;
                hit_value = hit;
                break;
            }

            ray_pos.0 += xdir;
        }

        ray_pos = (player.pos.0, player.pos.2);
        if ydir > 0.0 {
            ray_pos.1 = player.pos.2.ceil();
        } else {
            ray_pos.1 = player.pos.2.floor();
        }

        loop {
            ray_pos.0 = ray_line.x_from_y(ray_pos.1);

            if ray_pos.0 > level::MAP[0].len() as f32 - 1.0
                || ray_pos.1 > level::MAP.len() as f32 - 1.0
                || ray_pos.0 < 0.0
                || ray_pos.1 < 0.0
            {
                break;
            }

            let hit = level::MAP[ray_pos.1.round() as usize]
                [level::MAP[0].len() - ray_pos.0.floor() as usize];
            if hit > 0 {
                if ray_pos.0 < hit_x {
                    hit_x = ray_pos.0;
                    hit_value = hit;
                    break;
                }
            }

            ray_pos.1 += ydir;
        }

        // set wall height with
        if hit_value > 0 {
            let color: Color;
            match hit_value {
                1 => color = Color::RED,
                2 => color = Color::YELLOW,
                3 => color = Color::GREEN,
                4 => color = Color::BLUE,
                _ => color = Color::MAGENTA,
            }
            let hit_distance = ((ray_line.y_from_x(hit_x) - player.pos.2).powf(2.0)
                + (hit_x - player.pos.0).powf(2.0))
            .powf(0.5);
            let h;
            if hit_distance.abs() > 1e-20 {
                h = WINDOW_HEIGHT as f32 / hit_distance;
            } else {
                h = f32::MAX;
            }
            let y1 = ((WINDOW_HEIGHT as i32 / 2) - h as i32 / 2).max(0);
            let y2 = ((WINDOW_HEIGHT as i32 / 2) + h as i32 / 2).min(WINDOW_HEIGHT as i32 - 1);
            canvas.set_draw_color(color);
            draw_column(canvas, i as i32, y1, y2);
        }
    }

    canvas.present();
}

fn draw_column(canvas: &mut WindowCanvas, x: i32, y1: i32, y2: i32) {
    for i in y1..y2 {
        let _ = canvas.draw_point(Point::new(x, i));
    }
}
