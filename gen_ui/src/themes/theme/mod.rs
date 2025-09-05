mod color;
pub mod conf;

use std::{fmt::Display, str::FromStr};

pub use color::*;
use makepad_widgets::*;
use toml_edit::{Formatted, Item, Value};

use crate::{
    error::Error,
    prop::{
        manuel::{DARK, DARK_UP, ERROR, ERROR_UP, INFO, INFO_UP, PRIMARY, PRIMARY_UP, SUCCESS, SUCCESS_UP, WARNING, WARNING_UP},
        traits::{FromLiveValue, ToTomlValue},
    },
};

#[derive(Copy, Clone, Debug, Live, LiveHook, Default)]
#[live_ignore]
pub enum Theme {
    #[pick]
    #[default]
    Dark,
    Primary,
    Error,
    Warning,
    Success,
    Info,
}

impl FromLiveValue for Theme {
    fn from_live_value(value: &LiveValue) -> Option<Self> {
        if let LiveValue::BareEnum(theme) = value {
            match theme.to_string().as_str() {
                DARK_UP => Some(Theme::Dark),
                PRIMARY_UP => Some(Theme::Primary),
                ERROR_UP => Some(Theme::Error),
                WARNING_UP => Some(Theme::Warning),
                SUCCESS_UP => Some(Theme::Success),
                INFO_UP => Some(Theme::Info),
                _ => None,
            }
        } else {
            None
        }
    }
}


impl From<&LiveValue> for Theme {
    fn from(value: &LiveValue) -> Self {
        (value, Theme::default()).into()
    }
}

impl From<(&LiveValue, Theme)> for Theme {
    fn from((value, default): (&LiveValue, Theme)) -> Self {
        if let LiveValue::BareEnum(theme) = value {
            theme.to_string().parse().unwrap_or(default)
        } else {
            default
        }
    }
}

impl Theme {
    pub fn colors(&self) -> [Color; 10] {
        match self {
            Theme::Dark => [
                Self::dark(50),
                Self::dark(100),
                Self::dark(200),
                Self::dark(300),
                Self::dark(400),
                Self::dark(500),
                Self::dark(600),
                Self::dark(700),
                Self::dark(800),
                Self::dark(900),
            ],
            Theme::Primary => [
                Self::primary(50),
                Self::primary(100),
                Self::primary(200),
                Self::primary(300),
                Self::primary(400),
                Self::primary(500),
                Self::primary(600),
                Self::primary(700),
                Self::primary(800),
                Self::primary(900),
            ],
            Theme::Error => [
                Self::error(50),
                Self::error(100),
                Self::error(200),
                Self::error(300),
                Self::error(400),
                Self::error(500),
                Self::error(600),
                Self::error(700),
                Self::error(800),
                Self::error(900),
            ],
            Theme::Warning => [
                Self::warning(50),
                Self::warning(100),
                Self::warning(200),
                Self::warning(300),
                Self::warning(400),
                Self::warning(500),
                Self::warning(600),
                Self::warning(700),
                Self::warning(800),
                Self::warning(900),
            ],
            Theme::Success => [
                Self::success(50),
                Self::success(100),
                Self::success(200),
                Self::success(300),
                Self::success(400),
                Self::success(500),
                Self::success(600),
                Self::success(700),
                Self::success(800),
                Self::success(900),
            ],
            Theme::Info => [
                Self::info(50),
                Self::info(100),
                Self::info(200),
                Self::info(300),
                Self::info(400),
                Self::info(500),
                Self::info(600),
                Self::info(700),
                Self::info(800),
                Self::info(900),
            ],
        }
    }
    pub fn color(&self, level: u32) -> Color {
        match self {
            Theme::Dark => Self::dark(level),
            Theme::Primary => Self::primary(level),
            Theme::Error => Self::error(level),
            Theme::Warning => Self::warning(level),
            Theme::Success => Self::success(level),
            Theme::Info => Self::info(level),
        }
    }
    pub fn primary(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#F7ECFE",
                100 => "#EEDCFE",
                200 => "#D9BAFD",
                300 => "#C597FF",
                400 => "#AD72FF",
                500 => "#9254EA",
                600 => "#7438D2",
                700 => "#5629A4",
                800 => "#400B84",
                900 => "#280255",
                _ => "#9254EA",
            })
            .unwrap(),
        )
    }
    pub fn dark(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#F2F2F2",
                100 => "#E3E3E3",
                200 => "#C7C7C7",
                300 => "#ABABAB",
                400 => "#919191",
                500 => "#777777",
                600 => "#5E5E5E",
                700 => "#474747",
                800 => "#303030",
                900 => "#1B1B1C",
                _ => "#777777",
            })
            .unwrap(),
        )
    }
    pub fn info(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#E7F2FF",
                100 => "#D0E4FF",
                200 => "#A1C9FF",
                300 => "#76ACFF",
                400 => "#4E8FF8",
                500 => "#3271EA",
                600 => "#1157CE",
                700 => "#04409F",
                800 => "#012C6F",
                900 => "#001944",
                _ => "#3271EA",
            })
            .unwrap(),
        )
    }
    pub fn error(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#FFECEE",
                100 => "#FFDADC",
                200 => "#FFB3AE",
                300 => "#FF8983",
                400 => "#F55E57",
                500 => "#DB372D",
                600 => "#B3251E",
                700 => "#8A1A16",
                800 => "#60150F",
                900 => "#3A0907",
                _ => "#DB372D",
            })
            .unwrap(),
        )
    }
    pub fn warning(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#fff2e8",
                100 => "#ffd8bf",
                200 => "#ffbb96",
                300 => "#ff9c6e",
                400 => "#ff7a45",
                500 => "#fa541c",
                600 => "#d4380d",
                700 => "#ad2102",
                800 => "#871400",
                900 => "#610b00",
                _ => "#fa541c",
            })
            .unwrap(),
        )
    }
    pub fn success(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#DDF8D8",
                100 => "#BEEFBB",
                200 => "#80DA88",
                300 => "#44C265",
                400 => "#1AA64A",
                500 => "#128937",
                600 => "#006C35",
                700 => "#00522C",
                800 => "#00381F",
                900 => "#002110",
                _ => "#128937",
            })
            .unwrap(),
        )
    }
}

impl FromStr for Theme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            DARK => Ok(Theme::Dark),
            PRIMARY => Ok(Theme::Primary),
            ERROR => Ok(Theme::Error),
            WARNING => Ok(Theme::Warning),
            SUCCESS => Ok(Theme::Success),
            INFO => Ok(Theme::Info),
            _ => Err(Error::ThemeStyleParse(format!(
                "Unknown theme style: {}",
                s
            ))),
        }
    }
}

impl TryFrom<&Item> for Theme {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, <Theme as TryFrom<&Item>>::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&Value> for Theme {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, <Theme as TryFrom<&Value>>::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<Option<&str>> for Theme {
    type Error = Error;

    fn try_from(value: Option<&str>) -> Result<Self, <Theme as TryFrom<&Value>>::Error> {
        value
            .ok_or(Error::ThemeStyleParse(
                "[global.theme] should be a string".to_string(),
            ))?
            .parse()
    }
}

impl From<Theme> for Value {
    fn from(value: Theme) -> Self {
        Value::String(Formatted::new(
            match value {
                Theme::Dark => DARK,
                Theme::Primary => PRIMARY,
                Theme::Error => ERROR,
                Theme::Warning => WARNING,
                Theme::Success => SUCCESS,
                Theme::Info => INFO,
            }
            .to_string(),
        ))
    }
}


impl ToTomlValue for Theme {
    fn to_toml_value(&self) -> Value {
        Value::from(*self)
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Value::from(*self).to_string().as_str())
    }
}
