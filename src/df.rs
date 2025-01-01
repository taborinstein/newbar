#[repr(C)]
struct DF {
        total: i32,
        used: i32
    }
extern "C" {
    
    fn df_root() -> DF;
}

pub fn df() -> f32 {
    let df_out = unsafe { df_root() };
    df_out.used as f32 / df_out.total as f32

}