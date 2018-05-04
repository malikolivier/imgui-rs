use std::mem;
use std::ops::Deref;

use gfx::handle::ShaderResourceView;
use gfx::{CombinedError, Factory, Resources};
use imgui::{FromImTexture, ImTexture, ImTextureID};

pub struct Texture<R: Resources> {
    view: ShaderResourceView<R, [f32; 4]>,
    width: u16,
    height: u16,
}

impl<R: Resources> ImTexture for Texture<R> {
    fn get_id(&self) -> ImTextureID {
        unsafe { mem::transmute(self) }
    }
    fn get_size(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }
}

impl<R: Resources> Deref for Texture<R> {
    type Target = ShaderResourceView<R, [f32; 4]>;
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl<R: Resources> FromImTexture for Texture<R> {
    fn from_id<'a>(texture_id: ImTextureID) -> &'a Self {
        unsafe { mem::transmute::<_, &Texture<R>>(texture_id) }
    }
}

impl<R: Resources> Texture<R> {
    pub fn from_raw<F>(
        factory: &mut F,
        width: u16,
        height: u16,
        data: &[u8],
    ) -> Result<Self, CombinedError>
    where
        F: Factory<R>,
    {
        use gfx::format::Rgba8;
        use gfx::texture::{AaMode, Kind, Mipmap};

        let kind = Kind::D2(width, height, AaMode::Single);
        factory
            .create_texture_immutable_u8::<Rgba8>(kind, Mipmap::Provided, &[data])
            .map(|(_, view)| Texture {
                view,
                width,
                height,
            })
    }
}
