use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Value;

use crate::{
    error::Error,
    prop::{
        manuel::{RECT, ROUND},
        traits::FromLiveValue,
    },
};

#[derive(Live, LiveHook, Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum SwitchMode {
    #[pick]
    #[default]
    Round = shader_enum(1),
    Rect = shader_enum(2),
}

impl TryFrom<&Value> for SwitchMode {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mode_str = value
            .as_str()
            .ok_or_else(|| Error::ThemeStyleParse("SwitchMode should be a string".to_string()))?;

        mode_str.parse()
    }
}

impl FromLiveValue for SwitchMode {
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

impl FromStr for SwitchMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ROUND => Ok(Self::Round),
            RECT => Ok(Self::Rect),
            _ => Err(Error::ThemeStyleParse(format!("Unknown SwitchMode: {}", s))),
        }
    }
}

impl ToLiveValue for SwitchMode {
    fn to_live_value(&self) -> LiveValue {
        match self {
            SwitchMode::Round => LiveValue::BareEnum(live_id!(Round)),
            SwitchMode::Rect => LiveValue::BareEnum(live_id!(Rect)),
        }
    }
}
