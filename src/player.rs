use crate::{input::InputHandler, math::vector::Vec3};

pub struct Player {
    pub pos: Vec3,
    pub rotation: f32,
    pub velocity: Vec3,
}

impl Player {
    pub fn new(pos: Vec3, rotation: f32) -> Self {
        Self {
            pos,
            rotation,
            velocity: Vec3::zero(),
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &InputHandler) {
        let mut local_velocity = Vec3::zero();

        if input.key_inputs["forwards"].is_pressed {
            local_velocity.z += 1.0;
        }
        if input.key_inputs["backwards"].is_pressed {
            local_velocity.z -= 1.0;
        }
        if input.key_inputs["left"].is_pressed {
            local_velocity.x -= 1.0;
        }
        if input.key_inputs["right"].is_pressed {
            local_velocity.x += 1.0;
        }
        if input.key_inputs["turn_left"].is_pressed {
            self.rotation -= 1.0 * delta_time;
        }
        if input.key_inputs["turn_right"].is_pressed {
            self.rotation += 1.0 * delta_time;
        }

        self.velocity = local_velocity.rotate_y(-self.rotation);

        self.pos.x += self.velocity.x * delta_time;
        self.pos.z += self.velocity.z * delta_time;
    }
}
