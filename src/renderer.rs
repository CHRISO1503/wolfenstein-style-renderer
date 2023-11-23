use std::f32::consts::PI;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::math::line_2d::Line2D;
use crate::player::Player;
use crate::{level, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render(canvas: &mut WindowCanvas, player: &Player) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    const FOV: f32 = PI / 2.0;

    for i in 0..WINDOW_WIDTH {
        // cast ray
        let ray_direction = (player.rotation - FOV / 2.0 + FOV * i as f32 / WINDOW_WIDTH as f32)
            .rem_euclid(2.0 * PI);

        let xdir;
        if ray_direction < PI {
            xdir = 1;
        } else {
            xdir = -1;
        }
        let ydir;
        if ray_direction > PI / 2.0 && ray_direction < 3.0 * PI / 2.0 {
            ydir = -1;
        } else {
            ydir = 1;
        }

        let m = -(ray_direction - PI / 2.0).tan();
        let c = player.pos.z - m * player.pos.x;
        let ray_line = Line2D::new(m, c);

        let grid_step_dists = (m.abs(), 1.0 / m.abs());

        let mut ray_dists = (
            if xdir == 1 {
                ((player.pos.x.ceil() - player.pos.x).powf(2.0)
                    + (ray_line.y_from_x(player.pos.x.ceil()) - player.pos.y).powf(2.0))
                .sqrt()
            } else {
                ((player.pos.x.floor() - player.pos.x).powf(2.0)
                    + (ray_line.y_from_x(player.pos.x.floor()) - player.pos.y).powf(2.0))
                .sqrt()
            },
            if ydir == 1 {
                ((player.pos.x - ray_line.x_from_y(player.pos.y.ceil())).powf(2.0)
                    + (player.pos.y.ceil() - player.pos.y).powf(2.0))
                .sqrt()
            } else {
                ((player.pos.x - ray_line.x_from_y(player.pos.y.floor())).powf(2.0)
                    + (player.pos.y.floor() - player.pos.y).powf(2.0))
                .sqrt()
            },
        );

        let player_grid = (
            if xdir == 1 {
                player.pos.x.ceil() as i32
            } else {
                player.pos.x.floor() as i32
            },
            if ydir == 1 {
                player.pos.y.ceil() as i32
            } else {
                player.pos.y.floor() as i32
            },
        );

        let mut ray_grids = (0, 0);

        let mut hit_value: u8 = 0;
        let mut hit_distance;
        loop {
            let step;
            let ray_coords;
            if ray_dists.0 < ray_dists.1 {
                hit_distance = ray_dists.0;
                step = (1, 0);
                ray_coords = (
                    (player_grid.0 + ray_grids.0) as i32,
                    ray_line
                        .y_from_x(player_grid.0 as f32 + ray_grids.0 as f32)
                        .floor() as i32,
                );
            } else {
                //same in y direction
                hit_distance = ray_dists.1;
                step = (0, 1);
                ray_coords = (
                    ray_line
                        .x_from_y(player_grid.1 as f32 + ray_grids.1 as f32)
                        .floor() as i32,
                    (player_grid.1 + ray_grids.1) as i32,
                );
            }

            //check for ray hit
            if level::MAP.len() as i32 - 1 - ray_coords.1 > 0
                && level::MAP.len() as i32 - 1 - ray_coords.1 < level::MAP[0].len() as i32
                && ray_coords.0 > 0
                && ray_coords.0 < level::MAP.len() as i32
            {
                // hit
                hit_value =
                    level::MAP[level::MAP.len() - 1 - ray_coords.1 as usize][ray_coords.0 as usize];
                if hit_value > 0 {
                    break;
                }
            } else {
                break;
            }

            // move to next grid
            ray_grids.0 += step.0 * xdir;
            ray_dists.0 += step.0 as f32 * grid_step_dists.0;
            ray_grids.1 += step.1 * ydir;
            ray_dists.1 += step.1 as f32 * grid_step_dists.1;
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
            let h;
            if hit_distance > 1e-20 {
                h = WINDOW_HEIGHT as f32 / hit_distance;
            } else {
                h = WINDOW_HEIGHT as f32;
            }
            let y0 = ((WINDOW_HEIGHT as i32 / 2) - h as i32 / 2).max(0);
            let y1 = ((WINDOW_HEIGHT as i32 / 2) + h as i32 / 2).min(WINDOW_HEIGHT as i32 - 1);
            canvas.set_draw_color(color);
            draw_column(canvas, i as i32, y0, y1);
        }
    }

    canvas.present();
}

fn draw_column(canvas: &mut WindowCanvas, x: i32, y0: i32, y1: i32) {
    for i in y0..y1 {
        let _ = canvas.draw_point(Point::new(x, i));
    }
}
