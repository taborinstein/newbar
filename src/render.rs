pub mod time;
pub mod volume;
pub mod battery;
pub mod workspace;
pub mod cpu;
pub mod mem;

use std::f64::consts::PI;

use gtk::cairo;
use super::audio;

pub fn fullwidth_grad(cr: &cairo::Context) {
    let pat = cairo::LinearGradient::new(0.0, 15.0, 1920.0 - 5.0, 15.0);
    pat.add_color_stop_rgb(0.000, 1.0, 0.0, 0.0);
    pat.add_color_stop_rgb(0.200, 1.0, 0.8, 0.0);
    pat.add_color_stop_rgb(0.400, 0.0, 1.0, 0.0);
    pat.add_color_stop_rgb(0.600, 0.0, 1.0, 0.8);
    pat.add_color_stop_rgb(0.800, 0.0, 0.0, 1.0);
    pat.add_color_stop_rgb(1.000, 1.0, 0.0, 1.0);
    cr.set_source(pat).unwrap();
}
pub fn circle(cr: &cairo::Context, x: f64, dir: i32)
{
    cr.arc(x, 15.0, 15.0, dir as f64 * 270.0 * (PI / 180.0), dir as f64 * 90.0 * (PI / 180.0));
}
pub fn default_color(cr: &cairo::Context) {
    // cr.set_source_rgba(0.129, 0.129, 0.141, 0.5);
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.1);
}
pub fn rg_g(b: f32) -> f32 {
    if b < 50.0 { b / 50.0 } else { 1.0 }
}
pub fn rg_r(b: f32) -> f32 {
    if b > 50.0 { 1.0 - ((b - 50.0)/ 50.0) } else { 1.0 }
}