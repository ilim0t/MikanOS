use core::slice;

use super::*;

pub struct Writer<'global> {
    frame_buffer: &'global mut [u8],
    pixel_format: PixelFormat,
    pixels_per_scan_line: usize,
    pub horizontal_resolution: usize,
    pub vertical_resolution: usize,
}

impl Writer<'_> {
    pub fn new(config: &FrameBufferConfig) -> Writer {
        let frame_buffer = unsafe {
            slice::from_raw_parts_mut(
                config.frame_buffer,
                (config.vertical_resolution * config.pixels_per_scan_line * 4) as usize,
            )
        };

        Writer {
            frame_buffer,
            pixel_format: config.pixel_format,
            pixels_per_scan_line: config.pixels_per_scan_line as usize,
            horizontal_resolution: config.horizontal_resolution as usize,
            vertical_resolution: config.vertical_resolution as usize,
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

    fn get_slice_index(&self, Point { x, y }: &Point) -> usize {
        assert!(*x < self.horizontal_resolution);
        assert!(*y < self.vertical_resolution);
        4 * (self.pixels_per_scan_line * y + x)
    }

    pub fn at(&self, point: &Point) -> &dyn RefColor {
        let ptr = &self.frame_buffer[self.get_slice_index(point)] as *const u8;
        let ref_color = self.pixel_format.convert_into_ref_color(ptr);
        unsafe { &*ref_color }
    }

    fn at_mut(&mut self, point: &Point) -> &mut dyn RefColor {
        let ptr = &mut self.frame_buffer[self.get_slice_index(point)] as *mut u8;
        let ref_color = self.pixel_format.convert_into_ref_color_mut(ptr);
        unsafe { &mut *ref_color }
    }
}
