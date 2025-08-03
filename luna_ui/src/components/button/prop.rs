use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Prop},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    getter_setter_prop,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CURSOR, DISABLED, FLOW, HEIGHT, HOVER, MARGIN, PADDING,
            PRESSED, SHADOW_COLOR, SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl, Radius,
    },
    themes::{Color, Theme, TomlValueTo},
    try_from_toml_item,
    utils::get_from_itable,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ButtonProp {
    #[live(ButtonBasicProp::default())]
    pub basic: ButtonBasicProp,
    #[live(ButtonBasicProp::from_state(Theme::default(), ButtonState::Hover))]
    pub hover: ButtonBasicProp,
    #[live(ButtonBasicProp::from_state(Theme::default(), ButtonState::Pressed))]
    pub pressed: ButtonBasicProp,
    #[live(ButtonBasicProp::from_state(Theme::default(), ButtonState::Disabled))]
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

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            ButtonState::Basic => &mut self.basic,
            ButtonState::Hover => &mut self.hover,
            ButtonState::Pressed => &mut self.pressed,
            ButtonState::Disabled => &mut self.disabled,
        }
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            ButtonState::Basic,
            [
                (ButtonState::Hover, &mut self.hover),
                (ButtonState::Pressed, &mut self.pressed),
                (ButtonState::Disabled, &mut self.disabled),
            ],
        );
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

try_from_toml_item! {
    ButtonProp {
        basic => BASIC, ButtonBasicProp::default(),|v| (v, ButtonState::Basic).try_into(),
        hover => HOVER, ButtonBasicProp::from_state(Theme::default(), ButtonState::Hover),|v| (v, ButtonState::Hover).try_into(),
        pressed => PRESSED, ButtonBasicProp::from_state(Theme::default(), ButtonState::Pressed),|v| (v, ButtonState::Pressed).try_into(),
        disabled => DISABLED, ButtonBasicProp::from_state(Theme::default(), ButtonState::Disabled),|v| (v, ButtonState::Disabled).try_into()
    }, "[component.button] should be a table"
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ButtonBasicProp {
    #[live]
    pub theme: Theme,
    // --- background ----------------
    #[live]
    pub background_color: Vec4,
    #[live(true)]
    pub background_visible: bool,
    // --- shadow -------------------
    #[live]
    pub shadow_color: Vec4,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(0.0)]
    pub blur_radius: f32,
    #[live(vec2(0.0, 0.0))]
    pub shadow_offset: Vec2,
    // --- border -------------------
    #[live(0.0)]
    pub border_width: f32,
    #[live]
    pub border_color: Vec4,
    #[live(Radius::new(4.0))]
    pub border_radius: Radius,
    // --- cursor -------------------
    #[live(MouseCursor::Hand)]
    pub cursor: MouseCursor,
    #[live(Margin::from_f64(6.0))]
    pub margin: Margin,
    #[live(Padding::from_xy(10.0, 16.0))]
    pub padding: Padding,
    #[live(Flow::Right)]
    pub flow: Flow,
    #[live(Align::from_f64(0.5))]
    pub align: Align,
    #[live(Size::Fit)]
    pub height: Size,
    #[live(Size::Fit)]
    pub width: Size,
    #[live(6.0)]
    pub spacing: f64,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
}

impl From<&ButtonBasicProp> for ViewBasicProp {
    fn from(value: &ButtonBasicProp) -> Self {
        let ButtonBasicProp {
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
            abs_pos,
        } = *value;

        ViewBasicProp {
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
            rotation: 0.0,
            scale: 1.0,
            padding,
            margin,
            clip_x: false,
            clip_y: false,
            align,
            cursor,
            flow,
            spacing,
            height,
            width,
            abs_pos,
        }
    }
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
        19
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            BACKGROUND_COLOR => {
                let (background_color, _, _) = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(background_color.into());
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            SHADOW_COLOR => {
                let (_, _, shadow_color) = Self::state_colors(self.theme, state);
                self.shadow_color = Vec4::from_live_color(value).unwrap_or(shadow_color.into());
            }
            SPREAD_RADIUS => {
                self.spread_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            BLUR_RADIUS => {
                self.blur_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            SHADOW_OFFSET => {
                self.shadow_offset = Vec2::from_live_value(value).unwrap_or(vec2(0.0, 0.0));
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(0.0);
            }
            BORDER_COLOR => {
                let (_, border_color, _) = Self::state_colors(self.theme, state);
                self.border_color = Vec4::from_live_color(value).unwrap_or(border_color.into());
            }
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(4.0));
            }
            CURSOR => {
                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    MouseCursor::Hand
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(6.0));
            }
            PADDING => {
                self.padding =
                    Padding::from_live_value(value).unwrap_or(Padding::from_xy(10.0, 16.0));
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::Right);
            }
            ALIGN => {
                self.align = Align::from_live_value(value).unwrap_or(Align::from_f64(0.5));
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            SPACING => {
                self.spacing = f64::from_live_value(value).unwrap_or(6.0);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let (background_color, border_color, shadow_color) = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.border_color = border_color.into();
        self.shadow_color = shadow_color.into();
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
            abs_pos: None,
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        let (bg_level, border_level, shadow_level) = match state {
            ButtonState::Basic => (500, 500, 400),
            ButtonState::Hover => (400, 400, 300),
            ButtonState::Pressed => (600, 600, 500),
            ButtonState::Disabled => (300, 300, 200),
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

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(border_color), None.into()),
            (
                live_id!(border_radius),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (live_id!(border_width), None.into()),
            (live_id!(shadow_color), None.into()),
            (live_id!(spread_radius), None.into()),
            (live_id!(blur_radius), None.into()),
            (live_id!(shadow_offset), None.into()),
            (live_id!(background_visible), None.into()),
            (live_id!(cursor), None.into()),
            (live_id!(width), None.into()),
            (live_id!(height), None.into()),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (
                live_id!(padding),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (live_id!(align), Some(vec![live_id!(x), live_id!(y)]).into()),
            (live_id!(flow), None.into()),
            (live_id!(spacing), None.into()),
            (live_id!(abs_pos), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            abs_pos: self.abs_pos,
            margin: self.margin,
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self) -> Layout {
        Layout {
            clip_x: false,
            clip_y: false,
            padding: self.padding,
            align: self.align,
            flow: self.flow,
            spacing: self.spacing,
            ..Default::default()
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
        let abs_pos = get_from_itable(
            inline_table,
            ABS_POS,
            || Ok(None),
            |v| v.to_dvec2().map(Some),
        )?;

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
            abs_pos,
        })
    }
}

impl ButtonBasicProp {
    getter_setter_prop! {
        get_theme, set_theme: theme -> Theme,
        get_background_color, set_background_color: background_color -> Vec4,
        get_background_visible, set_background_visible: background_visible -> bool,
        get_shadow_color, set_shadow_color: shadow_color -> Vec4,
        get_spread_radius, set_spread_radius: spread_radius -> f32,
        get_blur_radius, set_blur_radius: blur_radius -> f32,
        get_shadow_offset, set_shadow_offset: shadow_offset -> Vec2,
        get_border_width, set_border_width: border_width -> f32,
        get_border_color, set_border_color: border_color -> Vec4,
        get_border_radius, set_border_radius: border_radius -> Radius,
        get_cursor, set_cursor: cursor -> MouseCursor,
        get_margin, set_margin: margin -> Margin,
        get_padding, set_padding: padding -> Padding,
        get_flow, set_flow: flow -> Flow,
        get_align, set_align: align -> Align,
        get_height, set_height: height -> Size,
        get_width, set_width: width -> Size,
        get_spacing, set_spacing: spacing -> f64,
        get_abs_pos, set_abs_pos: abs_pos -> Option<DVec2>
    }
}

component_state! {
    ButtonState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    },
    _ => ButtonState::Basic
}

impl From<ViewState> for ButtonState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => ButtonState::Basic,
            ViewState::Hover => ButtonState::Hover,
            ViewState::Pressed => ButtonState::Pressed,
            ViewState::Disabled => ButtonState::Disabled,
        }
    }
}

impl From<ButtonState> for ViewState {
    fn from(value: ButtonState) -> Self {
        match value {
            ButtonState::Basic => ViewState::Basic,
            ButtonState::Hover => ViewState::Hover,
            ButtonState::Pressed => ViewState::Pressed,
            ButtonState::Disabled => ViewState::Disabled,
        }
    }
}

impl ComponentState for ButtonState {
    fn is_disabled(&self) -> bool {
        matches!(self, ButtonState::Disabled)
    }
}
