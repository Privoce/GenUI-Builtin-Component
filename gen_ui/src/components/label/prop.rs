use makepad_widgets::*;

use crate::{
    component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Prop},
    },
    error::Error,
    get_get_mut, getter_setter_prop, interconvert_basic_prop_toml, interconvert_prop_toml,
    prop::{
        manuel::{
            BASIC, COLOR, DISABLED, FLOW, FONT_SIZE, HEIGHT, LINE_SPACING, MARGIN, PADDING, THEME,
            WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplyStateMapImpl,
    },
    themes::{Color, ColorFontConf, Theme, TomlValueTo},
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

interconvert_prop_toml! {
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
    #[live(1.0)]
    pub line_spacing: f32,
    #[live(Margin::from_f64(0.0))]
    pub margin: Margin,
    #[live(Padding::from_f64(0.0))]
    pub padding: Padding,
    // #[live]
    // pub align: Align,
    #[live(Size::Fit)]
    pub height: Size,
    #[live(Size::Fit)]
    pub width: Size,
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

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
            }
            COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.into());
            }
            FONT_SIZE => {
                self.font_size = f32::from_live_value(value).unwrap_or(12.0);
            }
            LINE_SPACING => {
                self.line_spacing = f32::from_live_value(value).unwrap_or(1.0);
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
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            _ => {}
        }
    }

    fn sync(&mut self, _state: Self::State) -> () {
        // self.color = Self::state_colors(Theme::default(), state).into();
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
            line_spacing: 1.0,
            margin: Margin::from_f64(0.0),
            padding: Padding::from_f64(0.0),
            flow: Flow::RightWrap,
            height: Size::Fit,
            width: Size::Fit,
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
            (live_id!(height), None.into()),
            (live_id!(width), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            margin: self.margin,
            height: self.height,
            width: self.width,
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

interconvert_basic_prop_toml! {
    LabelBasicProp {
        state = LabelState;
        colors = color;
        color => COLOR, |v| v.try_into(),
        {
            font_size => FONT_SIZE, 12.0, |v| v.to_f32(),
            line_spacing => LINE_SPACING, 1.0, |v| v.to_f32(),
            margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(Margin::from_f64(0.0)),
            padding => PADDING, Padding::from_f64(0.0), |v| v.to_padding(Padding::from_f64(0.0)),
            flow => FLOW, Flow::RightWrap, |v| v.to_flow(),
            height => HEIGHT, Size::Fit, |v| v.to_size(),
            width => WIDTH, Size::Fit, |v| v.to_size()
        }
    }, "LabelBasicProp should be a inline table"
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
