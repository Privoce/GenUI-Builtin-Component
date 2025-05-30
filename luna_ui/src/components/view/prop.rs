use makepad_widgets::*;

use crate::{shader::ViewDrawProp, styles::Radius};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ViewProp {
    #[live]
    pub basic: ViewBasicProp,
    #[live]
    pub hover: ViewBasicProp,
    #[live]
    pub pressed: ViewBasicProp,
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ViewBasicProp {
    #[live]
    pub background_color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub border_width: f32,
    #[live]
    pub border_radius: Radius,
    #[live]
    pub shadow_color: Vec4,
    #[live]
    pub spread_radius: Radius,
    #[live]
    pub blur_radius: Radius,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub background_visible: f32,
    #[live]
    pub opacity: f32,
    #[live]
    pub rotation: f32,
    #[live]
    pub scale: f32,
}

