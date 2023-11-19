use std::f32::consts::PI;

use sdl2::sys::SDL_atan2f;

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    pub fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn rotate(&self, q: Quaternion) -> Self {
        let qc = q.conjugate();
        Self {
            w: q.w * self.w * qc.w,
            x: q.x * self.x * qc.x,
            y: q.y * self.y * qc.y,
            z: q.z * self.z * qc.z,
        }
    }

    // There may be singularity problems here
    pub fn to_euler(&self) -> EulerAngle {
        EulerAngle {
            x: unsafe {
                SDL_atan2f(
                    2.0 * (self.w * self.x + self.y * self.z),
                    1.0 - 2.0 * (self.x * self.x + self.y * self.y),
                )
            },
            y: unsafe {
                2.0 * SDL_atan2f(
                    (1.0 + 2.0 * (self.w * self.y - self.x * self.z)).sqrt(),
                    (1.0 - 2.0 * (self.w * self.y - self.x * self.z)).sqrt(),
                ) - PI / 2.0
            },
            z: unsafe {
                SDL_atan2f(
                    2.0 * (self.w * self.z + self.x * self.y),
                    1.0 - 2.0 * (self.y * self.y + self.z * self.z),
                )
            },
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
    pub fn new(x: f32, y: f32, z: f32) -> EulerAngle {
        EulerAngle { x, y, z }
    }

    pub fn to_quaternion(&self) -> Quaternion {
        let cx = (self.x * 0.5).cos();
        let sx = (self.x * 0.5).sin();
        let cy = (self.y * 0.5).cos();
        let sy = (self.y * 0.5).sin();
        let cz = (self.z * 0.5).cos();
        let sz = (self.z * 0.5).sin();
        Quaternion {
            w: cx * cy * cz + sx * sy * sz,
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz + sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
        }
    }
}
