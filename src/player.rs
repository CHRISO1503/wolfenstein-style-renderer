use crate::{input::InputHandler, math::vector::Vec2};

pub struct Player {
    pub pos: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
}

impl Player {
    pub fn new(pos: Vec2, rotation: f32) -> Self {
        Self {
            pos,
            rotation,
            velocity: Vec2::zero(),
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &InputHandler) {
        let mut local_velocity = Vec2::zero();

        if input.key_inputs["forwards"].is_pressed {
            local_velocity.y += 1.0;
        }
        if input.key_inputs["backwards"].is_pressed {
            local_velocity.y -= 1.0;
        }
        if input.key_inputs["left"].is_pressed {
            local_velocity.x += 1.0;
        }
        if input.key_inputs["right"].is_pressed {
            local_velocity.x -= 1.0;
        }
        if input.key_inputs["turn_left"].is_pressed {
            self.rotation -= 1.0 * delta_time;
        }
        if input.key_inputs["turn_right"].is_pressed {
            self.rotation += 1.0 * delta_time;
        }

        self.velocity = local_velocity.rotate(self.rotation);

        self.pos.x += self.velocity.x * delta_time;
        self.pos.y += self.velocity.y * delta_time;
    }
}
