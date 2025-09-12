use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_colors, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
        view::{ViewBasicStyle, ViewState},
    },
    error::Error,
    get_get_mut, getter_setter_prop,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CURSOR, DISABLED, FLOW, HEIGHT, HOVER, MARGIN, PADDING,
            PRESSED, SHADOW_COLOR, SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToTomlValue, ToColor},
        ApplyStateMapImpl, Radius,
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
};

prop_interconvert! {
    ButtonStyle {
        basic_prop = ButtonBasicStyle;
        basic => BASIC, ButtonBasicStyle::default(),|v| (v, ButtonState::Basic).try_into(),
        hover => HOVER, ButtonBasicStyle::from_state(Theme::default(), ButtonState::Hover),|v| (v, ButtonState::Hover).try_into(),
        pressed => PRESSED, ButtonBasicStyle::from_state(Theme::default(), ButtonState::Pressed),|v| (v, ButtonState::Pressed).try_into(),
        disabled => DISABLED, ButtonBasicStyle::from_state(Theme::default(), ButtonState::Disabled),|v| (v, ButtonState::Disabled).try_into()
    }, "[component.button] should be a table"
}

impl Style for ButtonStyle {
    type State = ButtonState;

    type Basic = ButtonBasicStyle;

    fn len() -> usize {
        ButtonBasicStyle::len() * 4 // basic, hover, pressed, disabled
    }

    get_get_mut! {
        ButtonState::Basic => basic,
        ButtonState::Hover => hover,
        ButtonState::Pressed => pressed,
        ButtonState::Disabled => disabled
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

basic_prop_interconvert! {
    ButtonBasicStyle {
        state = ButtonState;
        {
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            shadow_color => SHADOW_COLOR, |v| v.try_into(),
            border_color => BORDER_COLOR, |v| v.try_into()
        };
        {
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            spread_radius: f32 => SPREAD_RADIUS, 0.0, |v| v.to_f32(),
            blur_radius: f32 => BLUR_RADIUS, 0.0, |v| v.to_f32(),
            shadow_offset: Vec2 => SHADOW_OFFSET, vec2(0.0, 0.0), |v| v.to_vec2(shadow_offset),
            border_width: f32 => BORDER_WIDTH, 0.0, |v| v.to_f32(),
            border_radius: Radius => BORDER_RADIUS, Radius::new(4.0), |v| v.try_into(),
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            padding: Padding => PADDING, Padding::from_xy(10.0, 16.0), |v| v.to_padding(padding),
            flow: Flow => FLOW, Flow::Right, |v| v.to_flow(),
            align: Align => ALIGN, Align::from_f64(0.5), |v| v.to_align(align),
            height: Size => HEIGHT, Size::Fit, |v| v.to_size(),
            width: Size => WIDTH, Size::Fit, |v| v.to_size(),
            spacing: f64 => SPACING, 6.0, |v| v.to_f64(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "ButtonBasicStyle should be a inline table"
}

impl From<&ButtonBasicStyle> for ViewBasicStyle {
    fn from(value: &ButtonBasicStyle) -> Self {
        let ButtonBasicStyle {
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

        ViewBasicStyle {
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

component_colors! {
    ButtonColors {
        colors = (Color, Color, Color);
        background_color, border_color, shadow_color
    }
}

impl BasicStyle for ButtonBasicStyle {
    type State = ButtonState;

    type Colors = ButtonColors;

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
                let color = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(color.background_color.into());
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            SHADOW_COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.shadow_color =
                    Vec4::from_live_color(value).unwrap_or(color.shadow_color.into());
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
                let color = Self::state_colors(self.theme, state);
                self.border_color =
                    Vec4::from_live_color(value).unwrap_or(color.border_color.into());
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
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
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
        let ButtonColors {
            background_color,
            border_color,
            shadow_color,
        } = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.border_color = border_color.into();
        self.shadow_color = shadow_color.into();
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let ButtonColors {
            background_color,
            border_color,
            shadow_color,
        } = Self::state_colors(theme, state);

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
            margin: Margin::from_f64(0.0),
            padding: Padding::from_xy(10.0, 16.0),
            flow: Flow::Right,
            align: Align::from_f64(0.5),
            height: Size::Fit,
            width: Size::Fit,
            spacing: 6.0,
            abs_pos: None,
        }
    }

    state_colors! {
        (bg_level, border_level, shadow_level),
        ButtonState::Basic => (500, 500, 400),
        ButtonState::Hover => (400, 400, 300),
        ButtonState::Pressed => (600, 600, 500),
        ButtonState::Disabled => (300, 300, 200)
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

impl ButtonBasicStyle {
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
