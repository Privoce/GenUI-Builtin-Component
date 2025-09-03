use makepad_widgets::MouseCursor;

use crate::prop::traits::FromLiveValue;

use super::ToCursor;

impl ToCursor for MouseCursor {
    fn from_str(s: &str) -> Self {
        match s {
            "default" | "Default" => MouseCursor::Default,
            "hand" | "Hand" => MouseCursor::Hand,
            "text" | "Text" => MouseCursor::Text,
            "move" | "Move" => MouseCursor::Move,
            "wait" | "Wait" => MouseCursor::Wait,
            "help" | "Help" => MouseCursor::Help,
            "not-allowed" | "NotAllowed" => MouseCursor::NotAllowed,
            "crosshair" | "Crosshair" => MouseCursor::Crosshair,
            "grab" | "Grab" => MouseCursor::Grab,
            "grabbing" | "Grabbing" => MouseCursor::Grabbing,
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
