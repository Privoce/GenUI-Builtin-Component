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
            BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, COLOR, CURSOR, DISABLED, FLOW, FONT_SIZE,
            HOVER, LINE_SPACING, MARGIN, PADDING, PRESSED, ROTATION, SCALE, SHADOW_COLOR,
            SHADOW_OFFSET, SPREAD_RADIUS, THEME, UNDERLINE_COLOR, UNDERLINE_VISIBLE,
            UNDERLINE_WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplyStateMapImpl, Radius,
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
};

prop_interconvert! {
    LinkProp {
        basic_prop = LinkBasicStyle;
        basic => BASIC, LinkBasicStyle::default(), |v| (v, LinkState::Basic).try_into(),
        hover => HOVER, LinkBasicStyle::from_state(Theme::default(), LinkState::Hover), |v| (v, LinkState::Hover).try_into(),
        pressed => PRESSED, LinkBasicStyle::from_state(Theme::default(), LinkState::Pressed), |v| (v, LinkState::Pressed).try_into(),
        disabled => DISABLED, LinkBasicStyle::from_state(Theme::default(), LinkState::Disabled), |v| (v, LinkState::Disabled).try_into()
    }, "[component.link] should be a table"
}

impl Style for LinkProp {
    type State = LinkState;
    type Basic = LinkBasicStyle;

    get_get_mut! {
        LinkState::Basic => basic,
        LinkState::Hover => hover,
        LinkState::Pressed => pressed,
        LinkState::Disabled => disabled
    }

    fn len() -> usize {
        4 * LinkBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            LinkState::Basic,
            [
                (LinkState::Hover, &mut self.hover),
                (LinkState::Pressed, &mut self.pressed),
                (LinkState::Disabled, &mut self.disabled),
            ],
        );
    }
}

basic_prop_interconvert! {
    LinkBasicStyle {
        state = LinkState;
        {
            color => COLOR, |v| v.try_into(),
            underline_color => UNDERLINE_COLOR, |v| v.try_into(),
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            border_color => BORDER_COLOR, |v| v.try_into(),
            shadow_color => SHADOW_COLOR, |v| v.try_into()
        };
        {
            font_size: f32 => FONT_SIZE, 10.0, |v| v.to_f32(),
            line_spacing: f32 => LINE_SPACING, 1.2, |v| v.to_f32(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            padding: Padding => PADDING, Padding::from_f64(0.0), |v| v.to_padding(padding),
            flow: Flow => FLOW, Flow::RightWrap, |v| v.to_flow(),
            underline_visible: bool => UNDERLINE_VISIBLE, true, |v| v.to_bool(),
            underline_width: f32 => UNDERLINE_WIDTH, 1.0, |v| v.to_f32(),
            border_width: f32 => BORDER_WIDTH, 1.0, |v| v.to_f32(),
            border_radius: Radius => BORDER_RADIUS, Radius::new(0.0), |v| v.try_into(),
            spread_radius: f32 => SPREAD_RADIUS, 0.0, |v| v.to_f32(),
            blur_radius: f32 => BLUR_RADIUS, 0.0, |v| v.to_f32(),
            shadow_offset: Vec2 => SHADOW_OFFSET, vec2(0.0, 0.0), |v| v.to_vec2(shadow_offset),
            background_visible: bool => BACKGROUND_VISIBLE, false, |v| v.to_bool(),
            rotation: f32 => ROTATION, 0.0, |v| v.to_f32(),
            scale: f32 => SCALE, 1.0, |v| v.to_f32(),
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v))),
            clip_x: bool => CLIP_X, false, |v| v.to_bool(),
            clip_y: bool => CLIP_Y, false, |v| v.to_bool(),
            align: Align => ALIGN, Align::default(), |v| v.to_align(Align::default())
        }
    }, "[component.link.$state] should be a inline table"
}

component_colors! {
    LinkColors {
        colors = (Color, Color, Color, Color, Color);
        color, underline_color, background_color, border_color, shadow_color
    }
}

impl LinkBasicStyle {
    getter_setter_prop! {
        get_theme, set_theme: theme -> Theme,
        get_color, set_color: color -> Vec4,
        get_font_size, set_font_size: font_size -> f32,
        get_line_spacing, set_line_spacing: line_spacing -> f32,
        get_margin, set_margin: margin -> Margin,
        get_padding, set_padding: padding -> Padding,
        get_flow, set_flow: flow -> Flow
    }
}

impl BasicStyle for LinkBasicStyle {
    type State = LinkState;
    type Colors = LinkColors;

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(colors.color.into());
            }
            FONT_SIZE => {
                self.font_size = f32::from_live_value(value).unwrap_or(12.0);
            }
            LINE_SPACING => {
                self.line_spacing = f32::from_live_value(value).unwrap_or(1.2);
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            PADDING => {
                self.padding = Padding::from_live_value(value).unwrap_or(Padding::from_f64(0.0));
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::RightWrap);
            }
            UNDERLINE_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.underline_color =
                    Vec4::from_live_color(value).unwrap_or(colors.underline_color.into());
            }
            UNDERLINE_VISIBLE => {
                self.underline_visible = bool::from_live_value(value).unwrap_or(true);
            }
            UNDERLINE_WIDTH => {
                self.underline_width = f32::from_live_value(value).unwrap_or(1.0);
            }
            BACKGROUND_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(colors.background_color.into());
            }
            BORDER_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.border_color =
                    Vec4::from_live_color(value).unwrap_or(colors.border_color.into());
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(0.0);
            }
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(0.0));
            }
            SHADOW_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.shadow_color =
                    Vec4::from_live_color(value).unwrap_or(colors.shadow_color.into());
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
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(false);
            }
            ROTATION => {
                self.rotation = f32::from_live_value(value).unwrap_or(0.0);
            }
            SCALE => {
                self.scale = f32::from_live_value(value).unwrap_or(1.0);
            }
            CLIP_X => {
                self.clip_x = bool::from_live_value(value).unwrap_or(false);
            }
            CLIP_Y => {
                self.clip_y = bool::from_live_value(value).unwrap_or(false);
            }
            ALIGN => {
                self.align = Align::from_live_value(value).unwrap_or(Align::default());
            }
            CURSOR => {
                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    MouseCursor::Hand
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let LinkColors {
            color,
            underline_color,
            background_color,
            border_color,
            shadow_color,
        } = Self::state_colors(self.theme, state);
        self.color = color.into();
        self.underline_color = underline_color.into();
        self.background_color = background_color.into();
        self.border_color = border_color.into();
        self.shadow_color = shadow_color.into();
    }

    fn len() -> usize {
        10
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let LinkColors {
            color,
            underline_color,
            background_color,
            border_color,
            shadow_color,
        } = Self::state_colors(theme, state);

        Self {
            theme,
            color: color.into(),
            underline_color: underline_color.into(),
            background_color: background_color.into(),
            border_color: border_color.into(),
            shadow_color: shadow_color.into(),
            underline_width: 1.0,
            underline_visible: true,
            font_size: 12.0,
            line_spacing: 1.2,
            margin: Margin::from_f64(0.0),
            padding: Padding::from_f64(0.0),
            flow: Flow::RightWrap,
            border_width: 0.0,
            border_radius: Radius::new(0.0),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            background_visible: false,
            rotation: 0.0,
            scale: 1.0,
            clip_x: false,
            clip_y: false,
            align: Align::default(),
            cursor: MouseCursor::Hand,
            abs_pos: None,
        }
    }

    state_colors! {
        (color_level, underline_level, background_level, border_level, shadow_level),
        LinkState::Basic => (300, 300, 500, 500, 400),
        LinkState::Hover => (200, 200, 400, 400, 300),
        LinkState::Pressed => (400, 400, 600, 600, 500),
        LinkState::Disabled => (100, 100, 300, 300, 200)
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(color), None.into()),
            (live_id!(font_size), None.into()),
            (live_id!(line_spacing), None.into()),
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
            (live_id!(flow), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(border_color), None.into()),
            (live_id!(border_width), None.into()),
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
            (live_id!(shadow_color), None.into()),
            (live_id!(spread_radius), None.into()),
            (live_id!(blur_radius), None.into()),
            (live_id!(shadow_offset), None.into()),
            (live_id!(background_visible), None.into()),
            (live_id!(rotation), None.into()),
            (live_id!(scale), None.into()),
            (live_id!(clip_x), None.into()),
            (live_id!(clip_y), None.into()),
            (live_id!(align), Some(vec![live_id!(x), live_id!(y)]).into()),
            (live_id!(cursor), None.into()),
            (live_id!(abs_pos), None.into()),
            (live_id!(underline_visible), None.into()),
            (live_id!(underline_width), None.into()),
            (live_id!(underline_color), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            margin: self.margin,
            height: Size::Fit,
            width: Size::Fit,
            abs_pos: self.abs_pos,
        }
    }

    fn layout(&self) -> Layout {
        Layout {
            padding: self.padding,
            flow: self.flow,
            clip_x: self.clip_x,
            clip_y: self.clip_y,
            align: self.align,
            ..Default::default()
        }
    }
}

impl From<&LinkBasicStyle> for ViewBasicStyle {
    fn from(value: &LinkBasicStyle) -> Self {
        let LinkBasicStyle {
            theme,
            margin,
            padding,
            flow,
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
            clip_x,
            clip_y,
            align,
            cursor,
            abs_pos,
            ..
        } = *value;
        ViewBasicStyle {
            theme,
            margin,
            padding,
            flow,
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
            clip_x,
            clip_y,
            align,
            cursor,
            abs_pos,
            ..Default::default()
        }
    }
}

component_state! {
    LinkState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    },
    _ => LinkState::Basic
}

impl ComponentState for LinkState {
    fn is_disabled(&self) -> bool {
        matches!(self, LinkState::Disabled)
    }
}

impl From<LinkState> for ViewState {
    fn from(value: LinkState) -> Self {
        match value {
            LinkState::Basic => ViewState::Basic,
            LinkState::Hover => ViewState::Hover,
            LinkState::Pressed => ViewState::Pressed,
            LinkState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for LinkState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => LinkState::Basic,
            ViewState::Hover => LinkState::Hover,
            ViewState::Pressed => LinkState::Pressed,
            ViewState::Disabled => LinkState::Disabled,
        }
    }
}
