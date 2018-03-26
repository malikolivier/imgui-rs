use std::marker::PhantomData;

use sys;
use sys::{ImDrawList, ImU32};

use super::{ImVec2, ImVec4, Ui};

/// Wrap ImU32 (a type typically used by ImGui to store packed colors)
#[derive(Copy, Clone)]
pub struct DrawColor(ImU32);

impl From<DrawColor> for ImU32 {
    fn from(color: DrawColor) -> Self { color.0 }
}

impl From<ImVec4> for DrawColor {
    fn from(v: ImVec4) -> Self { DrawColor(unsafe { sys::igColorConvertFloat4ToU32(v) }) }
}

impl From<[f32; 4]> for DrawColor {
    fn from(v: [f32; 4]) -> Self { DrawColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) }) }
}

impl From<(f32, f32, f32, f32)> for DrawColor {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        DrawColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

pub struct WindowDrawList<'ui> {
    draw_list: &'ui mut ImDrawList,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> WindowDrawList<'ui> {
    pub fn new(_: &Ui<'ui>) -> Self {
        Self {
            draw_list: unsafe {
                let draw_list = sys::igGetWindowDrawList();
                &mut *draw_list
            },
            _phantom: PhantomData,
        }
    }

    pub fn add_line<P1, P2, C>(&'ui mut self, p1: P1, p2: P2, c: C) -> Line<'ui>
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<DrawColor>,
    {
        Line::new(self.draw_list, p1, p2, c)
    }
}

pub struct Line<'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: DrawColor,
    thickness: f32,
    draw_list: &'ui mut ImDrawList,
}

impl<'ui> Line<'ui> {
    fn new<P1, P2, C>(draw_list: &'ui mut ImDrawList, p1: P1, p2: P2, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<DrawColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            color: c.into(),
            thickness: 1.0,
            draw_list,
        }
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn build(self) {
        unsafe {
            sys::ImDrawList_AddLine(
                self.draw_list as *mut ImDrawList,
                self.p1,
                self.p2,
                self.color.into(),
                self.thickness,
            )
        }
    }
}
