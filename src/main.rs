mod collisions;
mod particle;
mod physics;
mod render;
mod spatial_grid;
mod vec2;

use std::time::Instant;

use minifb::{Key, Window, WindowOptions};
use particle::Particle;
use spatial_grid::SpatialGrid;
use vec2::Vec2;

const WIDTH: usize = 420;
const HEIGHT: usize = 440;
const PARTICLE_SIZE: i32 = 4;
const PARTICLE_MASS: f32 = 0.04;
const COLLISION_ITERATIONS: usize = 4;

fn pseudo_jitter(seed: u32, magnitude: f32) -> f32 {
    let hashed = seed.wrapping_mul(2654435761);
    let normalized = (hashed % 1000) as f32 / 1000.0;
    (normalized - 0.5) * 2.0 * magnitude
}
fn clamp_to_bounds(p: &mut Particle, width: i32, height: i32) {
    let max_x = (width - p.size) as f32;
    let max_y = (height - p.size) as f32;

    if p.pos.x < 0.0 {
        p.pos.x = 0.0;
    } else if p.pos.x > max_x {
        p.pos.x = max_x;
    }

    if p.pos.y > max_y {
        p.pos.y = max_y;
    }
}
fn spawn_particles() -> Vec<Particle> {
    let mut particles = Vec::new();
    let cols = 1;
    let rows = 100;
    let spacing = (PARTICLE_SIZE + 2) as f32;
    let start_x = 40.0;
    let start_y = 200.0;

    for row in 0..rows {
        for col in 0..cols {
            let seed = (row * cols + col) as u32;
            let jitter_x = pseudo_jitter(seed, 0.7);
            let jitter_y = pseudo_jitter(seed.wrapping_add(9973), 0.7);

            let pos = Vec2::new(
                start_x + col as f32 * spacing + jitter_x,
                start_y + row as f32 * spacing + jitter_y,
            );
            particles.push(Particle::new(pos, PARTICLE_MASS, PARTICLE_SIZE, 0x00FFA500));
        }
    }
    particles
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Sandhills", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);

    let dt = 1.0 / 60.0;
    let mut particles = spawn_particles();
    let cell_size = PARTICLE_SIZE as f32 * 2.0;

    let mut last_title_update = Instant::now();
    let mut frame_count: u32 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
    buffer.fill(0x001E1E1E);

    for p in particles.iter_mut() {
        physics::step(p, dt, WIDTH as i32, HEIGHT as i32);
    }

    let grid = SpatialGrid::build(&particles, cell_size);

    for _ in 0..COLLISION_ITERATIONS {
        collisions::resolve_collisions(&mut particles, &grid);
    }

    // Collision resolution above can push a particle past a wall via
    // neighbor pressure -- physics::step only checked walls BEFORE that
    // happened. Re-clamp positions (no velocity change needed here,
    // we're just correcting drift, not a fresh bounce) so nothing ends
    // the frame outside the playfield.
    for p in particles.iter_mut() {
        clamp_to_bounds(p, WIDTH as i32, HEIGHT as i32);
    }

    for p in particles.iter() {
        render::draw_particle(p, WIDTH, HEIGHT, &mut buffer);
    }

    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    frame_count += 1;
        let elapsed = last_title_update.elapsed();
        if elapsed.as_secs_f32() >= 1.0 {
            let fps = frame_count as f32 / elapsed.as_secs_f32();
            window.set_title(&format!(
                "Rust Pixel Game Loop - {} particles - {:.0} FPS",
                particles.len(),
                fps
            ));
            frame_count = 0;
            last_title_update = Instant::now();
        }
    }
}