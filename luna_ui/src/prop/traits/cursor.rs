use makepad_widgets::{MouseCursor};

use crate::prop::traits::FromLiveValue;

use super::ToCursor;

impl ToCursor for MouseCursor {
    fn from_str(s: &str) -> Self {
        match s {
            "default" => MouseCursor::Default,
            "hand" => MouseCursor::Hand,
            "text" => MouseCursor::Text,
            "move" => MouseCursor::Move,
            "wait" => MouseCursor::Wait,
            "help" => MouseCursor::Help,
            "not-allowed" => MouseCursor::NotAllowed,
            "crosshair" => MouseCursor::Crosshair,
            "grab" => MouseCursor::Grab,
            "grabbing" => MouseCursor::Grabbing,
            _ => MouseCursor::Default,
        }
    }
}

impl FromLiveValue for MouseCursor {
    fn from_live_value(v: &makepad_widgets::LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let makepad_widgets::LiveValue::BareEnum(e) = v {
            Some(MouseCursor::from_str(&e.to_string()))
        } else {
            None
        }
    }
}