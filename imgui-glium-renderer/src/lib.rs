#[macro_use]
extern crate glium;
extern crate imgui;

mod context;
mod render;

pub use context::{AppConfig, AppContext};
pub use render::Texture;
