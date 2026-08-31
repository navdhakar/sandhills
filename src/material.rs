
#[derive(Clone, Copy)]


pub struct Material {
    pub friction: f32,
    pub restitution: f32,
    pub color: u32,
}

impl Material {
    pub const SAND: Material = Material {
        friction: 0.45,
        restitution: 0.05,
        color: 0x00FFA500,
    };
    pub const SAND_PALETTE: [u32; 4] = [
    0x00FFA500, // orange (original)
    0x00D2B48C, // tan
    0x00C2B280, // desert sand
    0x00E0C094, // pale sand
    ];
pub const SHADE_VARIATION: u8 = 18;
}