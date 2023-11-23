use std::{f32::consts::PI, time::Instant};

use input::InputHandler;
use math::vector::Vec3;
use player::Player;

mod input;
mod level;
mod math;
mod player;
mod renderer;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

// Create window
// Initialise entities
// Loop
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("3D Renderer Demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not initialize canvas");

    let mut input_handler = InputHandler::new();

    let mut player = Player::new(Vec3::new(4.5, 0.0, 6.5), PI);

    let mut event_pump = sdl_context.event_pump()?;
    let mut time = Instant::now();
    'running: loop {
        let delta_time = time.elapsed().as_secs_f32();
        time = Instant::now();
        if input_handler.read_inputs(&mut event_pump).is_err() {
            break 'running;
        }
        player.update(delta_time, &input_handler);
        renderer::render(&mut canvas, &player);
    }
    Ok(())
}
