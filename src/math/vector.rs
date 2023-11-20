#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        Self {
            x: angle.cos() * self.x - angle.sin() * self.z,
            y: self.y,
            z: angle.sin() * self.x + angle.cos() * self.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EulerAngle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl EulerAngle {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
