use super::*;
pub struct Writer<'global> {
    frame_buffer: &'global mut [u8],
    pixel_format: PixelFormat,
    pixels_per_scan_line: usize,
    horizontal_resolution: usize,
    vertical_resolution: usize,
}

impl Writer<'_> {
    pub fn new(config: &FrameBufferConfig) -> Writer {
        let &FrameBufferConfig {
            pixel_format,
            pixels_per_scan_line,
            vertical_resolution,
            horizontal_resolution,
            ..
        } = config;
        Writer {
            frame_buffer: config.get_slice(),
            pixel_format,
            pixels_per_scan_line: pixels_per_scan_line as usize,
            horizontal_resolution: horizontal_resolution as usize,
            vertical_resolution: vertical_resolution as usize,
        }
    }

    pub fn write(&mut self, point: &Point, &Color { r, g, b }: &Color) {
        let pixel = self.at_mut(point);
        *pixel.r_mut() = r;
        *pixel.g_mut() = g;
        *pixel.b_mut() = b;
    }

    pub fn write_all(&mut self, color: &Color) {
        for x in 0..self.pixels_per_scan_line {
            for y in 0..self.vertical_resolution {
                self.write(&Point { x, y }, color);
            }
        }
    }

    fn get_offset(&self, Point { x, y }: &Point) -> usize {
        assert!(*x < self.horizontal_resolution);
        assert!(*y < self.vertical_resolution);
        4 * (self.pixels_per_scan_line * y + x)
    }

    pub fn at(&self, point: &Point) -> &dyn RefColor {
        let ptr = &self.frame_buffer[self.get_offset(point)] as *const u8;
        let ref_color = self.pixel_format.convert_into_ref_color(ptr);
        unsafe { &*ref_color }
    }

    fn at_mut(&mut self, point: &Point) -> &mut dyn RefColor {
        let ptr = &mut self.frame_buffer[self.get_offset(point)] as *mut u8;
        let ref_color = self.pixel_format.convert_into_ref_color_mut(ptr);
        unsafe { &mut *ref_color }
    }
}
