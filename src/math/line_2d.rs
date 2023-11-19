// Line in the form of y = mx + c
#[derive(Debug)]
pub struct Line2D {
    m: f32,
    c: f32,
}

impl Line2D {
    pub fn new(m: f32, c: f32) -> Self {
        Self { m, c }
    }

    pub fn y_from_x(&self, x: f32) -> f32 {
        self.m * x + self.c
    }

    pub fn x_from_y(&self, y: f32) -> f32 {
        if self.m < 1e-20 {
            1e30
        } else {
            (y - self.c) / self.m
        }
    }
}
