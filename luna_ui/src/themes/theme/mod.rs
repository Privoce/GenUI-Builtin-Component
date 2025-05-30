mod color;
pub mod conf;

use std::str::FromStr;

pub use color::*;
use makepad_widgets::*;
use toml_edit::{Item, Value};

use crate::error::Error;

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
                50 => "#F7EBFF",
                100 => "#EACFFF",
                200 => "#DDB5FF",
                300 => "#CB96FF",
                400 => "#B382F0",
                500 => "#9E6CD8",
                600 => "#8755C2",
                700 => "#7141AC",
                800 => "#5A2D96",
                900 => "#451981",
                _ => "#B382F0",
            })
            .unwrap(),
        )
    }
    pub fn dark(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#dcdde1",
                100 => "#a2a6b1",
                200 => "#858a99",
                300 => "#6f7686",
                400 => "#535d6d",
                500 => "#424a57",
                600 => "#323843",
                700 => "#272b34",
                800 => "#20232b",
                900 => "#15181d",
                _ => "#000000",
            })
            .unwrap(),
        )
    }
    pub fn info(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#ABCAFF",
                100 => "#8CB8FF",
                200 => "#69A1FF",
                300 => "#478DFF",
                400 => "#2174FF",
                500 => "#0957D9",
                600 => "#084DCD",
                700 => "#073AB5",
                800 => "#062E9A",
                900 => "#1E2C60",
                _ => "#2174FF",
            })
            .unwrap(),
        )
    }
    pub fn error(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#FFF2F2",
                100 => "#FFD6D8",
                200 => "#FFB5B8",
                300 => "#FF9195",
                400 => "#FB6E77",
                500 => "#E35661",
                600 => "#C9384A",
                700 => "#B01C37",
                800 => "#960627",
                900 => "#730524",
                _ => "#FB6E77",
            })
            .unwrap(),
        )
    }
    pub fn warning(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#FFF4E5",
                100 => "#FFD8AD",
                200 => "#FFB97D",
                300 => "#FF9852",
                400 => "#ED8139",
                500 => "#D66724",
                600 => "#C25110",
                700 => "#A24006",
                800 => "#873105",
                900 => "#692204",
                _ => "#ED8139",
            })
            .unwrap(),
        )
    }
    pub fn success(level: u32) -> Color {
        Color::Hex(
            Hex::from_str(match level {
                50 => "#E8F7F1",
                100 => "#B3E8D1",
                200 => "#71D5AE",
                300 => "#37BF8E",
                400 => "#07A872",
                500 => "#06935F",
                600 => "#057E4C",
                700 => "#046939",
                800 => "#035428",
                900 => "#034116",
                _ => "#07A872",
            })
            .unwrap(),
        )
    }
}

impl FromStr for Theme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dark" => Ok(Theme::Dark),
            "primary" => Ok(Theme::Primary),
            "error" => Ok(Theme::Error),
            "warning" => Ok(Theme::Warning),
            "success" => Ok(Theme::Success),
            "info" => Ok(Theme::Info),
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
