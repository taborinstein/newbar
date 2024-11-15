use super::audio;
use gtk::cairo;

use super::{circle, fullwidth_grad, default_color};

pub fn vol(cr: &cairo::Context, x: f64) {
    let w = 95.0;
    let v = audio::vol() as i32;
    default_color(cr);
    circle(cr, x + 15.0, -1);
    cr.move_to(x + 15.0, 0.0);
    cr.line_to(x + 15.0 + w, 0.0);
    circle(cr, x + 15.0 + w, 1);
    cr.line_to(x + 15.0, 30.0);
    cr.fill().unwrap();
    fullwidth_grad(cr);
    cr.move_to(x + 15.0, 20.0);
    cr.show_text("Volume:").unwrap();
    if audio::muted() {
        cr.set_source_rgba(1.0, 0.4, 0.4, 1.0);
        cr.show_text(" --%").unwrap();
    } else {
        cr.set_source_rgba(0.4, 1.0, 0.4, 1.0);
        cr.show_text(format!(" {}%", v).as_str()).unwrap();
    }
    cr.fill().unwrap();
    cr.move_to(x + 15.0, 20.0);
    let pat = cairo::LinearGradient::new(15.0 + x, 15.0, x + 15.0 + 40.0, 15.0);
    pat.add_color_stop_rgb(0.0, 0.4, 1.0, 0.4);
    pat.add_color_stop_rgb(0.5, 1.0, 1.0, 0.4);
    pat.add_color_stop_rgb(1.0, 1.0, 0.4, 0.4);
    cr.set_source(pat).unwrap();
    const CHARS: [&str; 7] = ["V", "o", "l", "u", "m", "e", ""];
    if !audio::muted() {
        for i in 0..v / 15 {
            cr.show_text(CHARS[i as usize]).unwrap();
        }
    }
    cr.fill().unwrap();
}
