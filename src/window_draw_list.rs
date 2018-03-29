use sys;
use sys::{ImDrawCornerFlags, ImDrawList, ImU32};

use super::{ImVec2, ImVec4, Ui};

use std::marker::PhantomData;

/// Wrap ImU32 (a type typically used by ImGui to store packed colors)
/// This type is used to represent the color of drawing primitives in ImGui's
/// custom drawing API.
///
/// The type implements `From<ImU32>`, `From<ImVec4>`, `From<[f32; 4]>`,
/// `From<[f32; 3]>, `From<(f32, f32, f32, f32)>` and `From<(f32, f32, f32)>`
/// for convenience. If alpha is not provided, it is assumed to be 1.0 (255).
#[derive(Copy, Clone)]
pub struct ImColor(ImU32);

impl From<ImColor> for ImU32 {
    fn from(color: ImColor) -> Self { color.0 }
}

impl From<ImU32> for ImColor {
    fn from(color: ImU32) -> Self { ImColor(color) }
}

impl From<ImVec4> for ImColor {
    fn from(v: ImVec4) -> Self { ImColor(unsafe { sys::igColorConvertFloat4ToU32(v) }) }
}

impl From<[f32; 4]> for ImColor {
    fn from(v: [f32; 4]) -> Self { ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) }) }
}

impl From<(f32, f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

impl From<[f32; 3]> for ImColor {
    fn from(v: [f32; 3]) -> Self { [v[0], v[1], v[2], 1.0].into() }
}

impl From<(f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32)) -> Self { [v.0, v.1, v.2, 1.0].into() }
}

/// All types from which ImGui's custom draw API can be used implement this
/// trait.
pub trait DrawAPI<'ui> {
    fn draw_list(&self) -> *mut ImDrawList;
}

pub struct WindowDrawList<'ui> {
    draw_list: *mut ImDrawList,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> DrawAPI<'ui> for WindowDrawList<'ui> {
    fn draw_list(&self) -> *mut ImDrawList { self.draw_list }
}

impl<'ui> WindowDrawList<'ui> {
    pub fn new(_: &Ui<'ui>) -> Self {
        Self {
            draw_list: unsafe { sys::igGetWindowDrawList() },
            _phantom: PhantomData,
        }
    }

    pub fn channels_split<F: FnOnce(&ChannelsSplit)>(&self, channels_count: u32, f: F) {
        unsafe { sys::ImDrawList_ChannelsSplit(self.draw_list, channels_count as i32) };
        f(&ChannelsSplit(self));
        unsafe { sys::ImDrawList_ChannelsMerge(self.draw_list) };
    }
}

pub struct ChannelsSplit<'ui>(&'ui WindowDrawList<'ui>);

impl<'ui> DrawAPI<'ui> for ChannelsSplit<'ui> {
    fn draw_list(&self) -> *mut ImDrawList { self.0.draw_list }
}

impl<'ui> ChannelsSplit<'ui> {
    pub fn channels_set_current(&self, channel_index: u32) {
        unsafe { sys::ImDrawList_ChannelsSetCurrent(self.draw_list(), channel_index as i32) };
    }
}

macro_rules! impl_draw_list_methods {
    ($T: ident) => {
        impl<'ui> $T<'ui>
        where
            $T<'ui>: DrawAPI<'ui>,
        {
            pub fn add_line<P1, P2, C>(&self, p1: P1, p2: P2, c: C) -> Line<'ui, $T>
            where
                P1: Into<ImVec2>,
                P2: Into<ImVec2>,
                C: Into<ImColor>,
            {
                Line::new(self, p1, p2, c)
            }

            pub fn add_rect<P1, P2, C>(&self, p1: P1, p2: P2, c: C) -> Rect<'ui, $T>
            where
                P1: Into<ImVec2>,
                P2: Into<ImVec2>,
                C: Into<ImColor>,
            {
                Rect::new(self, p1, p2, c)
            }

            pub fn add_rect_filled_multicolor<P1, P2, C1, C2, C3, C4>(
                &self,
                p1: P1,
                p2: P2,
                c1: C1,
                c2: C2,
                c3: C3,
                c4: C4,
            ) where
                P1: Into<ImVec2>,
                P2: Into<ImVec2>,
                C1: Into<ImColor>,
                C2: Into<ImColor>,
                C3: Into<ImColor>,
                C4: Into<ImColor>,
            {
                unsafe {
                    sys::ImDrawList_AddRectFilledMultiColor(
                        self.draw_list(),
                        p1.into(),
                        p2.into(),
                        c1.into().into(),
                        c2.into().into(),
                        c3.into().into(),
                        c4.into().into(),
                    );
                }
            }
        }
    };
}

impl_draw_list_methods!(WindowDrawList);
impl_draw_list_methods!(ChannelsSplit);

pub struct Line<'ui, D: 'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: ImColor,
    thickness: f32,
    draw_list: &'ui D,
}

impl<'ui, D: DrawAPI<'ui>> Line<'ui, D> {
    fn new<P1, P2, C>(draw_list: &'ui D, p1: P1, p2: P2, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            color: c.into(),
            thickness: 1.0,
            draw_list,
        }
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn build(self) {
        unsafe {
            sys::ImDrawList_AddLine(
                self.draw_list.draw_list(),
                self.p1,
                self.p2,
                self.color.into(),
                self.thickness,
            )
        }
    }
}

pub struct Rect<'ui, D: 'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: ImColor,
    rounding: f32,
    flags: ImDrawCornerFlags,
    thickness: f32,
    filled: bool,
    draw_list: &'ui D,
}

impl<'ui, D: DrawAPI<'ui>> Rect<'ui, D> {
    fn new<P1, P2, C>(draw_list: &'ui D, p1: P1, p2: P2, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            color: c.into(),
            rounding: 0.0,
            flags: ImDrawCornerFlags::All,
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn round_top_left(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::TopLeft, value);
        self
    }

    pub fn round_top_right(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::TopRight, value);
        self
    }

    pub fn round_bot_left(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::BotLeft, value);
        self
    }

    pub fn round_bot_right(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::BotRight, value);
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddRectFilled(
                    self.draw_list.draw_list(),
                    self.p1,
                    self.p2,
                    self.color.into(),
                    self.rounding,
                    self.flags,
                );
            }
        } else {
            unsafe {
                sys::ImDrawList_AddRect(
                    self.draw_list.draw_list(),
                    self.p1,
                    self.p2,
                    self.color.into(),
                    self.rounding,
                    self.flags,
                    self.thickness,
                );
            }
        }
    }
}
