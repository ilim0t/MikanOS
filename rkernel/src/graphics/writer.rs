use super::*;
use core::slice;

#[repr(C)]
pub struct Buffer(u8, u8, u8, u8);

pub struct FrameSize {
    pub width: usize,
    pub height: usize,
}

pub struct PixelWriter {
    frame_buffer: &'static mut [Buffer],
    pixel_format: PixelFormat,
    pub frame_size: FrameSize,
    pixels_per_scan_line: usize,
}

impl PixelWriter {
    pub fn new(config: &FrameBufferConfig) -> PixelWriter {
        let frame_size = FrameSize {
            width: config.horizontal_resolution as usize,
            height: config.vertical_resolution as usize,
        };
        let pixels_per_scan_line = config.pixels_per_scan_line as usize;
        let frame_buffer = unsafe {
            slice::from_raw_parts_mut(
                config.frame_buffer as *mut Buffer,
                frame_size.height * pixels_per_scan_line,
            )
        };

        PixelWriter {
            frame_buffer,
            pixel_format: config.pixel_format,
            frame_size,

            pixels_per_scan_line,
        }
    }

    fn get_slice_index(&self, PixelPoint { x, y }: &PixelPoint) -> usize {
        assert!(*x < self.frame_size.width);
        assert!(*y < self.frame_size.height);
        self.pixels_per_scan_line * y + x
    }

    pub fn at(&self, point: &PixelPoint) -> &Buffer {
        &self.frame_buffer[self.get_slice_index(point)]
    }

    fn at_mut(&mut self, point: &PixelPoint) -> &mut Buffer {
        &mut self.frame_buffer[self.get_slice_index(point)]
    }

    pub fn write_pixel(&mut self, point: &PixelPoint, &Color { r, g, b }: &Color) {
        match self.pixel_format {
            PixelFormat::KPixelRGBReserved8BitPerColor => {
                let pixel_buffer = self.at_mut(point);
                pixel_buffer.0 = b;
                pixel_buffer.1 = g;
                pixel_buffer.2 = r;
            }
            PixelFormat::KPixelBGRReserved8BitPerColor => {
                let pixel_buffer = self.at_mut(point);
                pixel_buffer.0 = r;
                pixel_buffer.1 = g;
                pixel_buffer.2 = b;
            }
        };
    }

    pub fn clear(&mut self, color: &Color) {
        for x in 0..self.pixels_per_scan_line {
            for y in 0..self.frame_size.height {
                self.write_pixel(&PixelPoint { x, y }, color);
            }
        }
    }

    pub fn write_bytes(&mut self, &PixelPoint { x, y }: &PixelPoint, bytes: &[u8], color: &Color) {
        for (i, &c) in bytes.iter().enumerate() {
            self.write_byte(&PixelPoint { x: x + i * 8, y }, c, color);
        }
    }

    pub fn write_byte(&mut self, PixelPoint { x, y }: &PixelPoint, byte: u8, color: &Color) {
        let font = crate::font::get_font_data(byte);

        for (dy, line) in font.iter().enumerate() {
            for dx in 0..8 {
                let mask = 1 << (7 - dx);
                if (line & mask) != 0 {
                    self.write_pixel(
                        &PixelPoint {
                            x: x + dx,
                            y: y + dy,
                        },
                        color,
                    );
                }
            }
        }
    }
}
