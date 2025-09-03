use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Error {
    /// called when icon type cannot be transformed to target type. (In GIcon)
    IconTypeTransfom,
    /// called when widget height is fixed and bigger than max height or smaller than min height.
    ConflictHeight,
    /// called when widget width is fixed and bigger than max width or smaller than min width.
    ConflictWidth,
    /// can not load theme style file
    ThemeStyleFileLoad(String),
    ThemeStyleParse(String),
    InvalidPart{
        from: String,
        to: String
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IconTypeTransfom => f.write_str(
                "Cannot transform icon type to target type. You may use the non-exist icon type.",
            ),
            Error::ConflictHeight => f.write_str(
                "Widget height is fixed and bigger than max height or smaller than min height.",
            ),
            Error::ConflictWidth => f.write_str(
                "Widget width is fixed and bigger than max width or smaller than min width.",
            ),
            Error::ThemeStyleFileLoad(e) => {
                f.write_fmt(format_args!("Cannot load theme style file: {}", e))
            }
            Error::ThemeStyleParse(e) => {
                f.write_fmt(format_args!("Cannot parse theme style file: {}", e))
            }
            Error::InvalidPart { from, to } => {
                f.write_fmt(format_args!("Invalid part conversion from {} to {}", from, to))
            }
        }
    }
}
