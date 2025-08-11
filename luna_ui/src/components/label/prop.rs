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
        manuel::{BASIC, COLOR, DISABLED, FLOW, FONT_SIZE, LINE_SPACING, MARGIN, PADDING, THEME},
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl,
    },
    themes::{Color, ColorFontConf, Theme, TomlValueTo},
    try_from_toml_item,
    utils::get_from_itable as get,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct LabelProp {
    #[live(LabelBasicProp::default())]
    pub basic: LabelBasicProp,
    #[live(LabelBasicProp::from_state(Theme::default(), LabelState::Disabled))]
    pub disabled: LabelBasicProp,
}

impl Default for LabelProp {
    fn default() -> Self {
        Self {
            basic: LabelBasicProp::default(),
            disabled: LabelBasicProp::default(),
        }
    }
}

try_from_toml_item! {
    LabelProp {
        basic => BASIC, LabelBasicProp::default(), |v| (v, LabelState::Basic).try_into(),
        disabled => DISABLED, LabelBasicProp::from_state(Theme::default(), LabelState::Disabled), |v| (v, LabelState::Disabled).try_into()
    }, "[component.label] should be a table"
}

impl Prop for LabelProp {
    type State = LabelState;
    type Basic = LabelBasicProp;

    fn len() -> usize {
        2 * LabelBasicProp::len()
    }

    get_get_mut! {
        LabelState::Basic => basic,
        LabelState::Disabled => disabled
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            LabelState::Basic,
            [(LabelState::Disabled, &mut self.disabled)],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct LabelBasicProp {
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
    // #[live]
    // pub align: Align,
    #[live(Flow::RightWrap)]
    pub flow: Flow,
}

impl Default for LabelBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), LabelState::Basic)
    }
}

impl LabelBasicProp {
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

impl BasicProp for LabelBasicProp {
    type State = LabelState;
    type Colors = Color;

    fn set_from_str(&mut self, key: &str, value: &LiveValue, _state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
            }
            COLOR => {
                self.color = Vec4::from_live_color(value)
                    .unwrap_or(ColorFontConf::from_key("primary").into());
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
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.color = Self::state_colors(Theme::default(), state).into();
    }

    fn len() -> usize {
        7
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let color = Self::state_colors(theme, state);

        Self {
            theme,
            color: color.into(),
            font_size: 12.0,
            line_spacing: 1.2,
            margin: Margin::from_f64(0.0),
            padding: Padding::from_f64(0.0),
            flow: Flow::RightWrap,
        }
    }

    fn state_colors(_theme: Theme, state: Self::State) -> Self::Colors {
        match state {
            LabelState::Basic => ColorFontConf::from_key("primary"),
            LabelState::Disabled => ColorFontConf::from_key("disabled"),
        }
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

impl TryFrom<(&Item, LabelState)> for LabelBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, LabelState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "LabelProp should be a inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Value, LabelState)> for LabelBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Value, LabelState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "LabelProp should be a inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&InlineTable, LabelState)> for LabelBasicProp {
    type Error = Error;

    fn try_from((inline_table, state): (&InlineTable, LabelState)) -> Result<Self, Self::Error> {
        let theme = Theme::default();
        let theme = get(inline_table, THEME, || Ok(theme), |value| value.try_into())?;
        let color = Self::state_colors(theme, state);
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

        Ok(Self {
            theme,
            color,
            font_size,
            line_spacing,
            margin,
            padding,
            flow,
        })
    }
}

component_state! {
    LabelState {
        Basic => BASIC,
        Disabled => DISABLED
    },
    _ => LabelState::Basic
}

impl ComponentState for LabelState {
    fn is_disabled(&self) -> bool {
        matches!(self, LabelState::Disabled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Live, LiveHook, Default)]
#[live_ignore]
pub enum FontMode {
    #[pick]
    #[default]
    Regular,
    Bold,
    Italic,
    BoldItalic,
}
