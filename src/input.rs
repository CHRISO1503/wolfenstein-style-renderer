use sdl2::{event::Event, keyboard::Keycode, EventPump};

struct KeyInput {
    id: String,
    is_pressed: bool,
}

pub struct InputHandler {
    key_inputs: [KeyInput; 4],
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_inputs: [
                KeyInput {
                    id: "forwards".to_string(),
                    is_pressed: false,
                },
                KeyInput {
                    id: "backwards".to_string(),
                    is_pressed: false,
                },
                KeyInput {
                    id: "left".to_string(),
                    is_pressed: false,
                },
                KeyInput {
                    id: "right".to_string(),
                    is_pressed: false,
                },
            ],
        }
    }

    pub fn read_inputs(&mut self, event_pump: &mut EventPump) -> Result<(), String> {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Err("Quit pressed".to_string()),

                //TODO: Figure out generic pattern matching here with KeyInput (could not get it to
                //work)
                //                Event::KeyDown {
                //                  keycode: Some(Keycode::W),
                //                repeat: false,
                //              ..
                //        } => self.key_inputs[0].is_pressed = true,
                //      Event::KeyDown {
                //        keycode: Some(Keycode::S),
                //      repeat: false,
                //    ..
                //                } => self.key_inputs[1].is_pressed = true,
                //              Event::KeyDown {
                //                keycode: Some(Keycode::A),
                //              repeat: false,
                //            ..
                //      } => self.key_inputs[2].is_pressed = true,
                //    Event::KeyDown {
                //                    keycode: Some(Keycode::D),
                //                  repeat: false,
                //                ..
                //          } => self.key_inputs[3].is_pressed = true,
                //
                //              Event::KeyUp {
                //                keycode: Some(Keycode::W),
                //              repeat: false,
                //            ..
                //      } => self.key_inputs[0].is_pressed = false,
                //    Event::KeyUp {
                //                    keycode: Some(Keycode::S),
                //                  repeat: false,
                //                ..
                //          } => self.key_inputs[1].is_pressed = false,
                //        Event::KeyUp {
                //          keycode: Some(Keycode::A),
                //        repeat: false,
                //      ..
                //                } => self.key_inputs[2].is_pressed = false,
                //              Event::KeyUp {
                //                keycode: Some(Keycode::D),
                //              repeat: false,
                //            ..
                //      } => self.key_inputs[3].is_pressed = false,
                _ => {}
            };
        }
        Ok(())
    }
}
