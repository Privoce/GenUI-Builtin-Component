use crate::{
    prop::traits::{FromLiveColor, FromLiveValue, ToColor, ToTomlValue, ToU32},
    themes::{Color, Hex},
};

use super::{ToBool, ToFloat, ToVec};
use makepad_widgets::{vec2, vec3, vec4, DVec2, Vec2, Vec3, Vec4};

impl ToBool for f32 {
    fn to_bool(&self) -> bool {
        *self != 0.0
    }
}

impl ToFloat for bool {
    fn to_f32(&self) -> f32 {
        *self as u8 as f32
    }
    fn to_f64(&self) -> f64 {
        *self as u8 as f64
    }
}

impl ToVec for f64 {
    fn to_vec2(self) -> Vec2 {
        vec2(self as f32, self as f32)
    }

    fn to_vec3(self) -> Vec3 {
        vec3(self as f32, self as f32, self as f32)
    }

    fn to_vec4(self) -> Vec4 {
        vec4(self as f32, self as f32, self as f32, self as f32)
    }
}

impl ToVec for f32 {
    fn to_vec2(self) -> Vec2 {
        vec2(self, self)
    }

    fn to_vec3(self) -> Vec3 {
        vec3(self, self, self)
    }

    fn to_vec4(self) -> Vec4 {
        vec4(self, self, self, self)
    }
}

impl ToU32 for Vec4 {
    fn to_u32(self) -> u32 {
        (((self.x * 255.0) as u32) << 24)
            | (((self.y * 255.0) as u32) << 16)
            | (((self.z * 255.0) as u32) << 8)
            | ((self.w * 255.0) as u32)
    }
}

impl FromLiveValue for Vec4 {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let makepad_widgets::LiveValue::Vec4(vec4) = v {
            Some(*vec4)
        } else {
            None
        }
    }
}

impl FromLiveColor for Vec4 {
    fn from_live_color(v: &makepad_widgets::LiveValue) -> Option<Vec4>
    where
        Self: Sized,
    {
        if let makepad_widgets::LiveValue::Color(color) = v {
            Some(Vec4::from_u32(*color))
        } else {
            None
        }
    }
}

impl FromLiveValue for Vec2 {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let makepad_widgets::LiveValue::Vec2(vec2) = v {
            Some(*vec2)
        } else {
            None
        }
    }
}

impl ToColor for Vec4 {
    fn to_color(self) -> crate::themes::Color {
        Color::Hex(self.into())
    }

    fn to_hex_string(self) -> String {
        self.to_color().to_string()
    }

    fn from_hex(s: &str) -> Result<Self, crate::error::Error>
    where
        Self: Sized,
    {
        let hex: Hex = s.parse()?;
        Ok(Color::Hex(hex).into())
    }
}

impl FromLiveValue for DVec2 {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let makepad_widgets::LiveValue::Vec2(vec2) = v {
            Some((*vec2).into())
        } else {
            None
        }
    }
}

impl ToTomlValue for Vec2 {
    fn to_toml_value(&self) -> toml_edit::Value {
        let mut inline_table = toml_edit::InlineTable::new();
        inline_table.insert("x", self.x.to_toml_value());
        inline_table.insert("y", self.y.to_toml_value());
        toml_edit::Value::InlineTable(inline_table)
    }
}

impl ToTomlValue for Vec3 {
    fn to_toml_value(&self) -> toml_edit::Value {
        let mut inline_table = toml_edit::InlineTable::new();
        inline_table.insert("x", self.x.to_toml_value());
        inline_table.insert("y", self.y.to_toml_value());
        inline_table.insert("z", self.z.to_toml_value());
        toml_edit::Value::InlineTable(inline_table)
    }
}

impl ToTomlValue for Vec4 {
    fn to_toml_value(&self) -> toml_edit::Value {
        let mut inline_table = toml_edit::InlineTable::new();
        inline_table.insert("x", self.x.to_toml_value());
        inline_table.insert("y", self.y.to_toml_value());
        inline_table.insert("z", self.z.to_toml_value());
        inline_table.insert("w", self.w.to_toml_value());
        toml_edit::Value::InlineTable(inline_table)
    }
}

pub type AbsPos = Option<DVec2>;

impl ToTomlValue for AbsPos {
    fn to_toml_value(&self) -> toml_edit::Value {
        match self {
            Some(dvec2) => format!("vec2({}, {})", dvec2.x, dvec2.y).to_toml_value(),
            None => "None".to_string().to_toml_value(),
        }
    }
}
