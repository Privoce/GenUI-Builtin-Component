mod components;
mod conf;
mod global;
mod theme;

pub use components::*;
pub use conf::*;
pub use global::*;
use makepad_widgets::{Margin, Padding, Vec2};
pub use theme::*;
use toml_edit::Value;

use crate::error::Error;

pub trait TomlValueTo {
    fn to_f32(&self, default: f32) -> f32;
    fn to_f64(&self, default: f64) -> f64;
    fn to_vec2(&self, default: Vec2) -> Vec2;
    fn to_margin(&self, default: Margin) -> Margin;
    fn to_padding(&self, default: Padding) -> Padding;
}

impl TomlValueTo for Value {
    fn to_f32(&self, default: f32) -> f32 {
        self.as_float().map_or_else(|| default, |v| v as f32)
    }
    fn to_f64(&self, default: f64) -> f64 {
        self.as_float().map_or_else(|| default, |v| v as f64)
    }
    fn to_vec2(&self, default: Vec2) -> Vec2 {
        let inline_table = self
            .as_inline_table()
            .ok_or(Error::ThemeStyleParse(
                "Vec2 should be a inline table".to_string(),
            ))
            .unwrap();

        let x = inline_table
            .get("x")
            .map_or(default.x, |item| item.to_f32(default.x));
        let y = inline_table
            .get("y")
            .map_or(default.y, |item| item.to_f32(default.y));

        Vec2 { x, y }
    }

    fn to_margin(&self, default: Margin) -> Margin {
        let inline_table = self
            .as_inline_table()
            .ok_or(Error::ThemeStyleParse(
                "Margin should be a inline table".to_string(),
            ))
            .unwrap();

        let top = inline_table
            .get("top")
            .map_or(default.top, |item| item.to_f64(default.top));
        let right = inline_table
            .get("right")
            .map_or(default.right, |item| item.to_f64(default.right));
        let bottom = inline_table
            .get("bottom")
            .map_or(default.bottom, |item| item.to_f64(default.bottom));
        let left = inline_table
            .get("left")
            .map_or(default.left, |item| item.to_f64(default.left));

        Margin {
            top,
            right,
            bottom,
            left,
        }
    }

    fn to_padding(&self, default: Padding) -> Padding {
        let inline_table = self
            .as_inline_table()
            .ok_or(Error::ThemeStyleParse(
                "Padding should be a inline table".to_string(),
            ))
            .unwrap();

        let top = inline_table
            .get("top")
            .map_or(default.top, |item| item.to_f64(default.top));
        let right = inline_table
            .get("right")
            .map_or(default.right, |item| item.to_f64(default.right));
        let bottom = inline_table
            .get("bottom")
            .map_or(default.bottom, |item| item.to_f64(default.bottom));
        let left = inline_table
            .get("left")
            .map_or(default.left, |item| item.to_f64(default.left));

        Padding {
            top,
            right,
            bottom,
            left,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::Conf;

    #[test]
    fn toml_conf() {
        let conf = Conf::default();
        
        dbg!(conf);
    }
}