extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate imgui_sys as sys;

mod support;

use imgui::*;

use imgui_glium_renderer::AppContext;

fn main() {
    let mut app = AppContext::init("custom_texture.rs".to_owned(), Default::default()).unwrap();

    support::use_japanese_font(app.imgui_mut());

    // Make a custom texture looking like a gradiant
    let mut image_data: Vec<Vec<(f32, f32, f32, f32)>> = Vec::new();
    for i in 0..100 {
        let mut row: Vec<(f32, f32, f32, f32)> = Vec::new();
        for j in 0..100 {
            row.push((i as f32 / 100.0, j as f32 / 100.0, 0.0, 1.0));
        }
        image_data.push(row);
    }
    let texture_ref = app.register_texture(image_data).unwrap();
    let (font_texture_id, font_texture_size) = {
        let fonts = app.imgui_mut().fonts();
        (fonts.get_texture_id().unwrap(), fonts.texture_size())
    };

    app.run(|ui| {
        ui.window(im_str!("Custom texture"))
            .size((300.0, 400.0), ImGuiCond::FirstUseEver)
            .build(|| {
                let tex_w = 100.0;
                let tex_h = 100.0;
                ui.image(&texture_ref, [tex_w, tex_h])
                    .expect("Texture not found")
                    .build();
                ui.image(font_texture_id, font_texture_size)
                    .unwrap()
                    .build();
            });
        true
    }).unwrap();
}
