use crate::math::rotation::Quaternion;

pub struct Player {
    pub pos: (f32, f32, f32),
    pub rotation: Quaternion,
}

impl Player {
    pub fn new(pos: (f32, f32, f32), rotation: Quaternion) -> Self {
        Self { pos, rotation }
    }

    pub fn update() {}
}
