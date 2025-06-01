use makepad_widgets::*;

use crate::{styles::Radius, themes::Theme};

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

impl Default for ViewProp {
    fn default() -> Self {
        Self {
            basic: ViewBasicProp::new(Theme::Dark, ViewState::None),
            hover: ViewBasicProp::new(Theme::Dark, ViewState::Hover),
            pressed: ViewBasicProp::new(Theme::Dark, ViewState::Pressed),
        }
    }
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
    pub theme: Theme,
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

impl ViewBasicProp {
    pub fn new(theme: Theme, state: ViewState) -> Self {
        let (bg_level, border_level, shadow_level) = match state {
            ViewState::None => (400, 400, 300),
            ViewState::Hover => (300, 300, 200),
            ViewState::Pressed => (500, 500, 400),
        };

        let (background_color, border_color, shadow_color) = match theme {
            Theme::Dark => (
                Theme::Dark.color(bg_level),
                Theme::Dark.color(border_level),
                Theme::Dark.color(shadow_level),
            ),
            Theme::Primary => (
                Theme::Primary.color(bg_level),
                Theme::Primary.color(border_level),
                Theme::Primary.color(shadow_level),
            ),
            Theme::Error => (
                Theme::Error.color(bg_level),
                Theme::Error.color(border_level),
                Theme::Error.color(shadow_level),
            ),
            Theme::Warning => (
                Theme::Warning.color(bg_level),
                Theme::Warning.color(border_level),
                Theme::Warning.color(shadow_level),
            ),
            Theme::Success => (
                Theme::Success.color(bg_level),
                Theme::Success.color(border_level),
                Theme::Success.color(shadow_level),
            ),
            Theme::Info => (
                Theme::Info.color(bg_level),
                Theme::Info.color(border_level),
                Theme::Info.color(shadow_level),
            ),
        };

        Self {
            theme,
            background_color: background_color.into(),
            border_color: border_color.into(),
            border_width: 1.0,
            border_radius: Radius::new(6.0),
            shadow_color: shadow_color.into(),
            spread_radius: 5.0,
            blur_radius: 5.0,
            shadow_offset: vec2(0.0, 0.0),
            background_visible: true,
            rotation: 0.0,
            scale: 1.0,
            padding: Padding { left: 6.0, top: 6.0, right: 6.0, bottom: 6.0 },
            margin: Margin { left: 6.0, top: 6.0, right: 6.0, bottom: 6.0 },
            clip_x: false,
            clip_y: false,
            align: Align::default(),
            cursor: MouseCursor::default(),
            flow: Flow::Down,
            spacing: 6.0,
        }
    }
}


impl Default for ViewBasicProp {
    fn default() -> Self {
        ViewBasicProp::new(Theme::Dark, ViewState::None)
    }
}
