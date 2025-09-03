use makepad_widgets::{Flow, LiveValue};
use toml_edit::{Formatted, Value};

use crate::prop::{
    manuel::{DOWN, OVERLAY, RIGHT_UP, RIGHT_WRAP},
    traits::{FromLiveValue, ToTomlValue},
};

impl FromLiveValue for Flow {
    fn from_live_value(v: &LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let LiveValue::BareEnum(e) = v {
            match e.to_string().as_str() {
                DOWN => Some(Flow::Down),
                OVERLAY => Some(Flow::Overlay),
                RIGHT_UP => Some(Flow::Right),
                RIGHT_WRAP => Some(Flow::RightWrap),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl ToTomlValue for Flow {
    fn to_toml_value(&self) -> toml_edit::Value {
        Value::String(Formatted::new(
            match self {
                Flow::Down => DOWN,
                Flow::Overlay => OVERLAY,
                Flow::Right => RIGHT_UP,
                Flow::RightWrap => RIGHT_WRAP,
            }
            .to_string(),
        ))
    }
}
