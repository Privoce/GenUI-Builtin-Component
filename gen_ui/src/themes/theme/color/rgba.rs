use std::str::FromStr;

/// # RGBA Color
/// format: `rgba(r, g, b, a)`
///
/// range:
/// - `0-255` for `r`, `g`, `b`
/// - `0.0-1.0` for `a`
///
/// example: `rgba(255, 0, 0, 0.2)`
#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl FromStr for Rgba {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("rgba") {
            let s = s.trim_start_matches("rgba(").trim_end_matches(')');
            let parts: Vec<&str> = s.split(',').map(str::trim).collect();
            if parts.len() != 4 {
                return Err(crate::error::Error::ThemeStyleParse(
                    "Invalid RGBA color format".to_string(),
                ));
            }
            let r = parts[0].parse::<u8>().map_err(|_| {
                crate::error::Error::ThemeStyleParse("Invalid red value in RGBA color".to_string())
            })?;
            let g = parts[1].parse::<u8>().map_err(|_| {
                crate::error::Error::ThemeStyleParse(
                    "Invalid green value in RGBA color".to_string(),
                )
            })?;
            let b = parts[2].parse::<u8>().map_err(|_| {
                crate::error::Error::ThemeStyleParse("Invalid blue value in RGBA color".to_string())
            })?;
            let a = parts[3].parse::<f32>().map_err(|_| {
                crate::error::Error::ThemeStyleParse(
                    "Invalid alpha value in RGBA color".to_string(),
                )
            })?;
            Ok(Rgba { r, g, b, a })
        } else {
            return Err(crate::error::Error::ThemeStyleParse(
                "Invalid RGBA color format".to_string(),
            ));
        }
    }
}

impl From<Rgba> for makepad_widgets::Vec4 {
    fn from(value: Rgba) -> Self {
        makepad_widgets::Vec4 {
            x: value.r as f32 / 255.0,
            y: value.g as f32 / 255.0,
            z: value.b as f32 / 255.0,
            w: value.a, // Alpha value is already in the range [0.0, 1.0]
        }
    }
}

impl Rgba {
    pub fn from_rgb(rgb: crate::themes::theme::color::Rgb, alpha: f32) -> Self {
        Rgba {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: alpha.clamp(0.0, 1.0),
        }
    }
    pub fn with_opacity(self, opacity: f32) -> Self {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (self.a * opacity).clamp(0.0, 1.0),
        }
    }
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Rgba {
            r,
            g,
            b,
            a: a.clamp(0.0, 1.0),
        }
    }
}
