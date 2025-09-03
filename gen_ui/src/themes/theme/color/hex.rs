use std::{fmt::Display, str::FromStr};

use makepad_widgets::Vec4;

use crate::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct Hex(pub Vec4);

impl FromStr for Hex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 去掉开头的 '#' 符号
        let hex = s.trim_start_matches('#');

        // 解析 RGB 值
        let (r, g, b, a) = if hex.len() == 3 {
            // 如果是 3 位数的十六进制颜色，重复每个字符
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap();
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap();
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap();
            (r, g, b, 255)
        } else if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
            (r, g, b, 255)
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
            (r, g, b, a)
        } else {
            return Err(Error::ThemeStyleParse(format!("invalid hex color: {}", s)));
        };

        Ok(Self(Vec4 {
            x: r as f32 / 255.0,
            y: g as f32 / 255.0,
            z: b as f32 / 255.0,
            w: a as f32 / 255.0,
        }))
    }
}

impl Hex {
    fn to_vec4(self) -> Vec4 {
        self.0
    }
    pub fn with_opacity(self, opacity: f32) -> Self {
        Hex(Vec4 {
            x: self.0.x,
            y: self.0.y,
            z: self.0.z,
            w: (self.0.w * opacity).clamp(0.0, 1.0),
        })
    }
}

impl From<Hex> for Vec4 {
    fn from(hex: Hex) -> Self {
        hex.to_vec4()
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = (self.0.x * 255.0) as u8;
        let g = (self.0.y * 255.0) as u8;
        let b = (self.0.z * 255.0) as u8;
        let a = (self.0.w * 255.0) as u8;

        f.write_str(&format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a))
    }
}

impl From<Vec4> for Hex {
    fn from(vec: Vec4) -> Self {
        Hex(vec)
    }
}
