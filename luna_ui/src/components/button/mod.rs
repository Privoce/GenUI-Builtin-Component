use makepad_widgets::*;

use crate::{pure_after_apply, styles::BorderRadius, themes::Theme};

live_design!{
    link luna_basic;

    pub LButtonBase = {{LButton}}
}

#[derive(Live, Widget)]
pub struct LButton {
    #[live]
    pub theme: Theme,
    // --- background ----------------
    #[live]
    pub background_color: Vec4,
    #[live]
    pub background_visible: bool,
    #[live]
    pub hover_color: Vec4,
    #[live]
    pub pressed_color: Vec4,
    // --- shadow -------------------
    #[live]
    pub shadow_color: Vec4,
    #[live]
    pub spread_radius: f32,
    #[live]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    // --- border -------------------
    #[live]
    pub border_width: f32,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub border_radius: BorderRadius,
    // --- cursor -------------------
    #[live]
    pub cursor: MouseCursor,
    // --- visible -------------------
    #[live]
    pub visible: bool,
    #[walk]
    pub walk: Walk,
    
}

impl LiveHook for LButton {
    pure_after_apply!();
}