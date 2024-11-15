use cc;

fn main() {
    cc::Build::new()
        .file("src/audio.c")
        .compile("audio");
    println!("cargo:rustc-link-lib=pulse")
}