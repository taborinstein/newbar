extern "C" {
    fn vol_thread_start();
    fn get_vol() -> f32;
    fn get_muted() -> i32;
}
pub fn thread_start() {
    unsafe {
        vol_thread_start();
    }
}
pub fn vol() -> f32 {
    unsafe { get_vol() }
}
pub fn muted() -> bool {
    return match unsafe { get_muted() } {
        1 => true,
        _ => false,
    };
}