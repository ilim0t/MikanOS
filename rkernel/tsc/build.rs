extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/asm.c")
        .compile("asm");
}
