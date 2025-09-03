pub mod components;
pub mod conf;
mod global;
mod theme;

pub use global::*;
use makepad_widgets::{
    image_cache::ImageFit, Align, DVec2, Flow, Margin, MouseCursor, Padding, Size, Vec2,
};
pub use theme::*;
use toml_edit::Value;

use crate::{
    error::Error,
    prop::{
        manuel::{BIGGEST, HORIZONTAL, SIZE, SMALLEST, STRETCH, VERTICAL},
        traits::ToCursor,
    },
};

pub trait TomlValueTo {
    fn to_f32(&self) -> Result<f32, Error>;
    fn to_f64(&self) -> Result<f64, Error>;
    fn to_vec2(&self, default: Vec2) -> Result<Vec2, Error>;
    fn to_margin(&self, default: Margin) -> Result<Margin, Error>;
    fn to_padding(&self, default: Padding) -> Result<Padding, Error>;
    fn to_flow(&self) -> Result<Flow, Error>;
    fn to_bool(&self) -> Result<bool, Error>;
    fn to_align(&self, default: Align) -> Result<Align, Error>;
    fn to_size(&self) -> Result<Size, Error>;
    fn to_cursor(&self) -> Result<MouseCursor, Error>;
    fn to_dvec2(&self) -> Result<DVec2, Error>;
    fn to_image_fit(&self) -> Result<ImageFit, Error>;
}

impl TomlValueTo for Value {
    fn to_f32(&self) -> Result<f32, Error> {
        self.as_float()
            .ok_or(Error::ThemeStyleParse("Expected a float value".to_string()))
            .map(|v| v as f32)
    }
    fn to_f64(&self) -> Result<f64, Error> {
        self.as_float()
            .ok_or(Error::ThemeStyleParse("Expected a float value".to_string()))
    }
    fn to_vec2(&self, default: Vec2) -> Result<Vec2, Error> {
        let inline_table = self.as_inline_table().ok_or(Error::ThemeStyleParse(
            "Vec2 should be a inline table".to_string(),
        ))?;

        let x = inline_table
            .get("x")
            .map_or_else(|| Ok(default.x), |item| item.to_f32())?;
        let y = inline_table
            .get("y")
            .map_or_else(|| Ok(default.y), |item| item.to_f32())?;

        Ok(Vec2 { x, y })
    }

    fn to_margin(&self, default: Margin) -> Result<Margin, Error> {
        let inline_table = self
            .as_inline_table()
            .ok_or(Error::ThemeStyleParse(
                "Margin should be a inline table".to_string(),
            ))
            .unwrap();

        let top = inline_table
            .get("top")
            .map_or_else(|| Ok(default.top), |item| item.to_f64())?;
        let right = inline_table
            .get("right")
            .map_or_else(|| Ok(default.right), |item| item.to_f64())?;
        let bottom = inline_table
            .get("bottom")
            .map_or_else(|| Ok(default.bottom), |item| item.to_f64())?;
        let left = inline_table
            .get("left")
            .map_or_else(|| Ok(default.left), |item| item.to_f64())?;

        Ok(Margin {
            top,
            right,
            bottom,
            left,
        })
    }

    fn to_padding(&self, default: Padding) -> Result<Padding, Error> {
        let inline_table = self
            .as_inline_table()
            .ok_or(Error::ThemeStyleParse(
                "Padding should be a inline table".to_string(),
            ))
            .unwrap();

        let top = inline_table
            .get("top")
            .map_or_else(|| Ok(default.top), |item| item.to_f64())?;
        let right = inline_table
            .get("right")
            .map_or_else(|| Ok(default.right), |item| item.to_f64())?;
        let bottom = inline_table
            .get("bottom")
            .map_or_else(|| Ok(default.bottom), |item| item.to_f64())?;
        let left = inline_table
            .get("left")
            .map_or_else(|| Ok(default.left), |item| item.to_f64())?;

        Ok(Padding {
            top,
            right,
            bottom,
            left,
        })
    }

    fn to_flow(&self) -> Result<Flow, Error> {
        let flow_str = self.as_str().ok_or(Error::ThemeStyleParse(
            "Expected a string value for Flow".to_string(),
        ))?;

        Ok(match flow_str {
            "Right" => Flow::Right,
            "Down" => Flow::Down,
            "Overlay" => Flow::Overlay,
            "RightWrap" => Flow::RightWrap,
            _ => return Err(Error::ThemeStyleParse("Invalid Flow value".to_string())),
        })
    }

    fn to_bool(&self) -> Result<bool, Error> {
        self.as_bool().ok_or(Error::ThemeStyleParse(
            "Expected a boolean value".to_string(),
        ))
    }

    fn to_align(&self, default: Align) -> Result<Align, Error> {
        let inline_table = self.as_inline_table().ok_or(Error::ThemeStyleParse(
            "Align should be a inline table".to_string(),
        ))?;
        let x = inline_table
            .get("x")
            .map_or_else(|| Ok(default.x), |item| item.to_f64())?;

        let y = inline_table
            .get("y")
            .map_or_else(|| Ok(default.y), |item| item.to_f64())?;

        Ok(Align { x, y })
    }

    fn to_size(&self) -> Result<Size, Error> {
        if let Some(size_str) = self.as_str() {
            Ok(match size_str {
                "Fill" => Size::Fill,
                "Fit" => Size::Fit,
                "All" => Size::All,
                _ => {
                    return Err(Error::ThemeStyleParse(
                        "Size should be Fill, Fit, All or a float value".to_string(),
                    ));
                }
            })
        } else {
            self.to_f64().map_or_else(
                |_| {
                    Err(Error::ThemeStyleParse(
                        "Size should be Fill, Fit, All or a float value".to_string(),
                    ))
                },
                |v| Ok(Size::Fixed(v)),
            )
        }
    }

    fn to_cursor(&self) -> Result<MouseCursor, Error> {
        return if let Some(cursor_str) = self.as_str() {
            Ok(MouseCursor::from_str(cursor_str))
        } else {
            Err(Error::ThemeStyleParse(
                "Expected a string value for MouseCursor".to_string(),
            ))
        };
    }

    fn to_dvec2(&self) -> Result<DVec2, Error> {
        let inline_table = self.as_inline_table().ok_or(Error::ThemeStyleParse(
            "DVec2 should be a inline table".to_string(),
        ))?;

        let x = inline_table
            .get("x")
            .map_or_else(|| Ok(0.0), |item| item.to_f64())?;
        let y = inline_table
            .get("y")
            .map_or_else(|| Ok(0.0), |item| item.to_f64())?;

        Ok(DVec2 { x, y })
    }

    fn to_image_fit(&self) -> Result<ImageFit, Error> {
        let fit_str = self.as_str().ok_or(Error::ThemeStyleParse(
            "Expected a string value for ImageFit".to_string(),
        ))?;
        match fit_str {
            BIGGEST => Ok(ImageFit::Biggest),
            HORIZONTAL => Ok(ImageFit::Horizontal),
            SIZE => Ok(ImageFit::Size),
            SMALLEST => Ok(ImageFit::Smallest),
            STRETCH => Ok(ImageFit::Stretch),
            VERTICAL => Ok(ImageFit::Vertical),
            _ => Err(Error::ThemeStyleParse(
                "ImageFit should be Contain, Cover, Fill or None".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::conf::Conf;

    #[test]
    fn toml_conf() {
        let path = "/Users/shengyifei/projects/gen_ui/components/genui.theme.example.toml";
        let conf = Conf::default();
        let example_toml = PathBuf::from(path);
        // write to example toml
        fs::write(example_toml, conf.to_string()).unwrap();
    }
}
