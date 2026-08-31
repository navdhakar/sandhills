use crate::material::Material;
use crate::vec2::Vec2;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub size: i32,
    pub material: Material,
}

impl Particle {
    pub fn new(pos: Vec2, mass: f32, size: i32, material: Material) -> Self {
        Self {
            pos,
            vel: Vec2::zero(),
            mass,
            size,
            material,
        }
    }

    pub fn radius(&self) -> f32 {
        self.size as f32 / 2.0
    }

    pub fn center(&self) -> Vec2 {
        self.pos + Vec2::new(self.radius(), self.radius())
    }
}