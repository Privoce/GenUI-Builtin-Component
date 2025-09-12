use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_color, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
    },
    error::Error,
    get_get_mut, getter_setter_prop,
    prop::{
        manuel::{
            BASIC, COLOR, DISABLED, FLOW, FONT_SIZE, HEIGHT, LINE_SPACING, MARGIN, PADDING, THEME,
            WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplyStateMapImpl,
    },
    prop_interconvert,
    themes::{ColorFontConf, Theme, TomlValueTo},
};

prop_interconvert! {
    LabelProp {
        basic_prop = LabelBasicStyle;
        basic => BASIC, LabelBasicStyle::default(), |v| (v, LabelState::Basic).try_into(),
        disabled => DISABLED, LabelBasicStyle::from_state(Theme::default(), LabelState::Disabled), |v| (v, LabelState::Disabled).try_into()
    }, "[component.label] should be a table"
}

impl Style for LabelProp {
    type State = LabelState;
    type Basic = LabelBasicStyle;

    fn len() -> usize {
        2 * LabelBasicStyle::len()
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

basic_prop_interconvert! {
    LabelBasicStyle {
        state = LabelState;
        {color => COLOR, |v| v.try_into()};
        {
            font_size: f32 => FONT_SIZE, 12.0, |v| v.to_f32(),
            line_spacing: f32 => LINE_SPACING, 1.0, |v| v.to_f32(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(Margin::from_f64(0.0)),
            padding: Padding => PADDING, Padding::from_f64(0.0), |v| v.to_padding(Padding::from_f64(0.0)),
            flow: Flow => FLOW, Flow::RightWrap, |v| v.to_flow(),
            height: Size => HEIGHT, Size::Fit, |v| v.to_size(),
            width: Size => WIDTH, Size::Fit, |v| v.to_size()
        }
    }, "LabelBasicStyle should be a inline table"
}

impl LabelBasicStyle {
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

component_color! {
    LabelColors {
        colors = (Color);
        color
    }
}

impl BasicStyle for LabelBasicStyle {
    type State = LabelState;
    type Colors = LabelColors;

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
            }
            COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.color.into());
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
            color: color.color.into(),
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
            LabelState::Basic => ColorFontConf::from_key("primary").into(),
            LabelState::Disabled => ColorFontConf::from_key("disabled").into(),
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
