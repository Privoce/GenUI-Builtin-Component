use crate::prop::{
    manuel::{BOTTOM, LEFT, RIGHT, TOP},
    traits::{FromLiveValue, ToTomlValue},
};

use super::NewFrom;
use makepad_widgets::Margin;
use toml_edit::{Formatted, InlineTable, Value};

impl NewFrom for Margin {
    fn from_f64(uni: f64) -> Self {
        Margin {
            top: uni,
            right: uni,
            bottom: uni,
            left: uni,
        }
    }

    fn from_xy(x: f64, y: f64) -> Self {
        Margin {
            top: x,
            right: y,
            bottom: x,
            left: y,
        }
    }

    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self {
        Margin {
            top: x,
            right: y,
            bottom: z,
            left: w,
        }
    }
}

impl FromLiveValue for Margin {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let makepad_widgets::LiveValue::Vec4(vec4) = v {
            Some(Margin::from_vec4(vec4))
        } else {
            None
        }
    }
}

impl ToTomlValue for Margin {
    fn to_toml_value(&self) -> toml_edit::Value {
        let mut inline_table = InlineTable::new();
        inline_table.insert(TOP, Value::Float(Formatted::new(self.top)));
        inline_table.insert(RIGHT, Value::Float(Formatted::new(self.right)));
        inline_table.insert(BOTTOM, Value::Float(Formatted::new(self.bottom)));
        inline_table.insert(LEFT, Value::Float(Formatted::new(self.left)));
        Value::InlineTable(inline_table)
    }
}
