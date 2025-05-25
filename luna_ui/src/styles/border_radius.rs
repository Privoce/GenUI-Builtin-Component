use makepad_widgets::*;
use toml_edit::{Item, Value};

use crate::{error::Error, themes::TomlValueTo};

#[derive(Clone, Copy, Debug, Live, LiveRegister, LiveHook)]
#[live_ignore]
pub struct BorderRadius {
    #[live]
    pub top: f32,
    #[live]
    pub right: f32,
    #[live]
    pub bottom: f32,
    #[live]
    pub left: f32,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self::new(8.0)
    }
}

impl BorderRadius {
    pub fn new(radius: f32) -> Self {
        Self {
            top: radius,
            right: radius,
            bottom: radius,
            left: radius,
        }
    }
}

impl TryFrom<&Value> for BorderRadius {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "Border radius should be a inline table".to_string(),
        ))?;

        let top = inline_table.get("top").map_or(8.0, |item| item.to_f32(8.0));

        let right = inline_table
            .get("right")
            .map_or(8.0, |item| item.to_f32(8.0));

        let bottom = inline_table
            .get("bottom")
            .map_or(8.0, |item| item.to_f32(8.0));
        let left = inline_table
            .get("left")
            .map_or(8.0, |item| item.to_f32(8.0));
        Ok(BorderRadius {
            top,
            right,
            bottom,
            left,
        })
    }
}
