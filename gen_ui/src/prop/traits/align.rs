use crate::prop::traits::FromLiveValue;
use makepad_widgets::{Align, Vec2, Vec4};

use super::NewFrom;

impl NewFrom for Align {
    fn from_f64(uni: f64) -> Self {
        Align { x: uni, y: uni }
    }

    fn from_xy(x: f64, y: f64) -> Self {
        Align { x, y }
    }
    fn from_all(x: f64, y: f64, _z: f64, _w: f64) -> Self {
        Align { x, y }
    }
}

impl FromLiveValue for Align {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        match v {
            makepad_widgets::LiveValue::Float64(align) => Some(Align::from_f64(*align)),
            makepad_widgets::LiveValue::Vec2(Vec2 { x, y }) => {
                Some(Align::from_xy(*x as f64, *y as f64))
            }
            makepad_widgets::LiveValue::Vec4(Vec4 { x, y, z, w }) => {
                Some(Align::from_all(*x as f64, *y as f64, *z as f64, *w as f64))
            }
            _ => None,
        }
    }
}
