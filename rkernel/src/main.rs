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
            Color { r: 0, g: 0, b: 0 },
        )
    });
}

#[no_mangle]
pub extern "C" fn _start(frame_buffer_config: &FrameBufferConfig) -> ! {
    init(frame_buffer_config);

    for _ in 0..100 {
        for h in 0..HSV::AROUND {
            let color = HSV::new(h, 10, 90);

            CONSOLE.lock().get_mut().unwrap().bg_color = color.to_rgb();
            // CONSOLE.lock().get_mut().unwrap().clear();
            for _ in 0..10 {
                println!("{:?}, {:?}", color, color.to_rgb());
            }
        }
    }

    halt();
}

#[derive(Debug)]
struct HSV {
    h: i32,
    s: i32,
    v: i32,
}

impl HSV {
    const AROUND: i32 = 360;
    const SV_MAX: i32 = 100;

    fn new(h: i32, s: i32, v: i32) -> HSV {
        assert!(h < HSV::AROUND);
        assert!(s < HSV::SV_MAX);
        assert!(v < HSV::SV_MAX);

        assert!(HSV::SV_MAX * HSV::SV_MAX * HSV::AROUND < i32::MAX / 255);

        HSV { h, s, v }
    }

    fn to_rgb(&self) -> Color {
        let HSV { h, s, v } = self;

        let hight = 255 * v / HSV::SV_MAX;

        let mid = 255 * v * (HSV::SV_MAX * HSV::AROUND - s * ((6 * h) % HSV::AROUND))
            / (HSV::SV_MAX * HSV::SV_MAX * HSV::AROUND);
        let rev_mid =
            255 * v * (HSV::SV_MAX * HSV::AROUND - s * (HSV::AROUND - (6 * h) % HSV::AROUND))
                / (HSV::SV_MAX * HSV::SV_MAX * HSV::AROUND);

        let low = 255 * v * (HSV::SV_MAX - s) / (HSV::SV_MAX * HSV::SV_MAX);

        let r;
        let g;
        let b;

        match h * 6 / HSV::AROUND {
            0 => {
                r = hight;
                g = rev_mid;
                b = low;
            }
            1 => {
                r = mid;
                g = hight;
                b = low;
            }
            2 => {
                r = low;
                g = hight;
                b = rev_mid;
            }
            3 => {
                r = low;
                g = mid;
                b = hight;
            }
            4 => {
                r = rev_mid;
                g = low;
                b = hight;
            }
            5 => {
                r = hight;
                g = low;
                b = mid;
            }
            _ => panic!(),
        };

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}
