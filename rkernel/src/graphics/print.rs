use lazy_static::lazy_static;

use super::*;
use core::ptr::null_mut;
use spin::Mutex;

lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = {
        let frame_buffer_config = FrameBufferConfig {
            frame_buffer: null_mut(),
            pixels_per_scan_line: 0,
            horizontal_resolution: 0,
            vertical_resolution: 0,
            pixel_format: PixelFormat::KPixelRGBReserved8BitPerColor,
        };

        let writer = Writer::new(&frame_buffer_config);
        let console = Console::new(
            writer,
            Color { r: 0, g: 20, b: 0 },
            Color {
                r: 255,
                g: 255,
                b: 255,
            },
        );
        Mutex::new(console)
    };
}
