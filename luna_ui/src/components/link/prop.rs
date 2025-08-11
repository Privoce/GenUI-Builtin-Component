use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Prop},
        view::{ViewBasicProp, ViewState},
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
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl, Radius,
    },
    state_colors,
    themes::{Color, Theme, TomlValueTo},
    try_from_toml_item,
    utils::get_from_itable as get,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct LinkProp {
    #[live(LinkBasicProp::default())]
    pub basic: LinkBasicProp,
    #[live(LinkBasicProp::from_state(Theme::default(), LinkState::Hover))]
    pub hover: LinkBasicProp,
    #[live(LinkBasicProp::from_state(Theme::default(), LinkState::Pressed))]
    pub pressed: LinkBasicProp,
    #[live(LinkBasicProp::from_state(Theme::default(), LinkState::Disabled))]
    pub disabled: LinkBasicProp,
}

impl Default for LinkProp {
    fn default() -> Self {
        Self {
            basic: LinkBasicProp::default(),
            hover: LinkBasicProp::from_state(Theme::default(), LinkState::Hover),
            pressed: LinkBasicProp::from_state(Theme::default(), LinkState::Pressed),
            disabled: LinkBasicProp::from_state(Theme::default(), LinkState::Disabled),
        }
    }
}

try_from_toml_item! {
    LinkProp {
        basic => BASIC, LinkBasicProp::default(), |v| (v, LinkState::Basic).try_into(),
        hover => HOVER, LinkBasicProp::from_state(Theme::default(), LinkState::Hover), |v| (v, LinkState::Hover).try_into(),
        pressed => PRESSED, LinkBasicProp::from_state(Theme::default(), LinkState::Pressed), |v| (v, LinkState::Pressed).try_into(),
        disabled => DISABLED, LinkBasicProp::from_state(Theme::default(), LinkState::Disabled), |v| (v, LinkState::Disabled).try_into()
    }, "[component.link] should be a table"
}

impl Prop for LinkProp {
    type State = LinkState;
    type Basic = LinkBasicProp;

    get_get_mut! {
        LinkState::Basic => basic,
        LinkState::Hover => hover,
        LinkState::Pressed => pressed,
        LinkState::Disabled => disabled
    }

    fn len() -> usize {
        4 * LinkBasicProp::len()
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

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct LinkBasicProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub color: Vec4,
    #[live(12.0)]
    pub font_size: f32,
    #[live(1.2)]
    pub line_spacing: f32,
    #[live(Margin::from_f64(0.0))]
    pub margin: Margin,
    #[live(Padding::from_f64(0.0))]
    pub padding: Padding,
    #[live(true)]
    pub underline_visible: bool,
    #[live]
    pub underline_color: Vec4,
    #[live(1.0)]
    pub underline_width: f32,
    #[live(Flow::RightWrap)]
    pub flow: Flow,
    #[live]
    pub background_color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live(0.0)]
    pub border_width: f32,
    #[live(Radius::new(0.0))]
    pub border_radius: Radius,
    #[live]
    pub shadow_color: Vec4,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(0.0)]
    pub blur_radius: f32,
    #[live(vec2(0.0, 0.0))]
    pub shadow_offset: Vec2,
    #[live(false)]
    pub background_visible: bool,
    #[live(0.0)]
    pub rotation: f32,
    #[live(1.0)]
    pub scale: f32,
    #[live(false)]
    pub clip_x: bool,
    #[live(false)]
    pub clip_y: bool,
    #[live(Align::default())]
    pub align: Align,
    #[live(MouseCursor::default())]
    pub cursor: MouseCursor,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
}

impl Default for LinkBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), LinkState::Basic)
    }
}

impl LinkBasicProp {
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

impl BasicProp for LinkBasicProp {
    type State = LinkState;
    type Colors = (Color, Color, Color, Color, Color);

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            COLOR => {
                let (color, _, _, _, _) = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.into());
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
                let (_, underline_color, _, _, _) = Self::state_colors(self.theme, state);
                self.underline_color =
                    Vec4::from_live_color(value).unwrap_or(underline_color.into());
            }
            UNDERLINE_VISIBLE => {
                self.underline_visible = bool::from_live_value(value).unwrap_or(true);
            }
            UNDERLINE_WIDTH => {
                self.underline_width = f32::from_live_value(value).unwrap_or(1.0);
            }
            BACKGROUND_COLOR => {
                let (_, _, background_color, _, _) = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(background_color.into());
            }
            BORDER_COLOR => {
                let (_, _, _, border_color, _) = Self::state_colors(self.theme, state);
                self.border_color = Vec4::from_live_color(value).unwrap_or(border_color.into());
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(0.0);
            }
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(0.0));
            }
            SHADOW_COLOR => {
                let (_, _, _, _, shadow_color) = Self::state_colors(self.theme, state);
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
        let (color, underline_color, background_color, border_color, shadow_color) =
            Self::state_colors(self.theme, state);
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
        let (color, underline_color, background_color, border_color, shadow_color) =
            Self::state_colors(theme, state);

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
        LinkState::Basic => (400, 400, 500, 500, 400),
        LinkState::Hover => (300, 300, 400, 400, 300),
        LinkState::Pressed => (500, 500, 600, 600, 500),
        LinkState::Disabled => (200, 200, 300, 300, 200)
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
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            margin: self.margin,
            height: Size::Fit,
            width: Size::Fit,
            abs_pos: self.abs_pos,
        }
        .with_add_padding(self.padding)
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

impl TryFrom<(&Item, LinkState)> for LinkBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, LinkState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "LinkProp should be a inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Value, LinkState)> for LinkBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Value, LinkState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "LinkProp should be a inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&InlineTable, LinkState)> for LinkBasicProp {
    type Error = Error;

    fn try_from((inline_table, state): (&InlineTable, LinkState)) -> Result<Self, Self::Error> {
        let theme = Theme::default();
        let theme = get(inline_table, THEME, || Ok(theme), |value| value.try_into())?;
        let (color, underline_color, background_color, border_color, shadow_color) =
            Self::state_colors(theme, state);
        let color = get(inline_table, COLOR, || Ok(color), |value| value.try_into())?.into();
        let font_size = get(inline_table, FONT_SIZE, || Ok(10.0), |item| item.to_f32())?;
        let line_spacing = get(inline_table, LINE_SPACING, || Ok(1.2), |item| item.to_f32())?;

        let default_margin = Margin::from_f64(0.0);

        let margin = get(
            inline_table,
            MARGIN,
            || Ok(default_margin),
            |item| item.to_margin(default_margin),
        )?;

        let default_padding = Padding::from_f64(0.0);

        let padding = get(
            inline_table,
            PADDING,
            || Ok(default_padding),
            |item| item.to_padding(default_padding),
        )?;

        let flow = get(
            inline_table,
            FLOW,
            || Ok(Flow::RightWrap),
            |item| item.to_flow(),
        )?;

        let underline_visible = get(
            inline_table,
            UNDERLINE_VISIBLE,
            || Ok(true),
            |item| item.to_bool(),
        )?;

        let underline_width = get(
            inline_table,
            UNDERLINE_WIDTH,
            || Ok(1.0),
            |item| item.to_f32(),
        )?;

        let underline_color = get(
            inline_table,
            UNDERLINE_COLOR,
            || Ok(underline_color),
            |value| value.try_into(),
        )?
        .into();

        let background_color = get(
            inline_table,
            BACKGROUND_COLOR,
            || Ok(background_color),
            |v| v.try_into(),
        )?
        .into();

        let border_color = get(
            inline_table,
            BORDER_COLOR,
            || Ok(border_color),
            |v| v.try_into(),
        )?
        .into();

        let border_width = get(inline_table, BORDER_WIDTH, || Ok(1.0), |v| v.to_f32())?;

        let border_radius = get(
            inline_table,
            BORDER_RADIUS,
            || Ok(Radius::new(0.0)),
            |v| v.try_into(),
        )?;

        let shadow_color = get(
            inline_table,
            SHADOW_COLOR,
            || Ok(shadow_color),
            |v| v.try_into(),
        )?
        .into();

        let spread_radius = get(inline_table, SPREAD_RADIUS, || Ok(0.0), |v| v.to_f32())?;

        let blur_radius = get(inline_table, BLUR_RADIUS, || Ok(0.0), |v| v.to_f32())?;
        let shadow_offset = vec2(0.0, 0.0);
        let shadow_offset = get(
            inline_table,
            SHADOW_OFFSET,
            || Ok(shadow_offset),
            |v| v.to_vec2(shadow_offset),
        )?;

        let background_visible = get(
            inline_table,
            BACKGROUND_VISIBLE,
            || Ok(false),
            |v| v.to_bool(),
        )?;

        let rotation = get(inline_table, ROTATION, || Ok(0.0), |v| v.to_f32())?;
        let scale = get(inline_table, SCALE, || Ok(1.0), |v| v.to_f32())?;
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::default()
        };
        let cursor = get(inline_table, CURSOR, || Ok(cursor), |v| v.to_cursor())?;
        let abs_pos = get(
            inline_table,
            ABS_POS,
            || Ok(None),
            |v| v.to_dvec2().map(Some),
        )?;
        let clip_x = get(inline_table, CLIP_X, || Ok(false), |v| v.to_bool())?;
        let clip_y = get(inline_table, CLIP_Y, || Ok(false), |v| v.to_bool())?;
        let align = Align::default();
        let align = get(inline_table, ALIGN, || Ok(align), |v| v.to_align(align))?;
        Ok(Self {
            theme,
            color,
            font_size,
            line_spacing,
            margin,
            padding,
            flow,
            underline_visible,
            underline_color,
            underline_width,
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
        })
    }
}

impl From<&LinkBasicProp> for ViewBasicProp {
    fn from(value: &LinkBasicProp) -> Self {
        let LinkBasicProp {
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
        ViewBasicProp {
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
