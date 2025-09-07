use makepad_widgets::MouseCursor;

use crate::prop::traits::{FromLiveValue, ToTomlValue};

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
            "not_allowed" | "NotAllowed" => MouseCursor::NotAllowed,
            "crosshair" | "Crosshair" => MouseCursor::Crosshair,
            "grab" | "Grab" => MouseCursor::Grab,
            "grabbing" | "Grabbing" => MouseCursor::Grabbing,
            "n_resize" | "NResize" => MouseCursor::NResize,
            "ne_resize" | "NeResize" => MouseCursor::NeResize,
            "e_resize" | "EResize" => MouseCursor::EResize,
            "se_resize" | "SeResize" => MouseCursor::SeResize,
            "s_resize" | "SResize" => MouseCursor::SResize,
            "sw_resize" | "SwResize" => MouseCursor::SwResize,
            "w_resize" | "WResize" => MouseCursor::WResize,
            "nw_resize" | "NwResize" => MouseCursor::NwResize,
            "ns_resize" | "NsResize" => MouseCursor::NsResize,
            "nesw_resize" | "NeswResize" => MouseCursor::NeswResize,
            "ew_resize" | "EwResize" => MouseCursor::EwResize,
            "nwse_resize" | "NwseResize" => MouseCursor::NwseResize,
            "col_resize" | "ColResize" => MouseCursor::ColResize,
            "row_resize" | "RowResize" => MouseCursor::RowResize,
            "hidden" | "Hidden" => MouseCursor::Hidden,
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

impl ToTomlValue for  MouseCursor {
    fn to_toml_value(&self) -> toml_edit::Value {
        match self {
            MouseCursor::Hidden => "Hidden",
            MouseCursor::Default => "Default",
            MouseCursor::Crosshair => "Crosshair",
            MouseCursor::Hand => "Hand",
            MouseCursor::Arrow => "Arrow",
            MouseCursor::Move => "Move",
            MouseCursor::Text => "Text",
            MouseCursor::Wait => "Wait",
            MouseCursor::Help => "Help",
            MouseCursor::NotAllowed => "NotAllowed",
            MouseCursor::Grab => "Grab",
            MouseCursor::Grabbing => "Grabbing",
            MouseCursor::NResize => "NResize",
            MouseCursor::NeResize => "NeResize",
            MouseCursor::EResize => "EResize",
            MouseCursor::SeResize => "SeResize",
            MouseCursor::SResize => "SResize",
            MouseCursor::SwResize => "SwResize",
            MouseCursor::WResize => "WResize",
            MouseCursor::NwResize => "NwResize",
            MouseCursor::NsResize => "NsResize",
            MouseCursor::NeswResize => "NeswResize",
            MouseCursor::EwResize => "EwResize",
            MouseCursor::NwseResize => "NwseResize",
            MouseCursor::ColResize => "ColResize",
            MouseCursor::RowResize => "RowResize",
        }.to_string().to_toml_value()
    }
}