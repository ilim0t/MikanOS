#[derive(Clone, Copy)]
pub struct PixelPoint {
    pub x: usize, // Column, Width
    pub y: usize, // Row, Height
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
