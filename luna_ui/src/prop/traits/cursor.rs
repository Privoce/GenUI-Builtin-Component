use makepad_widgets::MouseCursor;

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