#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // 推測
           // #[repr(u32)]  // 明示的指定
pub enum PixelFormat {
    KPixelRGBReserved8BitPerColor,
    KPixelBGRReserved8BitPerColor,
}

#[repr(C)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub pixels_per_scan_line: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
}

#[no_mangle]
pub extern "C" fn _start(frame_buffer_config: &mut FrameBufferConfig) {
    for x in 0..frame_buffer_config.horizontal_resolution {
        for y in 0..frame_buffer_config.vertical_resolution {
            let pixel = (frame_buffer_config.frame_buffer as u32
                + 4 * (frame_buffer_config.pixels_per_scan_line * y + x))
                as *mut u8;
            unsafe {
                *pixel.offset(0) = 55;
                *pixel.offset(1) = 155;
                *pixel.offset(2) = 255;
            }
        }
    }
    loop {}
}
