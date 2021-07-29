use super::*;
use core::slice;

#[allow(dead_code)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Clone, Copy)]
#[repr(C)] // 推測
           // #[repr(u32)]  // 明示的指定
pub enum PixelFormat {
    KPixelRGBReserved8BitPerColor,
    KPixelBGRReserved8BitPerColor,
}

impl PixelFormat {
    pub fn convert_into_ref_color(&self, ptr: *const u8) -> *const dyn RefColor {
        match self {
            PixelFormat::KPixelRGBReserved8BitPerColor => ptr as *const PixelRGBColor,
            PixelFormat::KPixelBGRReserved8BitPerColor => ptr as *const PixelBGRColor,
        }
    }

    pub fn convert_into_ref_color_mut(&self, ptr: *mut u8) -> *mut dyn RefColor {
        match self {
            PixelFormat::KPixelRGBReserved8BitPerColor => ptr as *mut PixelRGBColor,
            PixelFormat::KPixelBGRReserved8BitPerColor => ptr as *mut PixelBGRColor,
        }
    }
}

#[repr(C)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub pixels_per_scan_line: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
}

impl FrameBufferConfig {
    pub fn get_slice(&self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.frame_buffer,
                (self.vertical_resolution * self.pixels_per_scan_line * 4) as usize,
            )
        }
    }

    #[allow(dead_code)]
    fn at(&self, Point { x, y }: &Point) -> &dyn RefColor {
        let count = 4 * (self.pixels_per_scan_line as usize * y + x);

        let ptr: *const u8 = unsafe { self.frame_buffer.add(count) };
        let ref_color = self.pixel_format.convert_into_ref_color(ptr);
        unsafe { &*ref_color }
    }

    #[allow(dead_code)]
    fn at_mut(&mut self, Point { x, y }: &Point) -> &mut dyn RefColor {
        let count = 4 * (self.pixels_per_scan_line as usize * y + x);

        let ptr = unsafe { self.frame_buffer.add(count) };
        let ref_color = self.pixel_format.convert_into_ref_color_mut(ptr);
        unsafe { &mut *ref_color }
    }
}
