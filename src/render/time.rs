use gtk::cairo;
use chrono;

use super::{circle, default_color, fullwidth_grad};


pub fn time(cr: &cairo::Context, x: f64) {
    let w = 50.0;
    default_color(cr);
    circle(cr, x + 15.0, -1);
    cr.move_to(x + 15.0, 0.0);
    cr.line_to(x + w + 15.0, 0.0);
    circle(cr, x + w + 15.0 + w, 1);
    cr.line_to(x + 15.0, 30.0);
    cr.fill().unwrap();
    cr.move_to(x + 10.0, 20.0);
    fullwidth_grad(cr);
    cr.show_text(chrono::Local::now().format("%m/%d %I:%M%P").to_string().as_str()).unwrap();
    cr.stroke().unwrap();
}