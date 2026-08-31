use crate::particle::Particle;
use crate::vec2::Vec2;

const RESTITUTION: f32 = 0.05;

const FRICTION: f32 = 0.45;

pub fn resolve_collisions(particles: &mut [Particle]) {
    let len = particles.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let (left, right) = particles.split_at_mut(j);
            resolve_pair(&mut left[i], &mut right[0]);
        }
    }
}

fn resolve_pair(a: &mut Particle, b: &mut Particle) {
    let radius_a = a.size as f32 / 2.0;
    let radius_b = b.size as f32 / 2.0;

    let center_a = a.pos + Vec2::new(radius_a, radius_a);
    let center_b = b.pos + Vec2::new(radius_b, radius_b);

    let diff = center_b - center_a;
    let dist = diff.length();
    let min_dist = radius_a + radius_b;

    if dist >= min_dist {
        return;
    }

    let normal = if dist > 0.0001 {
        diff * (1.0 / dist)
    } else {
        Vec2::new(0.0, -1.0)
    };

    let inv_mass_a = 1.0 / a.mass;
    let inv_mass_b = 1.0 / b.mass;
    let total_inv_mass = inv_mass_a + inv_mass_b;

    let relative_vel = b.vel - a.vel;
    let vel_along_normal = relative_vel.dot(normal);

    let mut j = 0.0;

    if vel_along_normal < 0.0 {
        j = -(1.0 + RESTITUTION) * vel_along_normal / total_inv_mass;
        let impulse = normal * j;
        a.vel = a.vel - impulse * inv_mass_a;
        b.vel = b.vel + impulse * inv_mass_b;
    }

    let relative_vel = b.vel - a.vel;

    let vel_along_normal = relative_vel.dot(normal);
    let tangent_velocity = relative_vel - normal * vel_along_normal;
    let tangent = tangent_velocity.normalized();

    if tangent.length() > 0.0001 && j > 0.0 {
        let vel_along_tangent = relative_vel.dot(tangent);

        let jt = -vel_along_tangent / total_inv_mass;

        let max_friction = FRICTION * j;
        let jt_clamped = jt.clamp(-max_friction, max_friction);

        let friction_impulse = tangent * jt_clamped;
        a.vel = a.vel - friction_impulse * inv_mass_a;
        b.vel = b.vel + friction_impulse * inv_mass_b;
    }

    let overlap = min_dist - dist;
    let correction_a = overlap * (inv_mass_a / total_inv_mass);
    let correction_b = overlap * (inv_mass_b / total_inv_mass);

    a.pos = a.pos - normal * correction_a;
    b.pos = b.pos + normal * correction_b;
}