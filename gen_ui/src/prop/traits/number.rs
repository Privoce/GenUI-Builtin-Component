use crate::prop::traits::{FromLiveValue, ToTomlValue};
use makepad_widgets::LiveValue;
use toml_edit::Value;

impl FromLiveValue for f32 {
    fn from_live_value(v: &LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        match v {
            LiveValue::Float64(num) => Some(*num as f32),
            LiveValue::Float32(num) => Some(*num),
            _ => None
        }
    }
}

impl FromLiveValue for f64 {
    fn from_live_value(v: &LiveValue) -> Option<Self>
    where
        Self: Sized {
        match v {
            LiveValue::Float64(num) => Some(*num),
            LiveValue::Float32(num) => Some(*num as f64),
            _ => None
        }
    }
}

impl ToTomlValue for f64 {
    fn to_toml_value(&self) -> Value {
        Value::Float(toml_edit::Formatted::new(*self))
    }
}

impl ToTomlValue for f32 {
    fn to_toml_value(&self) -> Value {
        Value::Float(toml_edit::Formatted::new(*self as f64))
    }
}

impl ToTomlValue for usize {
    fn to_toml_value(&self) -> Value {
        Value::Integer(toml_edit::Formatted::new(*self as i64))
    }
}