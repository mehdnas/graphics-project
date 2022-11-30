
pub const WINDOW_HEIGHT: u32 = 800;
pub const WINDOW_WIDTH: u32 = 800;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0_f32,
            g: 0.0_f32,
            b: 0.0_f32,
            a: 1.0_f32,
        }
    }
}

pub struct ColorU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for ColorU8 {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Clone for ColorU8 {
    fn clone(&self) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
