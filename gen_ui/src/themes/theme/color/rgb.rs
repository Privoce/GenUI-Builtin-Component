use std::str::FromStr;

use makepad_widgets::Vec4;

use crate::error::Error;

/// # RGB Color
/// format: `rgb(r, g, b)`
/// 
/// range: `0-255`
/// 
/// example: `rgb(255, 0, 0)`
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Rgb {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("rgb") {
            let s = s.trim_start_matches("rgb(").trim_end_matches(')');
            let parts: Vec<&str> = s.split(',').map(str::trim).collect();
            if parts.len() != 3 {
                return Err(Error::ThemeStyleParse(
                    "Invalid RGB color format".to_string(),
                ));
            }
            let r = parts[0].parse::<u8>().map_err(|_| {
                Error::ThemeStyleParse("Invalid red value in RGB color".to_string())
            })?;
            let g = parts[1].parse::<u8>().map_err(|_| {
                Error::ThemeStyleParse("Invalid green value in RGB color".to_string())
            })?;
            let b = parts[2].parse::<u8>().map_err(|_| {
                Error::ThemeStyleParse("Invalid blue value in RGB color".to_string())
            })?;
            Ok(Rgb { r, g, b })
        }else{
            return Err(Error::ThemeStyleParse(
                "Invalid RGB color format".to_string(),
            ));
        }
    }
}

impl From<Rgb> for Vec4 {
    fn from(value: Rgb) -> Self {
        Vec4 {
            x: value.r as f32 / 255.0,
            y: value.g as f32 / 255.0,
            z: value.b as f32 / 255.0,
            w: 1.0, // Default alpha value
        }
    }
}