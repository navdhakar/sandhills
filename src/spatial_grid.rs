use std::collections::HashMap;

use crate::particle::Particle;
use crate::vec2::Vec2;

pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialGrid {
    pub fn build(particles: &[Particle], cell_size: f32) -> Self {
        let mut cells: HashMap<(i32, i32), Vec<usize>> = HashMap::new();

        for (index, p) in particles.iter().enumerate() {
            let cell = cell_coords(p.center(), cell_size);
            cells.entry(cell).or_insert_with(Vec::new).push(index);
        }

        Self { cell_size, cells }
    }

    pub fn neighbors_of(&self, pos: Vec2) -> Vec<usize> {
        let (cx, cy) = cell_coords(pos, self.cell_size);
        let mut result = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(indices) = self.cells.get(&(cx + dx, cy + dy)) {
                    result.extend_from_slice(indices);
                }
            }
        }
        result
    }
}

fn cell_coords(pos: Vec2, cell_size: f32) -> (i32, i32) {
    ((pos.x / cell_size).floor() as i32, (pos.y / cell_size).floor() as i32)
}