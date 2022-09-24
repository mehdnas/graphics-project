
use nalgebra_glm as glm;

pub struct Line {
    pub start: glm::Vec2,
    pub end: glm::Vec2,
}

impl Line {
    pub fn new(start: glm::Vec2, end: glm::Vec2) -> Self {
        Self {
            start,
            end,
        }
    }
}
