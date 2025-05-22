use crate::{error::GError, themes::Themes, utils::hex_to_vec4};
use makepad_widgets::Vec4;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    sequence::{preceded, tuple},
    IResult,
};
use std::str::FromStr;
use toml_edit::{Item, Table};

#[derive(Debug, Clone)]
pub struct ThemeGlobal {
    /// the global theme
    pub theme: Themes,
    /// the global styles, default is Flat, users can add their own styles
    pub styles: Vec<String>,
    pub basic: ThemeBasic,
}

impl Default for ThemeGlobal {
    fn default() -> Self {
        Self {
            theme: Themes::Dark,
            styles: vec!["flat".to_string()],
            basic: ThemeBasic::default(),
        }
    }
}

impl TryFrom<&Item> for ThemeGlobal {
    type Error = GError;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value
            .as_table()
            .ok_or_else(|| GError::ThemeStyleParse("[global] should be a table".to_string()))?;

        let theme = table
            .get("theme")
            .map_or_else(|| Ok(Themes::default()), |theme| theme.try_into())?;

        let styles = table.get("styles").map_or_else(
            || Ok(vec!["flat".to_string()]),
            |styles| {
                styles
                    .as_array()
                    .ok_or_else(|| {
                        GError::ThemeStyleParse("[global.styles] should be an array".to_string())
                    })
                    .map(|arr| {
                        let mut styles = arr
                            .iter()
                            .filter_map(|item| item.as_str())
                            .map(|item| item.to_string())
                            .collect::<Vec<String>>();
                        styles.push("flat".to_string());
                        styles
                    })
            },
        )?;

        let basic = table
            .get("basic")
            .map_or_else(|| Ok(ThemeBasic::default()), |basic| basic.try_into())?;

        Ok(ThemeGlobal {
            theme,
            styles,
            basic,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ThemeBasic {
    /// basic color for any text
    pub color: Vec4,
    /// basic border with
    pub border_width: f32,
    /// basic border radius for controls, such as button
    pub border_radius: f32,
    /// basic height for controls, such as button, radio, checkbox height
    pub control_height: f32,
    pub  margin: Vec4,
    pub padding: Vec4,
}

impl Default for ThemeBasic {
    fn default() -> Self {
        Self {
            color: hex_to_vec4("#000000").unwrap(),
            border_width: 1.0,
            border_radius: 4.0,
            control_height: 32.0,
            margin: Vec4::all(6.0),
            padding: Vec4::all(6.0),
        }
    }
}

impl TryFrom<&Item> for ThemeBasic {
    type Error = GError;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        fn float(table: &Table, key: &str, default: f32) -> Result<f32, GError> {
            table.get(key).map_or_else(
                || Ok(default),
                |item| {
                    item.as_float()
                        .ok_or_else(|| {
                            GError::ThemeStyleParse(format!(
                                "[global.basic.{}] should be a float",
                                key
                            ))
                        })
                        .map(|f| f as f32)
                },
            )
        }

        fn vec4(table: &Table, key: &str, default: Vec4, ) -> Result<Vec4, GError> {
            table.get(key).map_or_else(
                || Ok(default),
                |item| {
                    item.as_array()
                        .ok_or_else(|| {
                            GError::ThemeStyleParse(format!(
                                "[global.basic.{}] should be a vec4, which format is [x, y, z, w]",
                                key
                            ))
                        })
                        .and_then(|arr| {
                            let arr = arr.iter().filter_map(|item| item.as_float()).map(|f|f as f32).collect::<Vec<f32>>();

                            let len = arr.len();

                            match len {
                                0 => Ok(Vec4::all(0.0)),
                                1 => Ok(Vec4::all(arr[0])),
                                2 => Ok(Vec4 { x: arr[0], y: arr[1], z: arr[0], w: arr[1] }),
                                4 => Ok(Vec4 { x: arr[0], y: arr[1], z: arr[2], w: arr[3] }),
                                _ => Err(GError::ThemeStyleParse(format!(
                                    "[global.basic.{}] should be a vec4, which format is [x, y, z, w], accept length is 0, 1, 2, 4, but got {}",
                                    key, len
                                ))), 
                                    
                             }
                        })
                },
            )
        }

        let table = value.as_table().ok_or_else(|| {
            GError::ThemeStyleParse("[global.basic] should be a table".to_string())
        })?;

        let color = table.get("color").map_or_else(
            || Ok(Hex("#000000".to_string())),
            |color| {
                color
                    .as_str()
                    .ok_or_else(|| {
                        GError::ThemeStyleParse("[global.basic.color] should be a hex".to_string())
                    })
                    .and_then(|color| {
                        Hex::try_from(color).map_err(|e| {
                            GError::ThemeStyleParse(format!(
                                "[global.basic.color] parse error: {}",
                                e
                            ))
                        })
                    })
            },
        )?;

        let border_width = float(table, "border_width",  1.0)?;
        let border_radius = float(table, "border_radius", 4.0)?;
        let control_height = float(table, "control_height", 32.0)?;
        let margin = vec4(table, "margin", Vec4::all(6.0))?;
        let padding = vec4(table, "padding", Vec4::all(6.0))?;

        Ok(ThemeBasic {
            color: hex_to_vec4(&color.to_vec4()).unwrap(),
            border_width,
            border_radius,
            control_height,
            margin,
            padding,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hex(pub String);

impl Hex {
    /// 将16进制颜色转换为vec4
    pub fn to_vec4(&self) -> String {
        fn u8_to_str(hex: &str, start: usize, end: usize) -> String {
            float_to_str(u8::from_str_radix(&hex[start..end], 16).unwrap() as f32 / 255.0)
        }

        // 去掉开头的 '#' 符号
        let hex = self.0.trim_start_matches('#');

        // 解析 RGB 值
        let r = u8_to_str(hex, 0, 2);
        let g = u8_to_str(hex, 2, 4);
        let b = u8_to_str(hex, 4, 6);
        let a = u8_to_str(hex, 6, 8);

        format!("vec4({}, {}, {}, {})", r, g, b, a)
    }
}

impl TryFrom<&str> for Hex {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Hex::from_str(value)
    }
}

impl FromStr for Hex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_hex_color(s) {
            Ok((remain, hex)) => {
                if remain.is_empty() {
                    return Ok(Hex(format!("#{}", hex)));
                } else {
                    return Err(format!("Invalid hex color: {}", s));
                }
            }
            Err(e) => {
                return Err(format!("Invalid hex color: {}: {}", s, e));
            }
        }
    }
}

pub fn float_to_str(num: f32) -> String {
    if num.fract() == 0.0 {
        format!("{}.0", num)
    } else {
        format!("{}", num)
    }
}

/// parse 3 hex color
fn three_hex_digits(input: &str) -> IResult<&str, String> {
    map_res(tuple((hex_digit, hex_digit, hex_digit)), |(a, b, c)| {
        format!("{}{}{}{}{}{}FF", a, a, b, b, c, c).parse()
    })(input)
}

/// Parse 6 hex color
fn six_hex_digits(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    )))(input)
}

fn eight_hex_digits(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    )))(input)
}

/// parse hex color
/// - #3       : single
/// - #333     : third
/// - #333333  : sixth
/// - #33333333: eighth
pub fn parse_hex_color(input: &str) -> IResult<&str, String> {
    preceded(
        tag("#"),
        alt((
            map_res(eight_hex_digits, |s: &str| s.parse()),
            map_res(six_hex_digits, |s: &str| format!("{}FF", s).parse()),
            three_hex_digits,
            map_res(hex_digit, |s| format!("{}FF", s.repeat(6)).parse()),
        )),
    )(input)
}

/// parse single hex color
fn hex_digit(input: &str) -> IResult<&str, &str> {
    recognize(one_of("0123456789abcdefABCDEF"))(input)
}
