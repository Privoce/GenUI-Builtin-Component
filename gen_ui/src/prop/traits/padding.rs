use crate::prop::{
    manuel::{BOTTOM, LEFT, RIGHT, TOP},
    traits::{FromLiveValue, NewFrom, ToTomlValue},
};
use makepad_widgets::Padding;
use toml_edit::{Formatted, InlineTable, Value};

impl NewFrom for Padding {
    fn from_f64(uni: f64) -> Self {
        Padding {
            top: uni,
            right: uni,
            bottom: uni,
            left: uni,
        }
    }

    fn from_xy(x: f64, y: f64) -> Self {
        Padding {
            top: x,
            right: y,
            bottom: x,
            left: y,
        }
    }

    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self {
        Padding {
            top: x,
            right: y,
            bottom: z,
            left: w,
        }
    }
}

impl FromLiveValue for Padding {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let makepad_widgets::LiveValue::Vec4(vec4) = v {
            Some(Padding::from_vec4(vec4))
        } else {
            None
        }
    }
}

impl ToTomlValue for Padding {
    fn to_toml_value(&self) -> toml_edit::Value {
        let mut inline_table = InlineTable::new();
        inline_table.insert(TOP, Value::Float(Formatted::new(self.top)));
        inline_table.insert(RIGHT, Value::Float(Formatted::new(self.right)));
        inline_table.insert(BOTTOM, Value::Float(Formatted::new(self.bottom)));
        inline_table.insert(LEFT, Value::Float(Formatted::new(self.left)));
        Value::InlineTable(inline_table)
    }
}
