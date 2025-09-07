use makepad_widgets::Size;
use toml_edit::Value;
use crate::prop::{
    manuel::{ALL, FILL, FIT_UP},
    traits::{FromLiveValue, ToTomlValue},
};

impl FromLiveValue for Size {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        match v {
            makepad_widgets::LiveValue::Float64(num) => Some(Size::Fixed(*num)),
            makepad_widgets::LiveValue::Float32(num) => Some(Size::Fixed(*num as f64)),
            makepad_widgets::LiveValue::BareEnum(e) => match e.to_string().as_str() {
                FILL => Some(Size::Fill),
                ALL => Some(Size::All),
                FIT_UP => Some(Size::Fit),
                _ => None,
            },
            _ => None,
        }
    }
}

impl ToTomlValue for Size {
    fn to_toml_value(&self) -> toml_edit::Value {
        match self {
            Size::Fixed(num) => Value::Float(toml_edit::Formatted::new(*num)),
            Size::Fill => Value::String(toml_edit::Formatted::new(FILL.to_string())),
            Size::All => Value::String(toml_edit::Formatted::new(ALL.to_string())),
            Size::Fit => Value::String(toml_edit::Formatted::new(FIT_UP.to_string())),
        }
    }
}

