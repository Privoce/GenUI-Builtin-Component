mod color;
mod components;
mod global;
mod hex;
mod style;

use std::env::current_exe;

pub use global::ThemeGlobal;
use toml_edit::DocumentMut;

use crate::error::GError;
// pub use color::ThemeColor;
// pub use style::ThemeStyle;

/// # Theme Handler
/// genui theme handler is used to load and map theme configurations
/// we have default theme and style, but if user want to define their own theme
/// this struct will load the config and map to builtin components
///
/// this struct will be stored in Cx, which can be easy to use in user side too.
///
#[derive(Debug, Clone, Default)]
pub struct ThemeHandler {
    pub global: ThemeGlobal,
    // theme: ThemeColor,
    // style: ThemeStyle,
}

impl ThemeHandler {
    pub fn load() -> Self {
        let path = current_exe()
            .map_err(|e| GError::ThemeStyleFileLoad(e.to_string()))
            .unwrap()
            .join("theme_style.toml");

        if path.exists() {
            // use toml edit to parse
            let content = std::fs::read_to_string(path)
                .map_err(|e| GError::ThemeStyleFileLoad(e.to_string()))
                .unwrap();
            content.parse::<DocumentMut>().unwrap().try_into().unwrap()
        } else {
            ThemeHandler::default()
        }
    }
}

impl TryFrom<DocumentMut> for ThemeHandler {
    type Error = GError;

    fn try_from(value: DocumentMut) -> Result<Self, Self::Error> {
        let global = value.get("global").map_or_else(
            || Ok(ThemeGlobal::default()),
            |item| item.try_into(),
        )?;

        Ok(ThemeHandler { global })
    }
}
