use super::{super::Store, circle, default_color, fullwidth_grad, rg_g, rg_r};
use gtk::cairo;
use std::fs;
pub struct CPU {
    pub work: i32,
    pub total: i32,
    pub percentage: f32,
}
fn get_cpu(store: &mut Store) {
    let stat = fs::read_to_string("/proc/stat").expect("Could not read /proc/stat");
    let mut total = 0;
    let mut work = 0;
    for line in stat.split("\n") {
        if line.starts_with("cpu") && !line.starts_with("cpu ") {
            let parts = line.split(" ");
            let mut i: usize = 0;
            for part in parts {
                if i == 0 {
                    i += 1;
                    continue;
                }
                if i < 4 {
                    work += part.parse::<i32>().unwrap();
                }
                total += part.parse::<i32>().unwrap();
                i += 1;
            }
        }
    }
    // println!("> {} {}", total, work);
    let percent = (work - store.cpu.work) as f32 / (total - store.cpu.total) as f32;
    store.cpu.work = work;
    store.cpu.total = total;
    store.cpu.percentage = percent * 100.0;
}
pub fn cpu(cr: &cairo::Context, x: f64, clk: i32, store: &mut Store) {
    if clk % 60 == 0 {
        get_cpu(store);
    }
    let p = store.cpu.percentage;
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
    cr.show_text("CPU: ").unwrap();
    cr.set_source_rgb(rg_r(100.0 - p) as f64, rg_g(100.0 - p) as f64, 0.0);
    if p > 9.999 {
        cr.show_text(format!("{:.1}%", p).as_str()).unwrap();
    } else {
        cr.show_text(format!("{:.2}%", p).as_str()).unwrap();
    }
    cr.stroke().unwrap();
}
