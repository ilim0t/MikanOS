pub struct Point {
    pub x: usize, // Column
    pub y: usize, // Row
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
#[repr(C)]
pub struct PixelRGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    __: u8,
}

#[allow(dead_code)]
#[repr(C)]
pub struct PixelBGRColor {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    __: u8,
}
pub trait RefColor {
    fn r(&self) -> &u8;
    fn g(&self) -> &u8;
    fn b(&self) -> &u8;

    fn r_mut(&mut self) -> &mut u8;
    fn g_mut(&mut self) -> &mut u8;
    fn b_mut(&mut self) -> &mut u8;
}

impl RefColor for PixelRGBColor {
    fn r(&self) -> &u8 {
        &self.r
    }
    fn g(&self) -> &u8 {
        &self.g
    }
    fn b(&self) -> &u8 {
        &self.b
    }

    fn r_mut(&mut self) -> &mut u8 {
        &mut self.r
    }
    fn g_mut(&mut self) -> &mut u8 {
        &mut self.g
    }
    fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }
}

impl RefColor for PixelBGRColor {
    fn r(&self) -> &u8 {
        &self.r
    }
    fn g(&self) -> &u8 {
        &self.g
    }
    fn b(&self) -> &u8 {
        &self.b
    }

    fn r_mut(&mut self) -> &mut u8 {
        &mut self.r
    }
    fn g_mut(&mut self) -> &mut u8 {
        &mut self.g
    }
    fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }
}
