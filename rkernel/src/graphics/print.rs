use super::*;
use core::fmt;
use spin::{Mutex, Once};

pub static CONSOLE: Mutex<Once<Console>> = Mutex::new(Once::new());

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::graphics::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().get_mut().unwrap().write_fmt(args).unwrap();
}
