use crate::particle::Particle;
use crate::vec2::Vec2;

const RESTITUION: f32 = 0.3;

pub const GRAVITY: Vec2 = Vec2 { x: 0.0, y: 9.8 };

pub fn step(p: &mut Particle, dt: f32, width: i32, height: i32) {
    apply_gravity(p, dt);
    integrate(p, dt);
    resolve_floor_collision(p, height);
    resolve_wall_collision(p, width);
}

fn apply_gravity(p: &mut Particle, dt: f32) {
    p.vel += GRAVITY * dt; // => v = u + at
}

fn integrate(p: &mut Particle, dt: f32) {
    p.pos += p.vel * dt;
}

fn resolve_floor_collision(p: &mut Particle, height: i32) {
    let max_y = (height - p.size) as f32;
    if p.pos.y > max_y {
        p.pos.y = max_y;
        p.vel.y = -RESTITUION * p.vel.y;
    }
}
fn resolve_wall_collision(p: &mut Particle, width: i32) {
    // pos is the top-left corner, so the valid range for it is
    // [0, width - size] -- not [size, width - size] like before.
    let max_x = (width - p.size) as f32;
    let min_x = 0.0;

    if p.pos.x > max_x {
        p.pos.x = max_x;
        p.vel.x = -RESTITUION * p.vel.x;
    } else if p.pos.x < min_x {
        // Previously this branch also snapped to max_x, teleporting a
        // particle that hit the LEFT wall all the way to the RIGHT wall.
        p.pos.x = min_x;
        p.vel.x = -RESTITUION * p.vel.x;
    }
}