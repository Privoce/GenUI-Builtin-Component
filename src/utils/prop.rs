use std::{path::PathBuf, str::FromStr};

use makepad_widgets::{vec2, vec3, vec4, DVec2, LiveDependency, LiveId, Rect, Vec2, Vec3, Vec4};

use crate::themes::{
    ThemeColorValue, ThemeDark, ThemeError, ThemeInfo, ThemePrimary, ThemeSuccess, ThemeWarning,
    Themes,
};

// -------------------------------------------------------------------------------------------------
/// This trait is used to get the color of the theme
pub trait ThemeColor {
    /// Get the color of the theme. if color is none, return the default color
    fn get(&self, theme: Themes, default: u32) -> Vec4;
    fn use_or(&self, hex: &str) -> Result<Vec4, Box<dyn std::error::Error>>;
}

impl ThemeColor for Option<Vec4> {
    fn get(&self, theme: Themes, default: u32) -> Vec4 {
        get_color(theme, self.as_ref(), default)
    }
    fn use_or(&self, hex: &str) -> Result<Vec4, Box<dyn std::error::Error>> {
        if let Some(target) = self {
            Ok(target.clone())
        } else {
            hex_to_vec4(hex)
        }
    }
}

pub fn hex_to_vec4(hex: &str) -> Result<Vec4, Box<dyn std::error::Error>> {
    // 去掉开头的 '#' 符号
    let hex = hex.trim_start_matches('#');

    // 解析 RGB 值
    let (r, g, b, a) = if hex.len() == 6 {
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
        return Err(format!("invalid hex color: {}", hex).into());
    };

    Ok(Vec4 {
        x: r as f32 / 255.0,
        y: g as f32 / 255.0,
        z: b as f32 / 255.0,
        w: a as f32 / 255.0,
    })
}

pub fn vec4_to_hex(vec4: &Vec4) -> String {
    let r = (vec4.x * 255.0) as u8;
    let g = (vec4.y * 255.0) as u8;
    let b = (vec4.z * 255.0) as u8;
    let a = (vec4.w * 255.0) as u8;

    format!("{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

/// v: color value, range: `[25, 50, 100 ,200, 300, 400, 500, 600, 700, 800, 900]`
pub fn get_color(theme: Themes, color: Option<&Vec4>, v: u32) -> Vec4 {
    return if let Some(target) = color {
        target.clone()
    } else {
        match theme {
            Themes::Dark => ThemeDark::v(v),
            Themes::Primary => ThemePrimary::v(v),
            Themes::Error => ThemeError::v(v),
            Themes::Warning => ThemeWarning::v(v),
            Themes::Success => ThemeSuccess::v(v),
            Themes::Info => ThemeInfo::v(v),
        }
    };
}

#[macro_export]
macro_rules! theme_color {
    (
        $(
            $T:ident {
                $(
                    $K:ident : $V:expr,
                )*
            }
        ),*
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $T(Vec4);

            impl $T {
                $(
                    pub const $K: &'static str = $V;
                )*
            }

            impl Default for $T {
                fn default() -> Self {
                    Self(crate::utils::hex_to_vec4(Self::_500).unwrap())
                }
            }


            impl crate::themes::ThemeColorValue for $T {
                fn v(target: u32) -> Vec4 {
                    crate::utils::hex_to_vec4(match target {
                        25 => Self::_25,
                        50 => Self::_50,
                        100 => Self::_100,
                        200 => Self::_200,
                        300 => Self::_300,
                        400 => Self::_400,
                        500 => Self::_500,
                        600 => Self::_600,
                        700 => Self::_700,
                        800 => Self::_800,
                        900 => Self::_900,
                        _ => panic!("invalid target"),
                    }).unwrap()
                }

                fn get(&self) -> Vec4 {
                    self.0
                }

                fn hex(target: u32) -> &'static str {
                    match target {
                        25 => Self::_25,
                        50 => Self::_50,
                        100 => Self::_100,
                        200 => Self::_200,
                        300 => Self::_300,
                        400 => Self::_400,
                        500 => Self::_500,
                        600 => Self::_600,
                        700 => Self::_700,
                        800 => Self::_800,
                        900 => Self::_900,
                        _ => panic!("invalid target"),
                    }
                }
            }
        )*
    };
}
// -------------------------------------------------------------------------------------------------
/// This trait is used to transform f32/f64 to bool
pub trait ToBool {
    /// Transform f32/f64 to bool
    fn to_bool(&self) -> bool;
}

impl ToBool for f32 {
    fn to_bool(&self) -> bool {
        *self != 0.0
    }
}

pub trait BoolToF32 {
    /// Transform bool to f32/f64
    fn to_f32(&self) -> f32;
}

impl BoolToF32 for bool {
    fn to_f32(&self) -> f32 {
        *self as u8 as f32
    }
}

pub trait FloatToVec {
    fn to_vec2(self) -> Vec2;
    fn to_vec3(self) -> Vec3;
    fn to_vec4(self) -> Vec4;
}

impl FloatToVec for f64 {
    fn to_vec2(self) -> Vec2 {
        vec2(self as f32, self as f32)
    }

    fn to_vec3(self) -> Vec3 {
        vec3(self as f32, self as f32, self as f32)
    }

    fn to_vec4(self) -> Vec4 {
        vec4(self as f32, self as f32, self as f32, self as f32)
    }
}

impl FloatToVec for f32 {
    fn to_vec2(self) -> Vec2 {
        vec2(self, self)
    }

    fn to_vec3(self) -> Vec3 {
        vec3(self, self, self)
    }

    fn to_vec4(self) -> Vec4 {
        vec4(self, self, self, self)
    }
}

// --------------------------------------------------------------------------------------------------
pub trait Render {
    fn render(&mut self);
}

// ---------------------------------------------------------------------------------------------------
pub trait ToPath {
    fn to_pathbuf(&self) -> PathBuf;
}

impl ToPath for LiveDependency {
    fn to_pathbuf(&self) -> PathBuf {
        PathBuf::from_str(self.as_str()).unwrap()
    }
}

// ----------------------------------------------------------------------------------------------------
pub trait RectExpand {
    fn abs_start(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_start_center(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_end(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
    fn abs_end_center(&mut self, by: &Rect, offset: Option<DVec2>) -> ();
}

impl RectExpand for Rect {
    fn abs_start(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, .. } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x;
            self.pos.y = pos.y + v.y;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y;
        };
    }

    fn abs_start_center(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x;
            self.pos.y = pos.y + v.y + size.y * 0.5 - self.size.y * 0.5;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y + size.y * 0.5 - self.size.y * 0.5;
        };
    }

    fn abs_end(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x + size.x;
            self.pos.y = pos.y + v.y;
        } else {
            self.pos.x = pos.x;
            self.pos.y = pos.y;
        };
    }

    fn abs_end_center(&mut self, by: &Rect, offset: Option<DVec2>) -> () {
        let Rect { pos, size } = by;

        if let Some(v) = offset {
            self.pos.x = pos.x + v.x + size.x;
            self.pos.y = pos.y + v.y + size.y * 0.5 - self.size.y * 0.5;
        } else {
            self.pos.x = pos.x + size.x;
            self.pos.y = pos.y + size.y * 0.5 - self.size.y * 0.5;
        };
    }
}

// ------------------------------------------------------------------------------------------------------------

pub trait ToDVec {
    fn to_dvec2(self) -> DVec2;
}

impl ToDVec for Vec2 {
    fn to_dvec2(self) -> DVec2 {
        DVec2 {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

// ------------------------------------------------------------------------------------------------------------

pub trait LiveIdExp {
    fn as_slice(&self) -> &[LiveId];
}

impl LiveIdExp for LiveId {
    fn as_slice(&self) -> &[LiveId] {
        std::slice::from_ref(self)
    }
}
