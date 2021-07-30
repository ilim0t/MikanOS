#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rkernel::graphics::{Color, FrameBufferConfig, PixelPoint, Writer};
use rkernel::misc::*;

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(frame_buffer_config: &FrameBufferConfig) -> ! {
    let mut writer = Writer::new(frame_buffer_config);

    writer.write_all(&Color {
        r: 50,
        g: 50,
        b: 50,
    });
    // let point = writer.at(&Point { x: 120000, y: 200000 });  // panic

    for x in 0..writer.frame_size.horizontal_resolution {
        for y in 0..100 {
            writer.write(
                &PixelPoint { x, y },
                &Color {
                    r: 50,
                    g: 100,
                    b: 50,
                },
            );
        }
    }

    writer.write_ascii(
        &PixelPoint { x: 80, y: 80 },
        '?',
        &Color {
            r: 255,
            g: 255,
            b: 255,
        },
    );

    halt();
}
