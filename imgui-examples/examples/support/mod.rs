use imgui::{FontGlyphRange, ImFontConfig, ImGui};

pub fn use_japanese_font(imgui: &mut ImGui) {
    let config = ImFontConfig::new()
        .oversample_h(1)
        .pixel_snap_h(true)
        .size_pixels(13.0);
    config.rasterizer_multiply(1.75).add_font(
        &mut imgui.fonts(),
        include_bytes!("../mplus-1p-regular.ttf"),
        &FontGlyphRange::japanese(),
    );
    config.merge_mode(true).add_default_font(&mut imgui.fonts());
}
