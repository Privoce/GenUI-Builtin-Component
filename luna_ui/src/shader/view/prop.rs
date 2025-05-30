use makepad_widgets::*;

use crate::components::view::ViewBasicProp;

/// ## ViewDrawProp
/// This struct defines the properties for drawing a view in the shader
#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ViewDrawProp {
    #[live]
    pub background_color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub border_width: f32,
    #[live]
    pub border_radius: Vec4,
    #[live]
    pub shadow_color: Vec4,
    #[live]
    pub spread_radius: Vec4,
    #[live]
    pub blur_radius: Vec4,
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

impl From<ViewBasicProp> for ViewDrawProp {
    fn from(value: ViewBasicProp) -> Self {
        let ViewBasicProp {
            background_color,
            border_color,
            border_width,
            border_radius,
            shadow_color,
            spread_radius,
            blur_radius,
            shadow_offset,
            background_visible,
            opacity,
            rotation,
            scale,
        } = value;

        Self {
            background_color,
            border_color,
            border_width,
            border_radius: border_radius.into(),
            shadow_color,
            spread_radius: spread_radius.into(),
            blur_radius: blur_radius.into(),
            shadow_offset,
            background_visible,
            opacity,
            rotation,
            scale,
        }
    }
}
