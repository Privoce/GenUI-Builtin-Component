use makepad_widgets::*;

use crate::styles::Radius;

use super::ViewState;

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

impl ViewProp {
    pub fn get(&self, state: ViewState) -> &ViewBasicProp {
        match state {
            ViewState::None => &self.basic,
            ViewState::Hover => &self.hover,
            ViewState::Pressed => &self.pressed,
        }
    }
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
    pub spread_radius: f32,
    #[live]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub background_visible: bool,
    #[live]
    pub rotation: f32,
    #[live]
    pub scale: f32,
    #[live]
    pub padding: Padding,
    #[live]
    pub margin: Margin,
    #[live]
    pub clip_x: bool,
    #[live]
    pub clip_y: bool,
    #[live]
    pub align: Align,
    #[live]
    pub cursor: MouseCursor,
    #[live]
    pub flow: Flow,
    #[live]
    pub spacing: f64,
}
