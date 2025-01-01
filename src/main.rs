use std::{time::Duration, env};


mod render;
mod audio;
mod df;



struct Store {
    battery: f32,
    chg: bool,
    active_workspace: i32,
    workspaces: [i32; 10],
    num_workspaces: i32,
    hypr_path: String,
    cpu: render::cpu::CPU,
    mem: f32,
    df: f32
}

use gtk::{cairo, gdk, glib, prelude::*};
use gtk4_layer_shell::{Edge, LayerShell};
fn draw_func(cr: &cairo::Context, clk: i32, store: &mut Store) {
    cr.set_line_width(3.0);
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
    cr.paint().unwrap();
    cr.set_font_size(15.0);
    cr.select_font_face("UbuntuMono Nerd Font", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    cr.new_path();
    let  mut node_pos: f64 = 1920.0 - 150.0;
    render::time::time(cr, node_pos);
    node_pos -= 135.0;
    render::volume::vol(cr, node_pos);
    node_pos -= 165.0;
    render::battery::bat(cr, node_pos, clk, store);
    node_pos -= 120.0;
    render::cpu::cpu(cr, node_pos, clk, store);
    node_pos -= 120.0;
    render::mem::mem(cr, node_pos, clk, store);
    node_pos -= 120.0;
    render::disk::disk(cr, node_pos, clk, store);
    node_pos = 0.0;
    render::workspace::wkspc(cr, node_pos, clk, store);
}
fn activate(application: &gtk::Application) {
    audio::thread_start();
    let window = gtk::ApplicationWindow::new(application);
    window.init_layer_shell();
    window.set_title(Some("newbar"));
    let display = gdk::Display::default().unwrap();
    let monitors = gdk::Display::monitors(&display);
    let monitor = monitors
        .item(0)
        .unwrap()
        .downcast::<gdk::Monitor>()
        .expect("Failed to downcast Object to Monitor");
    let workarea = monitor.geometry();
    window.set_default_size(workarea.width() - 20 + 1, 30);
    window.set_margin(Edge::Left, 5);
    window.set_margin(Edge::Right, 5);
    window.set_margin(Edge::Top, 5);
    window.auto_exclusive_zone_enable();
    let anchors = [
        (Edge::Left, false),
        (Edge::Right, false),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];
    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    let prov = gtk::CssProvider::new();
    prov.load_from_string("window{background-color:#0000;}");
    gtk::style_context_add_provider_for_display(
        &display,
        &prov,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let canvas = gtk::DrawingArea::new();
    let mut clk: i32 = 0;
    let mut store: Store = Store {
        battery: 0.0,
        active_workspace: 0,
        chg: false,
        num_workspaces: 0,
        workspaces: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        hypr_path: env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap(),
        cpu: render::cpu::CPU {
            work: 1,
            total: 1,
            percentage: 0.0
        },
        mem: 0.0,
        df: 0.0
    };
    canvas.set_draw_func(move |_area: &gtk::DrawingArea, cr: &cairo::Context, _width: i32, _height: i32| {
        draw_func(cr, clk, &mut store);
        clk += 1;
    });
    window.set_child(Some(&canvas));
    
    glib::timeout_add_local(Duration::from_millis(16), move || {
        canvas.queue_draw();
        glib::ControlFlow::Continue
    });
    window.present();
}
fn main() -> glib::ExitCode {
    let application = gtk::Application::new(Some("com.komali.newbar"), Default::default());
    application.connect_activate(activate);
    application.run()
}
