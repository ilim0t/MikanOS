use super::*;
use spin::{Mutex, Once};

pub static CONSOLE: Mutex<Once<Console>> = Mutex::new(Once::new());
