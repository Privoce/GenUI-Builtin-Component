use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Value;

use crate::{
    error::Error,
    prop::{
        manuel::{HORIZONTAL, VERTICAL},
        traits::FromLiveValue,
    },
};

#[derive(Live, LiveHook, Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
#[live_ignore]
pub enum Direction {
    #[pick]
    #[default]
    Horizontal,
    Vertical,
}

impl TryFrom<&Value> for Direction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let direction_str = value
            .as_str()
            .ok_or_else(|| Error::ThemeStyleParse("Direction should be a string".to_string()))?;

        direction_str.parse()
    }
}

impl FromLiveValue for Direction {
    fn from_live_value(v: &LiveValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let LiveValue::BareEnum(e) = v {
            e.to_string().parse().ok()
        } else {
            None
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            HORIZONTAL => Ok(Self::Horizontal),
            VERTICAL => Ok(Self::Vertical),
            _ => Err(Error::ThemeStyleParse(format!("Unknown Direction: {}", s))),
        }
    }
}

impl ToLiveValue for Direction {
    fn to_live_value(&self) -> LiveValue {
        match self {
            Direction::Horizontal => LiveValue::BareEnum(live_id!(Horizontal)),
            Direction::Vertical => LiveValue::BareEnum(live_id!(Vertical)),
        }
    }
}

#[derive(Live, LiveHook, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum Position4 {
    Left = shader_enum(1),
    Right = shader_enum(2),
    Top = shader_enum(3),
    #[pick]
    Bottom = shader_enum(4),
}

impl Default for Position4 {
    fn default() -> Self {
        Position4::Bottom
    }
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Position {
    Left = shader_enum(1),
    LeftTop = shader_enum(2),
    LeftBottom = shader_enum(3),
    Right = shader_enum(4),
    RightTop = shader_enum(5),
    RightBottom = shader_enum(6),
    Top = shader_enum(7),
    TopLeft = shader_enum(8),
    TopRight = shader_enum(9),
    #[pick]
    Bottom = shader_enum(10),
    BottomLeft = shader_enum(11),
    BottomRight = shader_enum(12),
}

impl Default for Position {
    fn default() -> Self {
        Position::Bottom
    }
}

impl Position {
    pub fn to_drawer(&self) -> Self {
        match self {
            Position::Left | Position::LeftTop | Position::LeftBottom => Position::Left,
            Position::Right | Position::RightTop | Position::RightBottom => Position::Right,
            Position::Top | Position::TopLeft | Position::TopRight => Position::Top,
            Position::Bottom | Position::BottomLeft | Position::BottomRight => Position::Bottom,
        }
    }
    /// return angle offset
    pub fn angle_offset(&self, size: DVec2) -> f32 {
        match self {
            Position::Left | Position::Right | Position::Bottom | Position::Top => 0.0,
            Position::LeftTop
            | Position::LeftBottom
            | Position::RightTop
            | Position::RightBottom => (size.y / 2.0) as f32,
            Position::TopLeft
            | Position::TopRight
            | Position::BottomLeft
            | Position::BottomRight => (size.x / 2.0) as f32,
        }
    }
}

/// The `TriggerMode` enum represents the different modes for a trigger
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum TriggerMode {
    #[pick]
    Click = shader_enum(1),
    Hover = shader_enum(2),
    Press = shader_enum(3),
}

impl Default for TriggerMode {
    fn default() -> Self {
        TriggerMode::Click
    }
}

impl TriggerMode {
    pub fn is_click(&self) -> bool {
        matches!(self, TriggerMode::Click)
    }
    pub fn is_hover(&self) -> bool {
        matches!(self, TriggerMode::Hover)
    }
    pub fn is_press(&self) -> bool {
        matches!(self, TriggerMode::Press)
    }
}