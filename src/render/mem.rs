use super::{super::Store, circle, default_color, fullwidth_grad, rg_g, rg_r};
use std::fs;
use gtk::cairo;

fn get_mem() -> f32 {
    let meminfo = fs::read_to_string("/proc/meminfo").expect("Could not read /proc/meminfo");
    let mut parts = meminfo.split("\n");
    let total_s = parts.nth(0).unwrap().split(" ").collect::<Vec<&str>>();
    let avail_s = parts.nth(1).unwrap().split(" ").collect::<Vec<&str>>();

    let total = total_s[total_s.len() - 2].parse::<f32>().unwrap();
    let avail = avail_s[avail_s.len() - 2].parse::<f32>().unwrap();
    // println!("{}", parts.nth(2).unwrap());
    
    ((total - avail) / total) * 100.0
}
pub fn mem(cr: &cairo::Context, x: f64, clk: i32, store: &mut Store) {
    if clk % 30 == 0 {
        store.mem = get_mem();
    }
    let m = store.mem;
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
    cr.show_text("RAM: ").unwrap();
    cr.set_source_rgb(rg_r(100.0 - m) as f64, rg_g(100.0 - m) as f64, 0.0);
    if m > 9.999 {
        cr.show_text(format!("{:.1}%", m).as_str()).unwrap();
    } else {
        cr.show_text(format!("{:.2}%", m).as_str()).unwrap();
    }
    cr.stroke().unwrap();
}