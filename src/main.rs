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

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Rust Pixel Game Loop", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);

    let dt = 1.0 / 60.0;
    let mut p = Particle::new(Vec2::new(10.0, 10.0), PARTICLE_MASS, PARTICLE_SIZE, 0x00FFA500);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0x001E1E1E);

        physics::step(&mut p, dt, WIDTH as i32, HEIGHT as i32);
        render::draw_particle(&p, WIDTH, HEIGHT, &mut buffer);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}