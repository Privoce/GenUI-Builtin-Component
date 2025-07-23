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
