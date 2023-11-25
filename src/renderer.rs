use std::f32::consts::PI;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use crate::math::vector::{Vec2, Vec2i};
use crate::player::Player;
use crate::{level, WINDOW_HEIGHT, WINDOW_WIDTH};

struct Rayhit {
    value: u8,
    side: i32,
    pos: Vec2,
}

pub fn render(canvas: &mut WindowCanvas, player: &Player) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    const FOV: f32 = PI / 2.0;
    let player_direction = Vec2::new(0.0, 1.0).rotate(player.rotation);

    for x in 0..WINDOW_WIDTH {
        let ray_direction =
            player_direction.rotate(-FOV / 2.0 + x as f32 / WINDOW_WIDTH as f32 * FOV);
        let mut ipos = Vec2i::new(player.pos.x as i32, player.pos.y as i32);

        let deltadist = Vec2::new(
            if ray_direction.x.abs() < 1e-20 {
                1e30
            } else {
                (1.0 / ray_direction.x).abs()
            },
            if ray_direction.y.abs() < 1e-20 {
                1e30
            } else {
                (1.0 / ray_direction.y).abs()
            },
        );

        let mut sidedist = Vec2::new(
            if ray_direction.x < 0.0 {
                deltadist.x * (player.pos.x - ipos.x as f32)
            } else {
                deltadist.x * (ipos.x as f32 + 1.0 - player.pos.x)
            },
            if ray_direction.y < 0.0 {
                deltadist.y * (player.pos.y - ipos.y as f32)
            } else {
                deltadist.y * (ipos.y as f32 + 1.0 - player.pos.y)
            },
        );

        let step = Vec2i::new(
            if ray_direction.x > 0.0 { 1 } else { -1 },
            if ray_direction.y > 0.0 { 1 } else { -1 },
        );

        let mut hit = Rayhit {
            value: 0,
            side: 0,
            pos: Vec2::new(0.0, 0.0),
        };

        while hit.value == 0 {
            if sidedist.x < sidedist.y {
                sidedist.x += deltadist.x;
                ipos.x += step.x;
                hit.side = 0;
            } else {
                sidedist.y += deltadist.y;
                ipos.y += step.y;
                hit.side = 1;
            }

            assert!(
                ipos.x >= 0
                    && ipos.x < level::MAP[0].len() as i32
                    && ipos.y >= 0
                    && ipos.y < level::MAP.len() as i32,
                "Out of bounds"
            );

            hit.value = level::MAP[ipos.y as usize][ipos.x as usize];
        }

        let mut color = match hit.value {
            1 => Color::RED,
            2 => Color::YELLOW,
            3 => Color::GREEN,
            4 => Color::BLUE,
            _ => Color::MAGENTA,
        };

        if hit.side == 1 {
            color.r = (color.r as f32 * 0.8) as u8;
            color.g = (color.g as f32 * 0.8) as u8;
            color.b = (color.b as f32 * 0.8) as u8;
        }

        hit.pos = Vec2::new(player.pos.x + sidedist.x, player.pos.y + sidedist.y);
        let dperp = if hit.side == 0 {
            sidedist.x - deltadist.x
        } else {
            sidedist.y - deltadist.y
        };

        let h = WINDOW_HEIGHT as f32 / dperp;
        let y0 = i32::max(WINDOW_HEIGHT as i32 / 2 - h as i32 / 2, 0);
        let y1 = i32::min(
            WINDOW_HEIGHT as i32 / 2 + h as i32 / 2,
            WINDOW_HEIGHT as i32 - 1,
        );

        draw_column(canvas, x as i32, 0, y0, Color::RGB(50, 50, 50));
        draw_column(canvas, x as i32, y0, y1, color);
        draw_column(canvas, x as i32, y1, WINDOW_HEIGHT as i32 - 1, Color::CYAN);
    }

    canvas.present();
}

fn draw_column(canvas: &mut WindowCanvas, x: i32, y0: i32, y1: i32, color: Color) {
    for i in y0..y1 {
        canvas.set_draw_color(color);
        let _ = canvas.draw_point(Point::new(x, i));
    }
}
