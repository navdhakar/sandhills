use crate::color_utils;
use crate::material::Material;
use crate::particle::Particle;
use crate::vec2::Vec2;

pub struct PourPoint {
    pub x: f32,
    pub y: f32,
    cooldown: u32,
    palette_index: usize
}

impl PourPoint {
    pub fn new(x: f32, y: f32, palette_index: usize) -> Self {
        Self {
            x,
            y,
            cooldown: 0,
            palette_index,
        }
    }
}

const POUR_INTERVAL_FRAMES: u32 = 1;

const POUR_JITTER: f32 = 1.5;

pub struct PourState {
    points: Vec<PourPoint>,
    next_seed: u32,
    palette_index: usize,
}

impl PourState {
    pub fn new(count: usize, width: f32, margin: f32) -> Self {
        let mut points = Vec::new();
        if count <= 1 {
            points.push(PourPoint::new(width / 2.0, 5.0, 0));
        } else {
            let usable = width - margin * 2.0;
            for i in 0..count {
                let t = i as f32 / (count - 1) as f32;
                let palette_index = i % Material::SAND_PALETTE.len();
                points.push(PourPoint::new(margin + usable * t, 5.0, palette_index));
            }
        }
        Self {
            points,
            next_seed: 0,
            palette_index: 0,
        }
    }

    pub fn cycle_palette(&mut self) {
        self.palette_index = (self.palette_index + 1) % Material::SAND_PALETTE.len();
    }
}
pub fn update(
    pour_state: &mut PourState,
    particles: &mut Vec<Particle>,
    enabled: bool,
    particle_size: i32,
    particle_mass: f32,
) {
    if !enabled {
        return;
    }

    for point in pour_state.points.iter_mut() {
        if point.cooldown > 0 {
            point.cooldown -= 1;
            continue;
        }
        point.cooldown = POUR_INTERVAL_FRAMES;

        let jitter = color_utils::pseudo_offset_f32(pour_state.next_seed, POUR_JITTER);
        let spawn_x = point.x + jitter - (particle_size as f32 / 2.0);
        let spawn_y = point.y;

        let base_color = Material::SAND_PALETTE[point.palette_index];
        let shaded_color =
            color_utils::shade(base_color, pour_state.next_seed, Material::SHADE_VARIATION);
        pour_state.next_seed = pour_state.next_seed.wrapping_add(1);

        let material = Material {
            friction: Material::SAND.friction,
            restitution: Material::SAND.restitution,
            color: shaded_color,
        };

        particles.push(Particle::new(
            Vec2::new(spawn_x, spawn_y),
            particle_mass,
            particle_size,
            material,
        ));
    }
}