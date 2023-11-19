use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode, EventPump};

struct KeyInput {
    keycode: Keycode,
    is_pressed: bool,
}

pub struct InputHandler {
    key_inputs: HashMap<String, KeyInput>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_inputs: HashMap::from([
                (
                    "forwards".to_string(),
                    KeyInput {
                        keycode: Keycode::W,
                        is_pressed: false,
                    },
                ),
                (
                    "forwards".to_string(),
                    KeyInput {
                        keycode: Keycode::S,
                        is_pressed: false,
                    },
                ),
                (
                    "forwards".to_string(),
                    KeyInput {
                        keycode: Keycode::A,
                        is_pressed: false,
                    },
                ),
                (
                    "forwards".to_string(),
                    KeyInput {
                        keycode: Keycode::D,
                        is_pressed: false,
                    },
                ),
            ]),
        }
    }

    pub fn read_inputs(&mut self, event_pump: &mut EventPump) -> Result<(), String> {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Err("Quit pressed".to_string()),

                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    for (_, input) in self.key_inputs.iter_mut() {
                        if keycode == input.keycode {
                            input.is_pressed = true;
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    for (_, input) in self.key_inputs.iter_mut() {
                        if keycode == input.keycode {
                            input.is_pressed = false;
                        }
                    }
                }

                _ => {}
            };
        }
        Ok(())
    }
}
