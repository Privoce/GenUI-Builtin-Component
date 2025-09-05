use crate::prop::traits::{FromLiveValue, ToTomlValue};

impl FromLiveValue for bool {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized {
        if let makepad_widgets::LiveValue::Bool(b) = v {
            Some(*b)
        } else {
            None
        }
    }
}

impl ToTomlValue for bool {
    fn to_toml_value(&self) -> toml_edit::Value {
        toml_edit::Value::Boolean(toml_edit::Formatted::new(*self))
    }
}