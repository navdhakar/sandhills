use crate::particle::Particle;
use crate::vec2::Vec2;

pub const GRAVITY: Vec2 = Vec2 { x: 0.0, y: 9.8 };

pub fn step(p: &mut Particle, dt: f32, width: i32, height: i32) {
    apply_gravity(p, dt);
    integrate(p, dt);
    resolve_floor_collision(p, height);
    let _ = width; // will matter once we add left/right wall collisions
}

fn apply_gravity(p: &mut Particle, dt: f32) {
    p.vel += GRAVITY * dt;
}

fn integrate(p: &mut Particle, dt: f32) {
    p.pos += p.vel * dt;
}

fn resolve_floor_collision(p: &mut Particle, height: i32) {
    let max_y = (height - p.size) as f32;
    if p.pos.y > max_y {
        p.pos.y = max_y;
        p.vel.y = 0.0;
    }
}