use std::rc::Rc;

use sys::{self, ImTextureID, ImVec2, ImVec4};

pub trait GetTextureID {
    fn get_texture_id(&self) -> Option<ImTextureID>;
    fn get_size(&self) -> (u32, u32);
}

#[derive(Clone)]
pub struct AnyTexture(Rc<Box<GetTextureID>>);

impl AnyTexture {
    pub(crate) fn new<T: 'static + GetTextureID>(texture: T) -> Self {
        AnyTexture(Rc::new(Box::new(texture)))
    }

    pub fn get_size(&self) -> (f32, f32) {
        let size = self.0.get_size();
        (size.0 as f32, size.1 as f32)
    }
}

use std::ops::Deref;

impl Deref for AnyTexture {
    type Target = Box<GetTextureID>;
    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.0)
    }
}

pub trait IntoTexture<T>
where T: GetTextureID,
{
    fn into_texture(self) -> T;
}


pub struct Image {
    texture_id: ImTextureID,
    size: ImVec2,
    uv0: ImVec2,
    uv1: ImVec2,
    tint_col: ImVec4,
    border_col: ImVec4,
}

impl Image {
    pub fn new<S>(texture: &AnyTexture, size: S) -> Result<Image, String>
    where
        S: Into<ImVec2>,
    {
        const DEFAULT_UV0: ImVec2 = ImVec2 { x: 0.0, y: 0.0 };
        const DEFAULT_UV1: ImVec2 = ImVec2 { x: 1.0, y: 1.0 };
        const DEFAULT_TINT_COL: ImVec4 = ImVec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        const DEFAULT_BORDER_COL: ImVec4 = ImVec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        if let Some(texture_id) = texture.0.get_texture_id() {
            Ok(Image {
                texture_id,
                size: size.into(),
                uv0: DEFAULT_UV0,
                uv1: DEFAULT_UV1,
                tint_col: DEFAULT_TINT_COL,
                border_col: DEFAULT_BORDER_COL,
            })
        } else {
            Err("Texture was dropped!".to_owned())
        }
    }

    pub fn uv0<T: Into<ImVec2>>(mut self, uv0: T) -> Self {
        self.uv0 = uv0.into();
        self
    }

    pub fn uv1<T: Into<ImVec2>>(mut self, uv1: T) -> Self {
        self.uv1 = uv1.into();
        self
    }

    pub fn tint_col<T: Into<ImVec4>>(mut self, tint_col: T) -> Self {
        self.tint_col = tint_col.into();
        self
    }

    pub fn border_col<T: Into<ImVec4>>(mut self, border_col: T) -> Self {
        self.border_col = border_col.into();
        self
    }

    pub fn build(self) {
        unsafe {
            sys::igImage(
                self.texture_id,
                self.size,
                self.uv0,
                self.uv1,
                self.tint_col,
                self.border_col,
            );
        }
    }
}
