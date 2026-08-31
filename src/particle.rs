use crate::vec2::Vec2;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub size: i32,
    pub color: u32,
}

impl Particle {
    pub fn new(pos: Vec2, mass: f32, size: i32, color: u32) -> Self {
        Self {
            pos,
            vel: Vec2::zero(),
            mass,
            size,
            color,
        }
    }
}