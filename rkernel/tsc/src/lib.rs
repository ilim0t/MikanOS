//! This crate allows you to read the x86 timestamp counter (TSC)
//! for when you require very low overhead time measurements.
//!
//! Unlike other crates, this one does not require nightly Rust as it
//! uses `cc` instead of inline asm - at the cost of an additional
//! `call`+`ret` on every invocation.
#![no_std]

mod ffi {
    extern "C" {
        pub fn rdtsc() -> u64;
    }
}

/// Simply invoke `rdtsc` and return the result.
#[inline(always)]
pub fn rdtsc() -> u64 {
    unsafe {
        ffi::rdtsc()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = rdtsc();
        let b = rdtsc();
        assert!(a < b);
    }
}
