#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rkernel::graphics::{Color, Console, FrameBufferConfig, PixelWriter, CONSOLE};
use rkernel::{misc::*, println};

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init(frame_buffer_config: &FrameBufferConfig) {
    CONSOLE.lock().call_once(|| {
        let pixel_writer = PixelWriter::new(frame_buffer_config);
        // let point = pixel_writer.at(&PixelPoint { x: 120000, y: 200000 });  // panic

        Console::new(
            pixel_writer,
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

    CONSOLE.lock().get_mut().unwrap().font_gallery();

    CONSOLE
        .lock()
        .get_mut()
        .unwrap()
        .write_bytes(b"Hello, World!\n");

    println!("Hello, World! こんにちは、世界!");
    println!("1/3={}", 1. / 3.);

    // panic!("Some panic message");

    halt();
}
