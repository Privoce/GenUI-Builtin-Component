mod colors;
pub mod style;
pub mod register;

pub use colors::*;
use makepad_widgets::*;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum Themes {
    #[pick]
    Dark,
    Primary,
    Error,
    Warning,
    Success,
    Info,
}

impl Default for Themes {
    fn default() -> Self {
        Themes::Dark
    }
}

impl ToLiveValue for Themes {
    fn to_live_value(&self) -> LiveValue {
        match self {
            Themes::Dark => LiveValue::BareEnum(live_id!(Dark)),
            Themes::Primary => LiveValue::BareEnum(live_id!(Primary)),
            Themes::Error => LiveValue::BareEnum(live_id!(Error)),
            Themes::Warning => LiveValue::BareEnum(live_id!(Warning)),
            Themes::Success => LiveValue::BareEnum(live_id!(Success)),
            Themes::Info => LiveValue::BareEnum(live_id!(Info)),
        }
    }
}

impl Themes {
    pub fn get(&self, v: u32) -> Vec4 {
        match self {
            Themes::Dark => ThemeDark::v(v),
            Themes::Primary => ThemePrimary::v(v),
            Themes::Error => ThemeError::v(v),
            Themes::Warning => ThemeWarning::v(v),
            Themes::Success => ThemeSuccess::v(v),
            Themes::Info => ThemeInfo::v(v),
        }
    }
    pub fn hex(&self, v: u32) -> &'static str {
        match self {
            Themes::Dark => ThemeDark::hex(v),
            Themes::Primary => ThemePrimary::hex(v),
            Themes::Error => ThemeError::hex(v),
            Themes::Warning => ThemeWarning::hex(v),
            Themes::Success => ThemeSuccess::hex(v),
            Themes::Info => ThemeInfo::hex(v),
        }
    }
    pub fn to_vec(&self) -> Vec<Vec4> {
        let levels = vec![25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900];
        levels.iter().map(|v| self.get(*v)).collect()
    }
}

impl Display for Themes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Themes::Dark => write!(f, "Dark"),
            Themes::Primary => write!(f, "Primary"),
            Themes::Error => write!(f, "Error"),
            Themes::Warning => write!(f, "Warning"),
            Themes::Success => write!(f, "Success"),
            Themes::Info => write!(f, "Info"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ThemeColor {
    Dark(ThemeDark),
    Primary(ThemePrimary),
    Error(ThemeError),
    Warning(ThemeWarning),
    Success(ThemeSuccess),
    Info(ThemeInfo),
}

pub trait ThemeColorValue: Default {
    fn v(target: u32) -> Vec4;
    fn get(&self) -> Vec4;
    fn hex(target: u32) -> &'static str;
}
