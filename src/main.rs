mod particle;
mod physics;
mod render;
mod vec2;

use minifb::{Key, Window, WindowOptions};
use particle::Particle;
use vec2::Vec2;

const WIDTH: usize = 320;
const HEIGHT: usize = 340;
const PARTICLE_SIZE: i32 = 4;
const PARTICLE_MASS: f32 = 0.04;

fn spawn_particles() -> Vec<Particle> {
    let mut particles = Vec::new();
    let cols = 15;
    let rows = 10;
    let spacing = (PARTICLE_SIZE + 2) as f32;
    let start_x = 40.0;
    let start_y = 10.0;

    // now we want to loop through each row and columns to set position of particle.

    for row in 0..rows {
        for col in 0..cols {
            //so what will be the postion ...?, lets use vector to define the pos
            let pos = Vec2::new(start_x + col as f32 * spacing, start_y + row as f32 * spacing);
            //add this as new particle
            particles.push(
                    Particle::new(pos, PARTICLE_MASS, PARTICLE_SIZE, 0x00FFA500)
                )
        }
    }
    // retunr particles
    particles
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Rust Pixel Game Loop", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);

    let dt = 1.0 / 60.0;
    let mut particles = spawn_particles();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0x001E1E1E);
        
        // took inspiration for this step function from ksim simualtions

        // now we need to simulate physics and render for each particle
        for p in particles.iter_mut(){
        physics::step(p, dt, WIDTH as i32, HEIGHT as i32);
        render::draw_particle(p, WIDTH, HEIGHT, &mut buffer);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
// forgot to record, so backtracking the code