use crate::prop::traits::FromLiveValue;

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