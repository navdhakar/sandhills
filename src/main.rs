mod collisions;
mod color_utils;
mod material;
mod particle;
mod physics;
mod pour;
mod render;
mod spatial_grid;
mod vec2;

use std::time::Instant;

use material::Material;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use particle::Particle;
use pour::PourState;
use spatial_grid::SpatialGrid;
use vec2::Vec2;

const WIDTH: usize = 820;
const HEIGHT: usize = 340;
const BASE_PARTICLE_SIZE: i32 = 4;
const BASE_PARTICLE_MASS: f32 = 0.04;
const MAX_PARTICLE_SIZE: i32 = 8; 
const MIN_PARTICLE_SIZE: i32 = 2;
const COLLISION_ITERATIONS: usize = 4;

const PUSH_RADIUS: f32 = 40.0;
const PUSH_STRENGTH: f32 = 4000.0;
const SPAWN_COOLDOWN_FRAMES: u32 = 2;

const POUR_POINT_COUNT: usize = 3;
const POUR_MARGIN: f32 = 30.0;

fn pseudo_jitter(seed: u32, magnitude: f32) -> f32 {
    let hashed = seed.wrapping_mul(2654435761);
    let normalized = (hashed % 1000) as f32 / 1000.0;
    (normalized - 0.5) * 2.0 * magnitude
}
struct SpawnState {
    next_seed: u32,
    palette_index: usize,
}

fn spawn_particles(spawn_state: &mut SpawnState) -> Vec<Particle> {
    let mut particles = Vec::new();
    let cols = 1;
    let rows = 500;
    let spacing = (BASE_PARTICLE_SIZE + 2) as f32;
    let start_x = 400.0;
    let start_y = 100.0;

    for row in 0..rows {
        for col in 0..cols {
            let pos_seed = (row * cols + col) as u32;
            let jitter_x = pseudo_jitter(pos_seed, 0.7);
            let jitter_y = pseudo_jitter(pos_seed.wrapping_add(9973), 0.7);

            let pos = Vec2::new(
                start_x + col as f32 * spacing + jitter_x,
                start_y + row as f32 * spacing + jitter_y,
            );

            let material = shaded_sand(spawn_state);
            particles.push(Particle::new(pos, BASE_PARTICLE_MASS, BASE_PARTICLE_SIZE, material));
        }
    }
    particles
}
fn shaded_sand(spawn_state: &mut SpawnState) -> Material {
    let base_color = Material::SAND_PALETTE[spawn_state.palette_index];
    let shaded_color = color_utils::shade(base_color, spawn_state.next_seed, Material::SHADE_VARIATION);
    spawn_state.next_seed = spawn_state.next_seed.wrapping_add(1);

    Material {
        friction: Material::SAND.friction,
        restitution: Material::SAND.restitution,
        color: shaded_color,
    }
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

fn apply_push_force(particles: &mut [Particle], cursor: Vec2, dt: f32) {
    for p in particles.iter_mut() {
        let diff = p.center() - cursor;
        let dist = diff.length();
        if dist < PUSH_RADIUS && dist > 0.001 {
            let falloff = 1.0 - (dist / PUSH_RADIUS);
            let dir = diff * (1.0 / dist);
            p.vel += dir * (PUSH_STRENGTH * falloff * dt);
        }
    }
}

fn mass_for_size(size: i32) -> f32 {
    let scale = size as f32 / BASE_PARTICLE_SIZE as f32;
    BASE_PARTICLE_MASS * scale * scale
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Rust Pixel Game Loop", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);

    let dt = 1.0 / 60.0;
    let mut spawn_state = SpawnState { next_seed: 0, palette_index: 0 };

    let mut particles: Vec<Particle> = Vec::new();

    let mut pour_state = PourState::new(POUR_POINT_COUNT, WIDTH as f32, POUR_MARGIN);
    let mut pouring_enabled = true; // toggle with P

    let cell_size = MAX_PARTICLE_SIZE as f32 * 2.0;

    let mut current_size = BASE_PARTICLE_SIZE;
    let mut spawn_cooldown: u32 = 0;
    let mut size_key_was_down = false;
    let mut color_key_was_down = false;
    let mut pour_toggle_key_was_down = false;

    let mut last_title_update = Instant::now();
    let mut frame_count: u32 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0x001E1E1E);

        let size_key_down = window.is_key_down(Key::Up) || window.is_key_down(Key::Down);
        if size_key_down && !size_key_was_down {
            if window.is_key_down(Key::Up) {
                current_size = (current_size + 1).min(MAX_PARTICLE_SIZE);
            } else {
                current_size = (current_size - 1).max(MIN_PARTICLE_SIZE);
            }
        }
        size_key_was_down = size_key_down;

        let color_key_down = window.is_key_down(Key::C);
        if color_key_down && !color_key_was_down {
            spawn_state.palette_index =
                (spawn_state.palette_index + 1) % Material::SAND_PALETTE.len();
        }
        color_key_was_down = color_key_down;

        let pour_toggle_key_down = window.is_key_down(Key::P);
        if pour_toggle_key_down && !pour_toggle_key_was_down {
            pouring_enabled = !pouring_enabled;
        }
        pour_toggle_key_was_down = pour_toggle_key_down;

        pour::update(
            &mut pour_state,
            &mut particles,
            pouring_enabled,
            current_size,
            mass_for_size(current_size),
        );

        if window.get_mouse_down(MouseButton::Left) {
            if spawn_cooldown == 0 {
                if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) {
                    let mass = mass_for_size(current_size);
                    let spawn_pos = Vec2::new(mx - current_size as f32 / 2.0, my - current_size as f32 / 2.0);
                    let material = shaded_sand(&mut spawn_state);
                    particles.push(Particle::new(spawn_pos, mass, current_size, material));
                    spawn_cooldown = SPAWN_COOLDOWN_FRAMES;
                }
            } else {
                spawn_cooldown -= 1;
            }
        } else {
            spawn_cooldown = 0;
        }

        if window.get_mouse_down(MouseButton::Right) {
            if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) {
                apply_push_force(&mut particles, Vec2::new(mx, my), dt);
            }
        }

        for p in particles.iter_mut() {
            physics::step(p, dt, WIDTH as i32, HEIGHT as i32);
        }

        let grid = SpatialGrid::build(&particles, cell_size);

        for _ in 0..COLLISION_ITERATIONS {
            collisions::resolve_collisions(&mut particles, &grid);
        }

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
                "Rust Pixel Game Loop - {} particles - {:.0} FPS - spawn size {} - pouring: {}",
                particles.len(),
                fps,
                current_size,
                if pouring_enabled { "on" } else { "off" }
            ));
            frame_count = 0;
            last_title_update = Instant::now();
        }
    }
}