
pub fn shade(base_color: u32, seed: u32, variation: u8) -> u32 {
    let r = ((base_color >> 16) & 0xFF) as i32;
    let g = ((base_color >> 8) & 0xFF) as i32;
    let b = (base_color & 0xFF) as i32;
    let r_offset = pseudo_offset(seed, variation);
    let g_offset = pseudo_offset(seed.wrapping_add(7919), variation);
    let b_offset = pseudo_offset(seed.wrapping_add(104729), variation);

    let r = (r + r_offset).clamp(0, 255);
    let g = (g + g_offset).clamp(0, 255);
    let b = (b + b_offset).clamp(0, 255);

    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn pseudo_offset(seed: u32, variation: u8) -> i32 {
    let hashed = seed.wrapping_mul(2654435761);
    let normalized = (hashed % 1000) as i32; // 0..1000
    let range = variation as i32 * 2;
    (normalized % range) - variation as i32
}
pub fn pseudo_offset_f32(seed: u32, magnitude: f32) -> f32 {
    let hashed = seed.wrapping_mul(2654435761);
    let normalized = (hashed % 1000) as f32 / 1000.0; // 0.0..1.0
    (normalized - 0.5) * 2.0 * magnitude
}