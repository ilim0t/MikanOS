#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use rkernel::graphics::{Color, Console, FrameBufferConfig, Writer, CONSOLE};
use rkernel::misc::*;

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

fn init(frame_buffer_config: &FrameBufferConfig) {
    CONSOLE.lock().call_once(|| {
        let writer = Writer::new(frame_buffer_config);
        // let point = writer.at(&PixelPoint { x: 120000, y: 200000 });  // panic

        Console::new(
            writer,
            Color { r: 0, g: 20, b: 0 },
            Color {
                r: 255,
                g: 255,
                b: 255,
            },
        )
    });
}

#[no_mangle]
pub extern "C" fn _start(frame_buffer_config: &FrameBufferConfig) -> ! {
    init(frame_buffer_config);

    // for i in 0..0xff {
    //     writer.write_byte(
    //         &PixelPoint {
    //             x: (8 * (i % 20)) as usize,
    //             y: (16 * (i / 20)) as usize,
    //         },
    //         i,
    //         &Color {
    //             r: 255,
    //             g: 255,
    //             b: 255,
    //         },
    //     );
    // }

    CONSOLE
        .lock()
        .get_mut()
        .unwrap()
        .write_string("Hello, World! こんにちは\n");

    writeln!(CONSOLE.lock().get_mut().unwrap(), "Hello, World!").unwrap();
    writeln!(CONSOLE.lock().get_mut().unwrap(), "こんにちは、世界!").unwrap();
    writeln!(CONSOLE.lock().get_mut().unwrap(), "1/3={}", 1. / 3.).unwrap();

    halt();
}
