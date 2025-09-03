use makepad_widgets::Size;

use crate::prop::traits::FromLiveValue;

impl FromLiveValue for Size {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        match v {
            makepad_widgets::LiveValue::Float64(num) => Some(Size::Fixed(*num)),
            makepad_widgets::LiveValue::Float32(num) => Some(Size::Fixed(*num as f64)),
            makepad_widgets::LiveValue::BareEnum(e) => match e.to_string().as_str() {
                "Fill" => Some(Size::Fill),
                "All" => Some(Size::All),
                "Fit" => Some(Size::Fit),
                _ => None,
            },
            _ => None,
        }
    }
}
