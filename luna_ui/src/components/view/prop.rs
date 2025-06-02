use makepad_widgets::*;
use toml_edit::{Item, Value};

use crate::{
    error::Error,
    styles::Radius,
    themes::{Color, Theme, TomlValueTo},
    utils::{get_from_itable, get_from_table},
};

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
    #[live]
    pub height: Size,
    #[live]
    pub width: Size,
}

impl ViewBasicProp {
    pub fn new(theme: Theme, state: ViewState) -> Self {
        let (background_color, border_color, shadow_color) = Self::state_color(theme, state);

        Self {
            theme,
            background_color: background_color.into(),
            border_color: border_color.into(),
            border_width: 1.0,
            border_radius: Radius::new(6.0),
            shadow_color: shadow_color.into(),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            background_visible: true,
            rotation: 0.0,
            scale: 1.0,
            padding: Padding {
                left: 6.0,
                top: 6.0,
                right: 6.0,
                bottom: 6.0,
            },
            margin: Margin {
                left: 6.0,
                top: 6.0,
                right: 6.0,
                bottom: 6.0,
            },
            clip_x: false,
            clip_y: false,
            align: Align::default(),
            cursor: MouseCursor::default(),
            flow: Flow::Down,
            spacing: 6.0,
            height: Size::Fill,
            width: Size::Fill,
        }
    }

    pub fn state_color(theme: Theme, state: ViewState) -> (Color, Color, Color) {
        let (bg_level, border_level, shadow_level) = match state {
            ViewState::None => (400, 400, 300),
            ViewState::Hover => (300, 300, 200),
            ViewState::Pressed => (500, 500, 400),
        };

        match theme {
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
        }
    }
}

impl Default for ViewBasicProp {
    fn default() -> Self {
        ViewBasicProp::new(Theme::Dark, ViewState::None)
    }
}

impl TryFrom<&Item> for ViewProp {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[components.view] should be a table".to_string(),
        ))?;

        let basic = get_from_table(
            table,
            "basic",
            || Ok(ViewBasicProp::default()),
            |item| (item, ViewState::None).try_into(),
        )?;

        let hover = get_from_table(
            table,
            "hover",
            || Ok(ViewBasicProp::new(Theme::Dark, ViewState::Hover)),
            |item| (item, ViewState::Hover).try_into(),
        )?;

        let pressed = get_from_table(
            table,
            "pressed",
            || Ok(ViewBasicProp::new(Theme::Dark, ViewState::Pressed)),
            |item| (item, ViewState::Pressed).try_into(),
        )?;

        Ok(ViewProp {
            basic,
            hover,
            pressed,
        })
    }
}

impl TryFrom<(&Item, ViewState)> for ViewBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, ViewState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.view.$state] should be an inline table".to_string(),
        ))?;
        let theme = Theme::default();
        let (background_color, border_color, shadow_color) = Self::state_color(theme, state);

        let theme = get_from_itable(inline_table, "theme", || Ok(theme), |v| v.try_into())?;

        let background_color = get_from_itable(
            inline_table,
            "background_color",
            || Ok(background_color),
            |v| v.try_into(),
        )?
        .into();

        let border_color = get_from_itable(
            inline_table,
            "border_color",
            || Ok(border_color),
            |v| v.try_into(),
        )?
        .into();

        let border_width =
            get_from_itable(inline_table, "border_width", || Ok(1.0), |v| v.to_f32())?;

        let border_radius = get_from_itable(
            inline_table,
            "border_radius",
            || Ok(Radius::new(6.0)),
            |v| v.try_into(),
        )?;

        let shadow_color = get_from_itable(
            inline_table,
            "shadow_color",
            || Ok(shadow_color),
            |v| v.try_into(),
        )?
        .into();

        let spread_radius =
            get_from_itable(inline_table, "spread_radius", || Ok(0.0), |v| v.to_f32())?;

        let blur_radius = get_from_itable(inline_table, "blur_radius", || Ok(0.0), |v| v.to_f32())?;

        let shadow_offset = get_from_itable(
            inline_table,
            "shadow_offset",
            || Ok(vec2(0.0, 0.0)),
            |v| v.to_vec2(vec2(0.0, 0.0)),
        )?;

        let background_visible = get_from_itable(
            inline_table,
            "background_visible",
            || Ok(true),
            |v| v.to_bool(),
        )?;

        let rotation = get_from_itable(inline_table, "rotation", || Ok(0.0), |v| v.to_f32())?;

        let scale = get_from_itable(inline_table, "scale", || Ok(1.0), |v| v.to_f32())?;

        let padding = get_from_itable(
            inline_table,
            "padding",
            || {
                Ok(Padding {
                    left: 6.0,
                    top: 6.0,
                    right: 6.0,
                    bottom: 6.0,
                })
            },
            |v| {
                v.to_padding(Padding {
                    left: 6.0,
                    top: 6.0,
                    right: 6.0,
                    bottom: 6.0,
                })
            },
        )?;

        let margin = get_from_itable(
            inline_table,
            "margin",
            || {
                Ok(Margin {
                    left: 6.0,
                    top: 6.0,
                    right: 6.0,
                    bottom: 6.0,
                })
            },
            |v| {
                v.to_margin(Margin {
                    left: 6.0,
                    top: 6.0,
                    right: 6.0,
                    bottom: 6.0,
                })
            },
        )?;
        let clip_x = get_from_itable(inline_table, "clip_x", || Ok(false), |v| v.to_bool())?;
        let clip_y = get_from_itable(inline_table, "clip_y", || Ok(false), |v| v.to_bool())?;

        let align = get_from_itable(
            inline_table,
            "align",
            || Ok(Align::default()),
            |v| v.to_align(Align::default()),
        )?;
        let cursor = get_from_itable(
            inline_table,
            "cursor",
            || Ok(MouseCursor::default()),
            |v| v.to_cursor(),
        )?;
        let flow = get_from_itable(inline_table, "flow", || Ok(Flow::Down), |v| v.to_flow())?;
        let spacing = get_from_itable(inline_table, "spacing", || Ok(6.0), |v| v.to_f64())?;
        let height = get_from_itable(
            inline_table,
            "height",
            || Ok(Size::Fill),
            |v| v.to_size(),
        )?;
        let width = get_from_itable(
            inline_table,
            "width",
            || Ok(Size::Fill),
            |v| v.to_size(),
        )?;

        Ok(Self {
            theme,
            background_color,
            border_color,
            border_width,
            border_radius,
            shadow_color,
            spread_radius,
            blur_radius,
            shadow_offset,
            background_visible,
            rotation,
            scale,
            padding,
            margin,
            clip_x,
            clip_y,
            align,
            cursor,
            flow,
            spacing,
            height,
            width,
        })
    }
}
