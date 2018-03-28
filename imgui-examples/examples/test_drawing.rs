extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::DrawAPI;

mod support;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const WHITE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    support::run("test_drawing".to_owned(), CLEAR_COLOR, |ui| {
        ui.with_window_draw_list(|draw_list| {
           draw_list.add_line([100.0, 100.0], [200.0, 200.0], WHITE_COLOR).build();
           draw_list.add_line([200.0, 100.0], [400.0, 200.0], WHITE_COLOR).build();
        });
        true
    });
}
