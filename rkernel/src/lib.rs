#![no_std]
#![feature(asm)]

pub mod graphics;
pub mod misc;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 2);
    }
}
