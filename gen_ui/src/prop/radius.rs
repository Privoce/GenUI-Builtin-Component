use makepad_widgets::*;
use toml_edit::Value;

use crate::{error::Error, prop::traits::{FromLiveValue, NewFrom, ToTomlValue}, themes::TomlValueTo};

/// ## Radius
/// Radius always use in:
/// - `border_radius`
/// ### Transform
/// Radius can be transformed into a `Vec4` where:
/// - `top` becomes `x`
/// - `right` becomes `y`
/// - `bottom` becomes `z`
/// - `left` becomes `w`
#[derive(Clone, Copy, Debug, Live, LiveRegister, LiveHook)]
#[live_ignore]
pub struct Radius {
    #[live]
    pub top: f32,
    #[live]
    pub right: f32,
    #[live]
    pub bottom: f32,
    #[live]
    pub left: f32,
}

impl ToTomlValue for Radius {
    fn to_toml_value(&self) -> Value {
        let mut inline_table = toml_edit::InlineTable::new();
        inline_table.insert("top", self.top.to_toml_value());
        inline_table.insert("right", self.right.to_toml_value());
        inline_table.insert("bottom", self.bottom.to_toml_value());
        inline_table.insert("left", self.left.to_toml_value());
        Value::InlineTable(inline_table)
    }
}

impl ToLiveValue for Radius {
    fn to_live_value(&self) -> LiveValue {
        LiveValue::Vec4((*self).into())
    }
}

impl Default for Radius {
    fn default() -> Self {
        Self::new(8.0)
    }
}

impl NewFrom for Radius {
    fn from_f64(uni: f64) -> Self {
        Self::new(uni as f32)
    }

    fn from_xy(x: f64, y: f64) -> Self {
        Self {
            top: x as f32,
            right: y as f32,
            bottom: x as f32,
            left: y as f32,
        }
    }

    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            top: x as f32,
            right: y as f32,
            bottom: z as f32,
            left: w as f32,
        }
    }
}

impl Radius {
    pub fn new(radius: f32) -> Self {
        Self {
            top: radius,
            right: radius,
            bottom: radius,
            left: radius,
        }
    }
}

impl TryFrom<&Value> for Radius {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "radius should be a inline table".to_string(),
        ))?;

        let top = inline_table
            .get("top")
            .map_or(Ok(8.0), |item| item.to_f32())?;

        let right = inline_table
            .get("right")
            .map_or(Ok(8.0), |item| item.to_f32())?;

        let bottom = inline_table
            .get("bottom")
            .map_or(Ok(8.0), |item| item.to_f32())?;

        let left = inline_table
            .get("left")
            .map_or(Ok(8.0), |item| item.to_f32())?;

        Ok(Radius {
            top,
            right,
            bottom,
            left,
        })
    }
}

impl From<Radius> for Vec4 {
    fn from(value: Radius) -> Self {
        Vec4 {
            x: value.top,
            y: value.right,
            z: value.bottom,
            w: value.left,
        }
    }
}

impl From<&Vec4> for Radius {
    fn from(value: &Vec4) -> Self {
        Radius {
            top: value.x,
            right: value.y,
            bottom: value.z,
            left: value.w,
        }
    }
}

impl FromLiveValue for Radius {
    fn from_live_value(v: &LiveValue) -> Option<Self>
    where
        Self: Sized {
        if let LiveValue::Vec4(vec4) = v {
            Some(Radius::from(vec4))
        } else {
            None
        }
    }
}