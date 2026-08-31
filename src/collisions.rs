use crate::particle::Particle;
use crate::vec2::Vec2;
use std::any::type_name_of_val;

const RESTITUION: f32 = 0.05; // restitution will decide how bouncy, if its 0 then system is inelastic, if 1 then perfectly  inelastic

pub fn resolve_collision(particles: &mut [Particle]) {
    // we will iterate all other particale for any particaluar particle at a time to see if there is collision.

    let len = particles.len();
    for i in 0..len {
        for j in (i+1)..len {
            let (left, right) = particles.split_at_mut(j);
            // resolve the collision in these two particles
            resolve_pair(&mut left[i], &mut right[0]); // [0,1,2,3...] => when particle is 1 then split at 2 => [0,1,| 2,3,4..], right is right[0]

        }
    }
}
// basically now we need to check if particle b is in colliding radius of particle a by calculating 2d distance

fn resolve_pair(a: &mut Particle, b: &mut Particle){
    // find out radius of particle
    let radius_a = a.size as f32 / 2.0;
    let radius_b = b.size as f32 / 2.0;

    // find out center of both particle, pos (x, y), pos is at top left corner, so center at (x + rad_a, y_rad+b)
    let center_a = a.pos + Vec2::new(radius_a, radius_a);
    let center_b = b.pos + Vec2::new(radius_b, radius_b);

    // lets calculate distance between two centers  as per the formula => sqrt(diff_x + diff_y)

    let diff = center_b - center_a;
    let dist = (diff.x*diff.x + diff.y*diff.y).sqrt(); // diff.x is (b(x)-a(x)), same for y
 
    let min_dist = radius_a + radius_b;

    if dist >= min_dist {
        return;
    }

    let overlap = min_dist - dist;

    let normal = if dist > 0.0001 {

        Vec2::new(diff.x / dist, diff.y / dist)
    }
    else {
        Vec2::new(0.0, -1.0) // of particle is at top move up the above particle
    };

    let inv_mass_a = 1.0/a.mass;
    let inv_mass_b = 1.0/b.mass;

    let relative_vel = b.vel - a.vel;
    //we need direction velocity, basically velocity along norma =>   vel_x.normal_x..
    let vel_along_normal = relative_vel.x*normal.x + relative_vel.x*normal.x;

    if vel_along_normal < 0.0 {
        let j = -(1.0+RESTITUION)*vel_along_normal/(inv_mass_a + inv_mass_b);
        // impulse along normal
        let impulse = normal * j;

        // velocity update
        a.vel = a.vel - impulse * inv_mass_a;
        b.vel = b.vel + impulse * inv_mass_b;
    }

    let overlap = min_dist - dist;
    let total_inv_mass = inv_mass_a + inv_mass_b;
    let correction_a = overlap * (inv_mass_a / total_inv_mass);
    let correction_b = overlap * (inv_mass_b / total_inv_mass);

    a.pos = a.pos - normal * correction_a;
    b.pos = b.pos + normal * correction_b;
}