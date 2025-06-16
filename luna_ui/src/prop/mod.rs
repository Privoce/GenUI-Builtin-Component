pub mod manuel;
mod radius;
pub mod traits;
use std::collections::HashMap;

use makepad_widgets::LiveValue;
pub use radius::Radius;

use crate::{prop::manuel::THEME, themes::Theme};

/// PropMap is a mapping from a property name to a LiveValue, used for storing properties in components
pub type PropMap = HashMap<String, LiveValue>;
/// ApplyMap is a mapping from a state to a LiveValue, used for applying properties in animations or props
/// means: if in Button, use ApplyMap<ButtonState>
pub type ApplyStateMap<S> = HashMap<S, PropMap>;

pub trait PropMapImpl {
    fn get_theme_then(&self, default: Theme) -> Theme;
}

impl PropMapImpl for PropMap {
    fn get_theme_then(&self, default: Theme) -> Theme {
        self.get(THEME)
            .map_or_else(|| default, |v| (v, default).into())
    }
}
