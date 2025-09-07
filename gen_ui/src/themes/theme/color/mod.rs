mod font;
mod hex;
mod rgb;
mod rgba;

use std::fmt::Display;

pub use font::ColorFontConf;
pub use hex::Hex;
use makepad_widgets::Vec4;
pub use rgb::Rgb;
pub use rgba::Rgba;
use toml_edit::{Formatted, Value};

use crate::error::Error;

#[derive(Debug, Clone, Copy, Default)]
pub enum Color {
    Hex(Hex),
    RGB(Rgb),
    RGBA(Rgba),
    WHITE,
    #[default]
    BLACK,
}

impl Color {
    pub fn with_opacity(self, opacity: f32) -> Self {
        match self {
            Color::Hex(hex) => Color::Hex(hex.with_opacity(opacity)),
            Color::RGB(rgb) => Color::RGBA(Rgba::from_rgb(rgb, opacity)),
            Color::RGBA(rgba) => Color::RGBA(rgba.with_opacity(opacity)),
            Color::WHITE => Color::RGBA(Rgba::new(255, 255, 255, opacity)),
            Color::BLACK => Color::RGBA(Rgba::new(0, 0, 0, opacity)),
        }
    }
}

impl TryFrom<&Value> for Color {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let color_str = value.as_str().ok_or(Error::ThemeStyleParse(
            "Color value must be a string".to_string(),
        ))?;

        if color_str.starts_with('#') {
            color_str.parse::<Hex>().map(Color::Hex)
        } else if color_str.starts_with("rgba") {
            color_str.parse::<Rgba>().map(Color::RGBA)
        } else if color_str.starts_with("rgb") {
            color_str.parse::<Rgb>().map(Color::RGB)
        } else {
            Err(Error::ThemeStyleParse("Invalid color format".to_string()))
        }
    }
}

impl From<Color> for Value {
    fn from(value: Color) -> Self {
        Value::String(Formatted::new(value.to_string()))
    }
}

impl From<Color> for Vec4 {
    fn from(value: Color) -> Self {
        match value {
            Color::Hex(hex) => hex.into(),
            Color::RGB(rgb) => rgb.into(),
            Color::RGBA(rgba) => rgba.into(),
            Color::WHITE => "#FFFFFF".parse::<Hex>().unwrap().into(),
            Color::BLACK => "#000000".parse::<Hex>().unwrap().into(),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(hex) => write!(f, "{}", hex),
            Color::RGB(rgb) => write!(f, "{}", Hex((*rgb).into())),
            Color::RGBA(rgba) => write!(f, "{}", Hex((*rgba).into())),
            Color::WHITE => write!(f, "#FFFFFF"),
            Color::BLACK => write!(f, "#000000"),
        }
    }
}
