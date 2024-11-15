use super::{super::Store, circle, default_color, fullwidth_grad, rg_g, rg_r};
use gtk::cairo;
use std::fs;

fn get_bat() -> f32 {
    let now_s = fs::read_to_string("/sys/class/power_supply/BAT0/energy_now")
        .expect("Could not open energy_now file");
    let full_s = fs::read_to_string("/sys/class/power_supply/BAT0/energy_full")
        .expect("Could not open energy_full file");
    now_s
        .trim()
        .parse::<f32>()
        .expect("Could not parse energy_now")
        / full_s
            .trim()
            .parse::<f32>()
            .expect("Could not parse energy_full")
        * 100.0
}
fn get_charging() -> bool /* too lazy to put this in the last func */ {
    fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .expect("Could not open status file")
        .starts_with("Charging")
}
pub fn bat(cr: &cairo::Context, x: f64, clk: i32, store: &mut Store) {
    if clk % 30 == 0 {
        store.battery = get_bat();
        store.chg = get_charging();
    }
    let w = 125.0;
    default_color(cr);
    circle(cr, x + 15.0, -1);
    cr.move_to(x + 15.0, 0.0);
    cr.line_to(x + 15.0 + w, 0.0);
    circle(cr, x + 15.0 + w, 1);
    cr.line_to(x + 15.0, 30.0);
    // cr.fill().unwrap();
    if store.battery > 15.0 || (clk % 60 / 2 < 16 && !store.chg) {
        default_color(cr);
    } else {
        let pat = cairo::LinearGradient::new(15.0 + x, 15.0, x + 15.0 + 40.0, 15.0);
        pat.add_color_stop_rgba(0.0, 1.0, 0.5, 0.2, 0.5);
        pat.add_color_stop_rgba(1.0, 1.0, 0.2, 0.2, 0.5);
        cr.set_source(pat).unwrap();
    }
    cr.fill().unwrap();
    cr.move_to(x + 15.0, 20.0);
    fullwidth_grad(cr);
    cr.show_text("Battery: ").unwrap();

    if store.chg {
        // cr.set_source_rgba(0.4, 1.0, 0.4, 1.0);
        let pt = cr.current_point().unwrap();
        let pat = cairo::LinearGradient::new(pt.0, pt.1, pt.0 + 60.0, pt.1);
        pat.add_color_stop_rgb(
            0.0,
            rg_r(store.battery) as f64,
            rg_g(store.battery) as f64,
            0.0,
        );
        pat.add_color_stop_rgb(1.0, 0.4, 1.0, 0.4);
        cr.set_source(pat).unwrap();
    } else {
        cr.set_source_rgb(rg_r(store.battery) as f64, rg_g(store.battery) as f64, 0.0);
    }
    if store.battery > 99.99 {
        cr.show_text(format!("{:.2}%", store.battery).as_str())
            .unwrap();
    } else {
        cr.show_text(format!("{:.3}%", store.battery).as_str())
            .unwrap();
    }
    // cr.show_text("%").unwrap();

    cr.stroke().unwrap();
}
