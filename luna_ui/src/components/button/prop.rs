use makepad_widgets::*;

use crate::{styles::Radius, themes::Theme};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ButtonProp {
    #[live]
    pub basic: ButtonBasicProp,
    #[live]
    pub hover: ButtonBasicProp,
    #[live]
    pub pressed: ButtonBasicProp,
    #[live]
    pub disabled: ButtonBasicProp,
}
#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ButtonBasicProp {
    #[live]
    pub theme: Theme,
    // --- background ----------------
    #[live]
    pub background_color: Vec4,
    #[live]
    pub background_visible: bool,
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
    pub border_radius: Radius,
    // --- cursor -------------------
    #[live]
    pub cursor: MouseCursor,
}
