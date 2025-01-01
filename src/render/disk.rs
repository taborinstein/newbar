use super::{super::Store, circle, default_color, fullwidth_grad, rg_g, rg_r};
use gtk::cairo;
use super::df;

pub fn disk(cr: &cairo::Context, x: f64, clk: i32, store: &mut Store) {
    if clk % 60 == 0 {
        store.df = df::df() * 100.0;
    }
    let p = store.df;
    let w = 80.0;
    default_color(cr);
    circle(cr, x + 15.0, -1);
    cr.move_to(x + 15.0, 0.0);
    cr.line_to(x + 15.0 + w, 0.0);
    circle(cr, x + 15.0 + w, 1);
    cr.line_to(x + 15.0, 30.0);
    cr.fill().unwrap();
    fullwidth_grad(cr);
    cr.move_to(x + 15.0, 20.0);

    cr.show_text("/: ").unwrap();
    cr.set_source_rgb(rg_r(100.0 - p) as f64, rg_g(100.0 - p) as f64, 0.0);
    if p > 9.999 {
        cr.show_text(format!("{:.3}%", p).as_str()).unwrap();
    } else {
        cr.show_text(format!("{:.4}%", p).as_str()).unwrap();
    }
    cr.stroke().unwrap();
}
