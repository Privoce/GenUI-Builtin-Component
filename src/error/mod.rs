use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum GError {
    /// called when icon type cannot be transformed to target type. (In GIcon)
    IconTypeTransfom,
    /// called when widget height is fixed and bigger than max height or smaller than min height.
    ConflictHeight,
    /// called when widget width is fixed and bigger than max width or smaller than min width.
    ConflictWidth,
    /// can not load theme style file
    ThemeStyleFileLoad(String),
    ThemeStyleParse(String),
}

impl Error for GError {}

impl Display for GError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GError::IconTypeTransfom => f.write_str(
                "Cannot transform icon type to target type. You may use the non-exist icon type.",
            ),
            GError::ConflictHeight => f.write_str(
                "Widget height is fixed and bigger than max height or smaller than min height.",
            ),
            GError::ConflictWidth => f.write_str(
                "Widget width is fixed and bigger than max width or smaller than min width.",
            ),
            GError::ThemeStyleFileLoad(e) => {
                f.write_fmt(format_args!("Cannot load theme style file: {}", e))
            }
            GError::ThemeStyleParse(e) => {
                f.write_fmt(format_args!("Cannot parse theme style file: {}", e))
            }
        }
    }
}
