use cc;
fn build_audio() {
    println!("cargo::rerun-if-changed=src/audio.c");
    cc::Build::new().file("src/audio.c").compile("audio");
}
fn build_df() {
    println!("cargo::rerun-if-changed=src/df.c");
    cc::Build::new().file("src/df.c").compile("df");
}
fn main() {
    build_audio();
    build_df();
    println!("cargo:rustc-link-lib=pulse")
}
