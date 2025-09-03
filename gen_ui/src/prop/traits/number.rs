use crate::prop::traits::FromLiveValue;
use makepad_widgets::LiveValue;
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

