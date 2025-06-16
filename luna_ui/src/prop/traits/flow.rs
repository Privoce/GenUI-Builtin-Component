use makepad_widgets::{Flow, LiveValue};

use crate::prop::traits::FromLiveValue;

impl FromLiveValue for Flow {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Self {
        if let LiveValue::BareEnum(e) = v {
            match e.to_string().as_str() {
                "Down" => Flow::Down,
                "Overlay" => Flow::Overlay,
                "Right" => Flow::Right,
                "RightWrap" => Flow::RightWrap,
                _ => Flow::default(),
            }
        } else {
            Flow::default()
        }
    }
}
