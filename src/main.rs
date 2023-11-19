use math::rotation::EulerAngle;
use player::Player;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

    let player = Player::new(
        (4.5, 0.0, 6.5),
        EulerAngle::new(0.0, 0.0, 0.0).to_quaternion(),
    );

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        renderer::render(&mut canvas, &player);
    }
    Ok(())
}
