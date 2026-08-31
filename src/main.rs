use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 340;
const PARTICLE_SIZE: i32 = 4;
const PARTICLE_MASS: f32 = 0.04;
const GRAVITATIONAL_ACC_VER: f32 = 9.8;
const GRAVITATIONAL_ACC_HOR: f32 = 0.0;
//we should go with mass directly, it should be bound with its volumne but for now constant mass is fine.
struct Particle {
    particle_x: i32,
    particle_y: i32,
    particle_mass: f32,
    particle_size: i32,
    vel_x: f32,
    vel_y: f32,
    particle_color: u32
}

impl Particle {

    fn new(particle_x: i32, particle_y: i32, particle_mass: f32, particle_size:i32, vel_x: f32, vel_y: f32, particle_color: u32) -> Self {
        Self {
            particle_x,
            particle_y,
            particle_mass,
            particle_size,
            vel_x,
            vel_y,
            particle_color
        }
    }

}

// it spaws particle as per thier pos and size, basically its a graphics handler, physics will be handled sperately.
// our particles are symmetrical
fn sand_spawn(px: usize, py: usize, psize:usize, pcolor: u32, buffer: &mut[u32]) {
for x in px..px + psize {
            for y in py..py + psize {
            buffer[(y as usize)*WIDTH + (x as usize)] = pcolor; //should be orange color pixel at  px, py
            }
        }
}

fn sys_physx(p: &mut Particle, time: f32, buffer: &mut [u32]) {
    let mut x_t = (((GRAVITATIONAL_ACC_HOR) * (time as f32) * (time as f32))/2.0) + (p.vel_x*(time as f32)) + (p.particle_x as f32);
    let mut y_t = (((GRAVITATIONAL_ACC_VER) * (time as f32) * (time as f32))/2.0) + (p.vel_y*(time as f32)) + (p.particle_y as f32);
    sand_spawn((x_t as usize), (y_t as usize), (p.particle_size as usize), p.particle_color, buffer);

}


fn main() {
    // 1. Create a 1D pixel buffer (0x00RRGGBB)
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // 2. Open a window
    let mut window = Window::new(
        "Rust Pixel Game Loop",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // Cap frame rate at ~60 FPS
    window.set_target_fps(60);
    // Moving pixel state
    let mut time: f32 = 0.0;
    let mut frame_counter: usize = 0;
    let mut px: i32 = 10;
    let mut py: i32 = 10;
    let mut dx: i32 = 0;
    let mut dy: i32 = 0; // these are the delta position change of a particle.

    let mut vx = 0.0;
    let mut vy = 0.0;

    let mut p = Particle::new(px, py, PARTICLE_MASS, PARTICLE_SIZE, vx, vy, 0x00FFA500);
 
    
    // 3. Game Loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear screen to dark gray (0x001E1E1E)
        buffer.fill(0x001E1E1E);     
        time = (frame_counter as f32) / 60.0; // this does not seem like a good approach, this number will grow large and large, for now shold be fine
        sys_physx(&mut p, time, &mut buffer);   
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        frame_counter += 1;
    }
}

