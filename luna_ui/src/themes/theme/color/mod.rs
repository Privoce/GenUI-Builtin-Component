mod font;
mod hex;
mod rgb;
mod rgba;

pub use font::ColorFontConf;
pub use hex::Hex;
pub use rgb::Rgb;
pub use rgba::Rgba;
use toml_edit::Value;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Color {
    Hex(Hex),
    RGB(Rgb),
    RGBA(Rgba),
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
