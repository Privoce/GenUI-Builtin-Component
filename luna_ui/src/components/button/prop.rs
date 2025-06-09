use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    components::{
        traits::{BasicProp, Prop},
        view::ViewState,
    },
    error::Error,
    prop::{
        manuel::{
            ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CURSOR, DISABLED, FLOW, HEIGHT, HOVER, MARGIN, PADDING,
            PRESSED, SHADOW_COLOR, SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::NewFrom,
        Radius,
    },
    themes::{Color, Theme, TomlValueTo},
    utils::{get_from_itable, get_from_table},
};

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

impl Prop for ButtonProp {
    type State = ButtonState;

    type Basic = ButtonBasicProp;

    fn len() -> usize {
        ButtonBasicProp::len() * 4 // basic, hover, pressed, disabled
    }

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            ButtonState::Basic => &self.basic,
            ButtonState::Hover => &self.hover,
            ButtonState::Pressed => &self.pressed,
            ButtonState::Disabled => &self.disabled,
        }
    }
}

impl Default for ButtonProp {
    fn default() -> Self {
        Self {
            basic: ButtonBasicProp::default(),
            hover: ButtonBasicProp::from_state(Theme::default(), ButtonState::Hover),
            pressed: ButtonBasicProp::from_state(Theme::default(), ButtonState::Pressed),
            disabled: ButtonBasicProp::from_state(Theme::default(), ButtonState::Disabled),
        }
    }
}

impl TryFrom<&Item> for ButtonProp {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[component.button] should be a table".to_string(),
        ))?;

        let basic = get_from_table(
            table,
            BASIC,
            || Ok(ButtonBasicProp::default()),
            |v| (v, ButtonState::Basic).try_into(),
        )?;

        let hover = get_from_table(
            table,
            HOVER,
            || {
                Ok(ButtonBasicProp::from_state(
                    Theme::default(),
                    ButtonState::Hover,
                ))
            },
            |v| (v, ButtonState::Hover).try_into(),
        )?;

        let pressed = get_from_table(
            table,
            PRESSED,
            || {
                Ok(ButtonBasicProp::from_state(
                    Theme::default(),
                    ButtonState::Pressed,
                ))
            },
            |v| (v, ButtonState::Pressed).try_into(),
        )?;

        let disabled = get_from_table(
            table,
            DISABLED,
            || {
                Ok(ButtonBasicProp::from_state(
                    Theme::default(),
                    ButtonState::Disabled,
                ))
            },
            |v| (v, ButtonState::Disabled).try_into(),
        )?;

        Ok(Self {
            basic,
            hover,
            pressed,
            disabled,
        })
    }
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
    #[live]
    pub margin: Margin,
    #[live]
    pub padding: Padding,
    #[live]
    pub flow: Flow,
    #[live]
    pub align: Align,
    #[live]
    pub height: Size,
    #[live]
    pub width: Size,
    #[live]
    pub spacing: f64,
}

impl Default for ButtonBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), ButtonState::Basic)
    }
}

impl BasicProp for ButtonBasicProp {
    type State = ButtonState;

    type Colors = (Color, Color, Color);

    fn len() -> usize {
        18
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let (background_color, border_color, shadow_color) = Self::state_colors(theme, state);

        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };

        Self {
            theme,
            background_color: background_color.into(),
            background_visible: true,
            shadow_color: shadow_color.into(),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            border_width: 0.0,
            border_color: border_color.into(),
            border_radius: Radius::new(4.0),
            cursor,
            margin: Margin::from_f64(6.0),
            padding: Padding::from_xy(10.0, 16.0),
            flow: Flow::Right,
            align: Align::from_f64(0.5),
            height: Size::Fit,
            width: Size::Fit,
            spacing: 6.0,
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        let (bg_level, border_level, shadow_level) = match state {
            ButtonState::Basic => (400, 400, 300),
            ButtonState::Hover => (300, 300, 200),
            ButtonState::Pressed => (500, 500, 400),
            ButtonState::Disabled => (600, 600, 500),
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

impl TryFrom<(&Item, ButtonState)> for ButtonBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, ButtonState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.button.$state] should be an inline table".to_string(),
        ))?;

        let theme = Theme::default();
        let theme = get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;
        let (background_color, border_color, shadow_color) = Self::state_colors(theme, state);

        let background_color = get_from_itable(
            inline_table,
            BACKGROUND_COLOR,
            || Ok(background_color),
            |v| v.try_into(),
        )?
        .into();

        let background_visible = get_from_itable(
            inline_table,
            BACKGROUND_VISIBLE,
            || Ok(true),
            |v| v.to_bool(),
        )?;
        let shadow_color = get_from_itable(
            inline_table,
            SHADOW_COLOR,
            || Ok(shadow_color),
            |v| v.try_into(),
        )?
        .into();

        let spread_radius =
            get_from_itable(inline_table, SPREAD_RADIUS, || Ok(0.0), |v| v.to_f32())?;
        let blur_radius = get_from_itable(inline_table, BLUR_RADIUS, || Ok(0.0), |v| v.to_f32())?;
        let shadow_offset = vec2(0.0, 0.0);
        let shadow_offset = get_from_itable(
            inline_table,
            SHADOW_OFFSET,
            || Ok(shadow_offset),
            |v| v.to_vec2(shadow_offset),
        )?;

        let border_width = get_from_itable(inline_table, BORDER_WIDTH, || Ok(0.0), |v| v.to_f32())?;
        let border_color = get_from_itable(
            inline_table,
            BORDER_COLOR,
            || Ok(border_color),
            |v| v.try_into(),
        )?
        .into();
        let border_radius = get_from_itable(
            inline_table,
            BORDER_RADIUS,
            || Ok(Radius::new(6.0)),
            |v| v.try_into(),
        )?;
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };

        let cursor = get_from_itable(inline_table, CURSOR, || Ok(cursor), |v| v.to_cursor())?;
        let margin = Margin::from_f64(6.0);
        let margin = get_from_itable(inline_table, MARGIN, || Ok(margin), |v| v.to_margin(margin))?;
        let padding = Padding::from_xy(10.0, 16.0);
        let padding = get_from_itable(
            inline_table,
            PADDING,
            || Ok(padding),
            |v| v.to_padding(padding),
        )?;
        let flow = get_from_itable(inline_table, FLOW, || Ok(Flow::Right), |v| v.to_flow())?;
        let align = Align::from_f64(0.5);
        let align = get_from_itable(inline_table, ALIGN, || Ok(align), |v| v.to_align(align))?;
        let height = get_from_itable(inline_table, HEIGHT, || Ok(Size::Fit), |v| v.to_size())?;
        let width = get_from_itable(inline_table, WIDTH, || Ok(Size::Fit), |v| v.to_size())?;
        let spacing = get_from_itable(inline_table, SPACING, || Ok(6.0), |v| v.to_f64())?;

        Ok(Self {
            theme,
            background_color,
            background_visible,
            shadow_color,
            spread_radius,
            blur_radius,
            shadow_offset,
            border_width,
            border_color,
            border_radius,
            cursor,
            margin,
            padding,
            flow,
            align,
            height,
            width,
            spacing,
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonState {
    #[default]
    Basic,
    Hover,
    Pressed,
    Disabled,
}

impl ButtonState {
    pub fn is_disabled(&self) -> bool {
        matches!(self, ButtonState::Disabled)
    }
}

impl From<ViewState> for ButtonState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::None => ButtonState::Basic,
            ViewState::Hover => ButtonState::Hover,
            ViewState::Pressed => ButtonState::Pressed,
        }
    }
}
