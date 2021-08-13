#[allow(dead_code)]


#[derive(Debug, Clone, Copy)]
#[repr(C)] // #[repr(u32)]
pub enum PixelFormat {
    KPixelRGBReserved8BitPerColor,
    KPixelBGRReserved8BitPerColor,
}

#[derive(Debug)]
#[repr(C)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub pixels_per_scan_line: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
}
