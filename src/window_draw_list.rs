use sys;
use sys::{ImDrawList, ImU32};

use super::{ImVec2, ImVec4, Ui};

use std::marker::PhantomData;

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

#[derive(Debug)]
pub struct WindowDrawList<'ui> {
    draw_list: *mut ImDrawList,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> WindowDrawList<'ui> {
    pub fn new(_: &Ui<'ui>) -> Self {
        Self {
            draw_list: unsafe {
                sys::igGetWindowDrawList()
            },
            _phantom: PhantomData,
        }
    }

    pub fn channels_split<F: FnOnce(&ChannelsSplit)>(&self, channels_count: u32, f: F) {
        unsafe { sys::ImDrawList_ChannelsSplit(self.draw_list, channels_count as i32) };
        f(&ChannelsSplit(self));
        unsafe { sys::ImDrawList_ChannelsMerge(self.draw_list) };
    }

    pub fn add_line<P1, P2, C>(&self, p1: P1, p2: P2, c: C) -> Line
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<DrawColor>,
    {
        Line::new(self, p1, p2, c)
    }
}

pub struct ChannelsSplit<'ui>(&'ui WindowDrawList<'ui>);

impl<'ui> ChannelsSplit<'ui> {
    pub fn channels_set_current(&self, channel_index: u32) {
        unsafe { sys::ImDrawList_ChannelsSetCurrent(self.0.draw_list, channel_index as i32) };
    }
}

pub struct Line<'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: DrawColor,
    thickness: f32,
    draw_list: &'ui WindowDrawList<'ui>,
}

impl<'ui> Line<'ui> {
    fn new<P1, P2, C>(draw_list: &'ui WindowDrawList<'ui>, p1: P1, p2: P2, c: C) -> Self
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
                self.draw_list.draw_list,
                self.p1,
                self.p2,
                self.color.into(),
                self.thickness,
            )
        }
    }
}
