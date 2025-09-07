use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Value;

use crate::{
    error::Error,
    prop::{
        manuel::{CROSS, ROUND, TICK},
        traits::{FromLiveValue, ToTomlValue},
    },
};

#[derive(Live, LiveHook, Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum ActiveMode {
    #[pick]
    #[default]
    /// 圆
    Round = shader_enum(1),
    /// 勾
    Tick = shader_enum(2),
    /// 横线
    Cross = shader_enum(3),
}

impl TryFrom<&Value> for ActiveMode {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mode_str = value
            .as_str()
            .ok_or_else(|| Error::ThemeStyleParse("ActiveMode should be a string".to_string()))?;

        mode_str.parse()
    }
}

impl FromLiveValue for ActiveMode {
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

impl ToTomlValue for ActiveMode {
    fn to_toml_value(&self) -> toml_edit::Value {
        match self {
            ActiveMode::Round => ROUND,
            ActiveMode::Tick => TICK,
            ActiveMode::Cross => CROSS,
        }
        .to_string()
        .to_toml_value()
    }
}

impl FromStr for ActiveMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ROUND => Ok(Self::Round),
            TICK => Ok(Self::Tick),
            CROSS => Ok(Self::Cross),
            _ => Err(Error::ThemeStyleParse(format!("Unknown ActiveMode: {}", s))),
        }
    }
}

impl ToLiveValue for ActiveMode {
    fn to_live_value(&self) -> LiveValue {
        match self {
            ActiveMode::Round => LiveValue::BareEnum(live_id!(Round)),
            ActiveMode::Tick => LiveValue::BareEnum(live_id!(Tick)),
            ActiveMode::Cross => LiveValue::BareEnum(live_id!(Cross)),
        }
    }
}
