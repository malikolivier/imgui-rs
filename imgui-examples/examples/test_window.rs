extern crate aflak_imgui as imgui;
extern crate aflak_imgui_glium_renderer as imgui_glium_renderer;
extern crate glium;

mod support;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

fn main() {
    support::run("test_window.rs".to_owned(), CLEAR_COLOR, |ui| {
        let mut open = true;
        ui.show_demo_window(&mut open);
        open
    });
}
