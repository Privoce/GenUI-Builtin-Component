pub mod conf;
mod color;

use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

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
        value
            .as_str()
            .ok_or(Error::ThemeStyleParse(
                "[global.theme] should be a string".to_string(),
            ))?
            .parse()
    }
}
