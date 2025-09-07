use makepad_widgets::image_cache::ImageFit;

use crate::prop::{
    manuel::{BIGGEST, HORIZONTAL, SIZE, SMALLEST, STRETCH, VERTICAL},
    traits::{FromLiveValue, ToTomlValue},
};

impl FromLiveValue for ImageFit {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        match v {
            makepad_widgets::LiveValue::BareEnum(enm) => match enm.to_string().as_str() {
                BIGGEST => Some(ImageFit::Biggest),
                HORIZONTAL => Some(ImageFit::Horizontal),
                SIZE => Some(ImageFit::Size),
                SMALLEST => Some(ImageFit::Smallest),
                STRETCH => Some(ImageFit::Stretch),
                VERTICAL => Some(ImageFit::Vertical),
                _ => None,
            },
            _ => None,
        }
    }
}

impl ToTomlValue for ImageFit {
    fn to_toml_value(&self) -> toml_edit::Value {
        match self {
            ImageFit::Biggest => BIGGEST,
            ImageFit::Horizontal => HORIZONTAL,
            ImageFit::Size => SIZE,
            ImageFit::Smallest => SMALLEST,
            ImageFit::Stretch => STRETCH,
            ImageFit::Vertical => VERTICAL,
        }
        .to_string()
        .to_toml_value()
    }
}