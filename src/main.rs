mod particle;
mod physics;
mod render;
mod vec2;
mod collisions;

use rand::Rng;
use minifb::{Key, Window, WindowOptions};
use particle::Particle;
use vec2::Vec2;
use collisions::{resolve_collisions};
use std::time::{Instant, Duration};

const WIDTH: usize = 320;
const HEIGHT: usize = 340;
const PARTICLE_SIZE: i32 = 1;
const PARTICLE_MASS: f32 = 0.04;
const COLLISION_ITERATIONS: usize = 4;

fn spawn_particles() -> Vec<Particle> {
    let mut rng = rand::rng();
    let mut particles = Vec::new();
    
    let cols = 50;
    let rows = 10;
    let spacing = (PARTICLE_SIZE + 2) as f32;
    let start_x = 80.0;
    let start_y = 200.0;

    // Define how much random "jitter" or offset you want to add to each particle
    let max_jitter = 5.0; 

    for row in 0..rows {
        for col in 0..cols {
            // 1. Generate random offsets directly inside the loop as f32
            let offset_x = rng.random_range(-max_jitter..=max_jitter);
            let offset_y = rng.random_range(-max_jitter..=max_jitter);

            // 2. Calculate coordinates and add the random offsets
            let x = start_x + (col as f32 * spacing) + offset_x;
            let y = start_y + (row as f32 * spacing) + offset_y;
            
            let pos = Vec2::new(x, y);

            // 3. Add this as a new particle
            particles.push(
                Particle::new(pos, PARTICLE_MASS, PARTICLE_SIZE, 0x00FFA500)
            );
        }
    }
    
    // Return the particles vector
    particles
}


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Rust Pixel Game Loop", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);

    let dt = 1.0 / 60.0;
    let mut particles = spawn_particles();
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Duration::from_secs(0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let delta = now.duration_since(last_time);
        last_time = now;

        frame_count += 1;
        fps_timer += delta;
        if fps_timer >= Duration::from_secs(1) {
            let fps = frame_count;
            window.set_title(&format!("Sandhills | FPS: {}", fps));
            
            // Reset counters for the next second
            frame_count = 0;
            fps_timer = Duration::from_secs(0);
        }
        buffer.fill(0x001E1E1E); 
        
        // took inspiration for this step function from ksim simualtions

        // now we need to simulate physics and render for each particle
        for p in particles.iter_mut(){
        physics::step(p, dt, WIDTH as i32, HEIGHT as i32);
        }
        //get the collision data as well before render, and not let two particles collide


        for _ in 0..COLLISION_ITERATIONS {
            resolve_collisions(&mut particles); // inject current state of all the particles
        }

        for p in particles.iter_mut(){
            render::draw_particle(p, WIDTH, HEIGHT, &mut buffer);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
