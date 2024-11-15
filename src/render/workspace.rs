use gtk::cairo;

// use socket2::{Domain, Socket, Type};
// use std::net::{SocketAddr, TcpListener};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use super::{super::Store, circle, default_color, fullwidth_grad};
fn request(msg: &str, store: &Store) -> String {
    let mut stream = UnixStream::connect(format!(
        "/run/user/1000/hypr/{}/.socket.sock",
        store.hypr_path
    ))
    .unwrap();
    stream
        .write(msg.as_bytes())
        .expect("Failed to write to stream");
    let mut res = String::new();
    stream
        .read_to_string(&mut res)
        .expect("Failed to write to string");
    return res;
}
pub fn wkspc(cr: &cairo::Context, x: f64, clk: i32, store: &mut Store) {
    if clk % 2 == 0 {
        let active_msg = request("activeworkspace", store);
        store.active_workspace = active_msg
            .split(" ")
            .nth(2)
            .expect("Could not split string")
            .parse::<i32>()
            .unwrap();
        let workspace_msg = request("workspaces", store);
        for i in 0..10 {
            store.workspaces[i] = 0;
        }
        store.num_workspaces = 0;
        for part in workspace_msg.split("\n") {
            if part.starts_with("workspace ID") {
                let n = part
                    .split(" ")
                    .nth(2)
                    .expect("Could not split string")
                    .parse::<i32>()
                    .unwrap();
                if n == -98 {
                    continue;
                }
                store.workspaces[n as usize - 1] = 1;
                store.num_workspaces += 1;
            }
        }
    }
    let spacing = 15;
    let w = (spacing * store.num_workspaces) as f64 - 5.0;
    default_color(cr);
    circle(cr, x + 15.0, -1);
    cr.move_to(x + 15.0, 0.0);
    cr.line_to(x + 15.0 + w, 0.0);
    circle(cr, x + 15.0 + w, 1);
    cr.line_to(x + 15.0, 30.0);
    cr.fill().unwrap();
    cr.set_source_rgb(1.0, 1.0, 1.0);
    let mut pos = 0;
    // cr.show_text("0").unwrap();
    for i in 0..10 {
        if store.workspaces[i] == 1 {
            let chr = format!("{}", if i < 9 { i + 1 } else { 0 });
            fullwidth_grad(cr);
            if store.active_workspace == i as i32 + 1 {
                cr.move_to(x + 15.0 + pos as f64, 22.0);
                cr.show_text("_").unwrap();
            }
            cr.move_to(x + 15.0 + pos as f64, 20.0);
            cr.show_text(chr.as_str()).unwrap();
            cr.stroke().unwrap();
            pos += spacing;
        }
    }
    // cr.fill();
}
