use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    components::traits::{BasicProp, Prop},
    error::Error,
    styles::manuel::{
        BASIC, COLOR, DISABLED, FLOW, FONT_SIZE, LINE_SPACING, MARGIN, PADDING, THEME,
    },
    themes::{Color, ColorFontConf, Theme, TomlValueTo},
    utils::{get_from_itable as get, get_from_table},
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct LabelProp {
    #[live]
    pub basic: LabelBasicProp,
    #[live]
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

impl TryFrom<&Item> for LabelProp {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[component.label] should be a table".to_string(),
        ))?;

        let basic = get_from_table(
            table,
            BASIC,
            || Ok(LabelBasicProp::default()),
            |v| (v, LabelState::None).try_into(),
        )?;

        let disabled = get_from_table(
            table,
            DISABLED,
            || {
                Ok(LabelBasicProp::from_state(
                    Theme::default(),
                    LabelState::Disabled,
                ))
            },
            |v| (v, LabelState::Disabled).try_into(),
        )?;

        Ok(Self { basic, disabled })
    }
}

impl Prop for LabelProp {
    type State = LabelState;
    type Basic = LabelBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            LabelState::None => &self.basic,
            LabelState::Disabled => &self.disabled,
        }
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct LabelBasicProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub color: Vec4,
    #[live]
    pub font_size: f32,
    #[live]
    pub line_spacing: f32,
    #[live]
    pub margin: Margin,
    #[live]
    pub padding: Padding,
    // #[live]
    // pub align: Align,
    #[live(Flow::RightWrap)]
    pub flow: Flow,
}

impl Default for LabelBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), LabelState::None)
    }
}

impl BasicProp for LabelBasicProp {
    type State = LabelState;
    type Colors = Color;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let color = Self::state_colors(theme, state);

        Self {
            theme,
            color: color.into(),
            font_size: 12.0,
            line_spacing: 1.2,
            margin: Margin {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
            padding: Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
            flow: Flow::RightWrap,
        }
    }

    fn state_colors(_theme: Theme, state: Self::State) -> Self::Colors {
        match state {
            LabelState::None => ColorFontConf::from_key("primary"),
            LabelState::Disabled => ColorFontConf::from_key("disabled"),
        }
    }
}

impl TryFrom<(&Item, LabelState)> for LabelBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, LabelState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "LabelProp should be a inline table".to_string(),
        ))?;
        let theme = Theme::default();
        let theme = get(inline_table, THEME, || Ok(theme), |value| value.try_into())?;
        let color = Self::state_colors(theme, state);
        let color = get(inline_table, COLOR, || Ok(color), |value| value.try_into())?.into();
        let font_size = get(inline_table, FONT_SIZE, || Ok(10.0), |item| item.to_f32())?;
        let line_spacing = get(inline_table, LINE_SPACING, || Ok(1.2), |item| item.to_f32())?;

        let default_margin = Margin {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        };

        let margin = get(
            inline_table,
            MARGIN,
            || Ok(default_margin),
            |item| item.to_margin(default_margin),
        )?;

        let default_padding = Padding {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        };

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

#[derive(Debug, Copy, Clone)]
pub enum LabelState {
    None,
    Disabled,
}
