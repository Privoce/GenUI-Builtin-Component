use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Prop},
    },
    error::Error,
    get_get_mut, getter_setter_prop,
    prop::{
        manuel::{
            BASIC, COLOR, DISABLED, FLOW, FONT_SIZE, HOVER, LINE_SPACING, MARGIN, PADDING, PRESSED,
            THEME, UNDERLINE_COLOR, UNDERLINE_VISIBLE, UNDERLINE_WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl,
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
        2 * LinkBasicProp::len()
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
    #[live]
    pub underline_visible: bool,
    #[live]
    pub underline_color: Vec4,
    #[live(1.0)]
    pub underline_width: f32,
    #[live(Flow::RightWrap)]
    pub flow: Flow,
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
    type Colors = (Color, Color);

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
            }
            COLOR => {
                let (color, _) = Self::state_colors(self.theme, state);
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
                let (_, underline_color) = Self::state_colors(self.theme, state);
                self.underline_color =
                    Vec4::from_live_color(value).unwrap_or(underline_color.into());
            }
            UNDERLINE_VISIBLE => {
                self.underline_visible = bool::from_live_value(value).unwrap_or(true);
            }
            UNDERLINE_WIDTH => {
                self.underline_width = f32::from_live_value(value).unwrap_or(1.0);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let (color, underline_color) = Self::state_colors(Theme::default(), state);
        self.color = color.into();
        self.underline_color = underline_color.into();
    }

    fn len() -> usize {
        10
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let (color, underline_color) = Self::state_colors(theme, state);

        Self {
            theme,
            color: color.into(),
            underline_color: underline_color.into(),
            underline_width: 1.0,
            underline_visible: true,
            font_size: 12.0,
            line_spacing: 1.2,
            margin: Margin::from_f64(0.0),
            padding: Padding::from_f64(0.0),
            flow: Flow::RightWrap,
        }
    }

    state_colors! {
        (color_level, underline_level),
        LinkState::Basic => (400, 400),
        LinkState::Hover => (300, 300),
        LinkState::Pressed => (500, 500),
        LinkState::Disabled => (200, 200)
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
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            margin: self.margin,
            height: Size::Fit,
            width: Size::Fit,
            ..Default::default()
        }
        .with_add_padding(self.padding)
    }

    fn layout(&self) -> Layout {
        Layout {
            padding: self.padding,
            flow: self.flow,
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
        let (color, underline_color) = Self::state_colors(theme, state);
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
        })
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
