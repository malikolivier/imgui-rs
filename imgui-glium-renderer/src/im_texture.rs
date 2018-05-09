use std::borrow::Cow;
use std::mem;
use std::ops::Deref;

use glium::backend::Facade;
use glium::texture::{PixelValue, RawImage2d, Texture2dDataSource, TextureCreationError};
use glium::Texture2d;
use imgui::{FromImTexture, ImTexture, ImTextureID};

/// Handle to a glium texture
///
/// Implements [`Deref`] to get direct access to the underlying [`Texture2d`]
/// object.
pub struct Texture(Texture2d);

impl ImTexture for Texture {
    fn get_id(&self) -> ImTextureID {
        unsafe { mem::transmute(self) }
    }
    fn get_size(&self) -> (u32, u32) {
        self.0.dimensions()
    }
}

impl Deref for Texture {
    type Target = Texture2d;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromImTexture for Texture {
    fn from_id<'a>(texture_id: ImTextureID) -> &'a Self {
        unsafe { mem::transmute::<_, &Texture>(texture_id) }
    }
}

impl Texture {
    pub fn from_texture_2d(texture: Texture2d) -> Self {
        Texture(texture)
    }

    pub fn from_data<'a, F, T>(facade: &F, data: T) -> Result<Self, TextureCreationError>
    where
        T: Texture2dDataSource<'a>,
        F: Facade,
    {
        Texture2d::new(facade, data).map(Texture)
    }

    pub fn from_raw<F, P>(
        facade: &F,
        width: u32,
        height: u32,
        data: &[P],
    ) -> Result<Self, TextureCreationError>
    where
        F: Facade,
        P: PixelValue,
    {
        Self::from_data(
            facade,
            RawImage2d {
                data: Cow::Borrowed(data),
                width,
                height,
                format: <P as PixelValue>::get_format(),
            },
        )
    }
}
