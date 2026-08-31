use crate::particle::Particle;

pub fn draw_particle(p: &Particle, width: usize, height: usize, buffer: &mut [u32]) {
    let (px, py) = (p.pos.x as i32, p.pos.y as i32);
    if px < 0 || py < 0 {
        return;
    }
    let (px, py) = (px as usize, py as usize);
    for x in px..px + p.size as usize {
        for y in py..py + p.size as usize {
            if x < width && y < height {
                buffer[y * width + x] = p.color;
            }
        }
    }
}