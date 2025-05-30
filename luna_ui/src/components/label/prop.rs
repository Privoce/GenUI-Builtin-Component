use makepad_widgets::*;
use toml_edit::Value;

use crate::{
    error::Error,
    themes::{Color, Theme, TomlValueTo},
    utils::get_from_itable as get,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct LabelProp {
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

impl Default for LabelProp {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            color: Color::Hex("#FFFFFFE6".parse().unwrap()).into(),
            font_size: 10.0,
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
}

impl TryFrom<&Value> for LabelProp {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "LabelProp should be a inline table".to_string(),
        ))?;

        let theme = get(
            inline_table,
            "theme",
            || Ok(Theme::default()),
            |value| value.try_into(),
        )?;

        let color = get(
            inline_table,
            "color",
            || Ok(Color::Hex("#FFFFFFE6".parse()?)),
            |value| value.try_into(),
        )?
        .into();

        let font_size = get(inline_table, "font_size", || Ok(10.0), |item| item.to_f32())?;
        let line_spacing = get(
            inline_table,
            "line_spacing",
            || Ok(1.2),
            |item| item.to_f32(),
        )?;

        let default_margin = Margin {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        };

        let margin = get(
            inline_table,
            "margin",
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
            "padding",
            || Ok(default_padding),
            |item| item.to_padding(default_padding),
        )?;

        let flow = get(
            inline_table,
            "flow",
            || Ok(Flow::RightWrap),
            |item| item.to_flow(),
        )?;

        Ok(LabelProp {
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
