use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_state, components::traits::{BasicProp, ComponentState, Prop}, error::Error, getter_setter_prop, prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, CURSOR, DISABLED, FLOW, HEIGHT, HOVER,
            MARGIN, PADDING, PRESSED, ROTATION, SCALE, SHADOW_COLOR, SHADOW_OFFSET, SPACING,
            SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl, Radius,
    }, themes::{Color, Theme, TomlValueTo}, utils::{get_from_itable, get_from_table}
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ViewProp {
    #[live(ViewBasicProp::default())]
    pub basic: ViewBasicProp,
    #[live]
    pub hover: ViewBasicProp,
    #[live]
    pub pressed: ViewBasicProp,
    #[live]
    pub disabled: ViewBasicProp,
}

impl Default for ViewProp {
    fn default() -> Self {
        Self {
            basic: ViewBasicProp::default(),
            hover: ViewBasicProp::from_state(Theme::default(), ViewState::Hover),
            pressed: ViewBasicProp::from_state(Theme::default(), ViewState::Pressed),
            disabled: ViewBasicProp::from_state(Theme::default(), ViewState::Basic),
        }
    }
}

impl Prop for ViewProp {
    type State = ViewState;
    type Basic = ViewBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            ViewState::Basic => &self.basic,
            ViewState::Hover => &self.hover,
            ViewState::Pressed => &self.pressed,
            ViewState::Disabled => &self.disabled,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            ViewState::Basic => &mut self.basic,
            ViewState::Hover => &mut self.hover,
            ViewState::Pressed => &mut self.pressed,
            ViewState::Disabled => &mut self.disabled,
        }
    }

    fn len() -> usize {
        ViewBasicProp::len() * 3
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            ViewState::Basic,
            [
                (ViewState::Hover, &mut self.hover),
                (ViewState::Pressed, &mut self.pressed),
                (ViewState::Disabled, &mut self.disabled),
            ],
        );
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
            BASIC,
            || Ok(ViewBasicProp::default()),
            |item| (item, ViewState::Basic).try_into(),
        )?;

        let hover = get_from_table(
            table,
            HOVER,
            || {
                Ok(ViewBasicProp::from_state(
                    Theme::default(),
                    ViewState::Hover,
                ))
            },
            |item| (item, ViewState::Hover).try_into(),
        )?;

        let pressed = get_from_table(
            table,
            PRESSED,
            || {
                Ok(ViewBasicProp::from_state(
                    Theme::default(),
                    ViewState::Pressed,
                ))
            },
            |item| (item, ViewState::Pressed).try_into(),
        )?;

        let disabled = get_from_table(
            table,
            DISABLED,
            || {
                Ok(ViewBasicProp::from_state(
                    Theme::default(),
                    ViewState::Disabled,
                ))
            },
            |item| (item, ViewState::Disabled).try_into(),
        )?;

        Ok(ViewProp {
            basic,
            hover,
            pressed,
            disabled,
        })
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct ViewBasicProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub background_color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live(0.0)]
    pub border_width: f32,
    #[live(Radius::new(4.0))]
    pub border_radius: Radius,
    #[live]
    pub shadow_color: Vec4,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(0.0)]
    pub blur_radius: f32,
    #[live(vec2(0.0, 0.0))]
    pub shadow_offset: Vec2,
    #[live(true)]
    pub background_visible: bool,
    #[live(0.0)]
    pub rotation: f32,
    #[live(1.0)]
    pub scale: f32,
    #[live(Padding::from_f64(6.0))]
    pub padding: Padding,
    #[live(Margin::from_f64(0.0))]
    pub margin: Margin,
    #[live(false)]
    pub clip_x: bool,
    #[live(false)]
    pub clip_y: bool,
    #[live(Align::default())]
    pub align: Align,
    #[live(MouseCursor::default())]
    pub cursor: MouseCursor,
    #[live(Flow::Down)]
    pub flow: Flow,
    #[live(6.0)]
    pub spacing: f64,
    #[live(Size::Fill)]
    pub height: Size,
    #[live(Size::Fill)]
    pub width: Size,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
}

impl BasicProp for ViewBasicProp {
    type State = ViewState;

    type Colors = (Color, Color, Color);

    fn len() -> usize {
        22
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
            BORDER_COLOR => {
                let (_, border_color, _) = Self::state_colors(self.theme, state);
                self.border_color = Vec4::from_live_color(value).unwrap_or(border_color.into());
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(0.0);
            }
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(4.0));
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
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            ROTATION => {
                self.rotation = f32::from_live_value(value).unwrap_or(0.0);
            }
            SCALE => {
                self.scale = f32::from_live_value(value).unwrap_or(1.0);
            }
            PADDING => {
                self.padding = Padding::from_live_value(value).unwrap_or(Padding::from_f64(6.0));
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
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
                    MouseCursor::Default
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::Down);
            }
            SPACING => {
                self.spacing = f64::from_live_value(value).unwrap_or(6.0);
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fill);
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fill);
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
            border_color: border_color.into(),
            border_width: 0.0,
            border_radius: Radius::new(4.0),
            shadow_color: shadow_color.into(),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            background_visible: true,
            rotation: 0.0,
            scale: 1.0,
            padding: Padding::from_f64(6.0),
            margin: Margin::from_f64(0.0),
            clip_x: false,
            clip_y: false,
            align: Align::default(),
            cursor,
            flow: Flow::Down,
            spacing: 6.0,
            height: Size::Fill,
            width: Size::Fill,
            abs_pos: None,
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        let (bg_level, border_level, shadow_level) = match state {
            ViewState::Basic => (500, 500, 400),
            ViewState::Hover => (400, 400, 300),
            ViewState::Pressed => (600, 600, 500),
            ViewState::Disabled => (300, 300, 200),
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
    fn live_props() -> Vec<(LiveId, Option<Vec<LiveId>>)> {
        vec![
            (live_id!(theme), None),
            (live_id!(background_color), None),
            (live_id!(border_color), None),
            (live_id!(border_width), None),
            (
                live_id!(border_radius),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (live_id!(shadow_color), None),
            (live_id!(spread_radius), None),
            (live_id!(blur_radius), None),
            (live_id!(shadow_offset), None),
            (live_id!(background_visible), None),
            (live_id!(rotation), None),
            (live_id!(scale), None),
            (
                live_id!(padding),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (live_id!(clip_x), None),
            (live_id!(clip_y), None),
            (live_id!(align), None),
            (live_id!(cursor), None),
            (live_id!(flow), None),
            (live_id!(spacing), None),
            (live_id!(height), None),
            (live_id!(width), None),
            (live_id!(abs_pos), None),
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
}

impl Default for ViewBasicProp {
    fn default() -> Self {
        ViewBasicProp::from_state(Theme::default(), ViewState::Basic)
    }
}

impl TryFrom<(&Value, ViewState)> for ViewBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Value, ViewState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.view.$state] should be an inline table".to_string(),
        ))?;
        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Item, ViewState)> for ViewBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, ViewState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.view.$state] should be an inline table".to_string(),
        ))?;
        (inline_table, state).try_into()
    }
}

impl TryFrom<(&InlineTable, ViewState)> for ViewBasicProp {
    type Error = Error;

    fn try_from((inline_table, state): (&InlineTable, ViewState)) -> Result<Self, Self::Error> {
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

        let border_color = get_from_itable(
            inline_table,
            BORDER_COLOR,
            || Ok(border_color),
            |v| v.try_into(),
        )?
        .into();

        let border_width = get_from_itable(inline_table, BORDER_WIDTH, || Ok(1.0), |v| v.to_f32())?;

        let border_radius = get_from_itable(
            inline_table,
            BORDER_RADIUS,
            || Ok(Radius::new(4.0)),
            |v| v.try_into(),
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

        let background_visible = get_from_itable(
            inline_table,
            BACKGROUND_VISIBLE,
            || Ok(true),
            |v| v.to_bool(),
        )?;

        let rotation = get_from_itable(inline_table, ROTATION, || Ok(0.0), |v| v.to_f32())?;
        let scale = get_from_itable(inline_table, SCALE, || Ok(1.0), |v| v.to_f32())?;
        let padding = Padding::from_f64(6.0);
        let padding = get_from_itable(
            inline_table,
            PADDING,
            || Ok(padding),
            |v| v.to_padding(padding),
        )?;
        let margin = Margin::from_f64(0.0);
        let margin = get_from_itable(inline_table, MARGIN, || Ok(margin), |v| v.to_margin(margin))?;
        let clip_x = get_from_itable(inline_table, CLIP_X, || Ok(false), |v| v.to_bool())?;
        let clip_y = get_from_itable(inline_table, CLIP_Y, || Ok(false), |v| v.to_bool())?;
        let align = Align::default();
        let align = get_from_itable(inline_table, ALIGN, || Ok(align), |v| v.to_align(align))?;
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::default()
        };
        let cursor = get_from_itable(inline_table, CURSOR, || Ok(cursor), |v| v.to_cursor())?;
        let flow = get_from_itable(inline_table, FLOW, || Ok(Flow::Down), |v| v.to_flow())?;
        let spacing = get_from_itable(inline_table, SPACING, || Ok(6.0), |v| v.to_f64())?;
        let height = get_from_itable(inline_table, HEIGHT, || Ok(Size::Fill), |v| v.to_size())?;
        let width = get_from_itable(inline_table, WIDTH, || Ok(Size::Fill), |v| v.to_size())?;
        let abs_pos = get_from_itable(
            inline_table,
            ABS_POS,
            || Ok(None),
            |v| v.to_dvec2().map(Some),
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
            abs_pos,
        })
    }
}

impl ViewBasicProp {
    getter_setter_prop! {
        get_theme, set_theme: theme -> Theme,
        get_background_color, set_background_color: background_color -> Vec4,
        get_border_color, set_border_color: border_color -> Vec4,
        get_border_width, set_border_width: border_width -> f32,
        get_border_radius, set_border_radius: border_radius -> Radius,
        get_shadow_color, set_shadow_color: shadow_color -> Vec4,
        get_spread_radius, set_spread_radius: spread_radius -> f32,
        get_blur_radius, set_blur_radius: blur_radius -> f32,
        get_shadow_offset, set_shadow_offset: shadow_offset -> Vec2,
        get_background_visible, set_background_visible: background_visible -> bool,
        get_rotation, set_rotation: rotation -> f32,
        get_scale, set_scale: scale -> f32,
        get_padding, set_padding: padding -> Padding,
        get_margin, set_margin: margin -> Margin,
        get_clip_x, set_clip_x: clip_x -> bool,
        get_clip_y, set_clip_y: clip_y -> bool,
        get_align, set_align: align -> Align,
        get_cursor, set_cursor: cursor -> MouseCursor,
        get_flow, set_flow: flow -> Flow,
        get_spacing, set_spacing: spacing -> f64,
        get_height, set_height: height -> Size,
        get_width, set_width: width -> Size,
        get_abs_pos, set_abs_pos: abs_pos -> Option<DVec2>
    }
}

component_state!{
    ViewState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    }, _ => ViewState::Basic
}


impl ViewState {
    pub fn id(&self) -> &[LiveId; 2] {
        match self {
            ViewState::Basic => id!(hover.off),
            ViewState::Hover => id!(hover.on),
            ViewState::Pressed => id!(hover.pressed),
            ViewState::Disabled => id!(hover.off),
        }
    }
}

impl ComponentState for ViewState {
    fn is_disabled(&self) -> bool {
        matches!(self, ViewState::Disabled)
    }
}