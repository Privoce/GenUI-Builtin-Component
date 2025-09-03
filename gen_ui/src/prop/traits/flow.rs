use makepad_widgets::{Flow, LiveValue};

use crate::prop::traits::FromLiveValue;

impl FromLiveValue for Flow {
    fn from_live_value(v: &LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let LiveValue::BareEnum(e) = v {
            
            match e.to_string().as_str() {
                "Down" => Some(Flow::Down),
                "Overlay" => Some(Flow::Overlay),
                "Right" => Some(Flow::Right),
                "RightWrap" => Some(Flow::RightWrap),
                _ => None,
            }
        } else {
            None
        }
    }
}
