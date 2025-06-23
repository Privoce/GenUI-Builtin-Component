pub mod manuel;
mod radius;
pub mod traits;
use std::collections::HashMap;

use makepad_widgets::{live_id, LiveId, LiveIdAsProp, LiveNode, LiveNodeSliceApi, LiveValue};
pub use radius::Radius;

use crate::{components::traits::Component, prop::manuel::THEME, themes::Theme};

/// PropMap is a mapping from a property name to a LiveValue, used for storing properties in components
pub type PropMap = HashMap<String, LiveValue>;
/// ApplyMap is a mapping from a state to a LiveValue, used for applying properties in animations or props
/// means: if in Button, use ApplyMap<ButtonState>
pub type ApplyStateMap<S> = HashMap<S, PropMap>;

pub trait PropMapImpl {
    fn get_theme_then(&self, default: Theme) -> Theme;
    /// ## diff returns a new ApplyStateMap with only the keys that are in `other` but not in `self`
    fn diff(&self, other: &Self) -> Self;
}

pub trait ApplyStateMapImpl {
    fn set_map<C, LP, P, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        LP: IntoIterator<Item = LiveId> + Copy,
        P: IntoIterator<Item = LiveId>,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, HashMap<String, LiveValue>) -> () + Copy;
}

impl<S> ApplyStateMapImpl for ApplyStateMap<S> {
    fn set_map<C, LP, P, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        LP: IntoIterator<Item = LiveId> + Copy,
        P: IntoIterator<Item = LiveId>,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, HashMap<String, LiveValue>) -> () + Copy,
    {
        if component.lifecycle().is_created() {
            component.set_index(index);
            next_or(component);
        }

        for prefix in prefixs {
            let mut applys = HashMap::new();
            for path in live_props {
                if let Some(i) = nodes.child_by_path(
                    index,
                    &[
                        live_id!(prop).as_field(),
                        prefix.as_field(),
                        path.as_field(),
                    ],
                ) {
                    let node = &nodes[i];
                    applys.insert(node.id.to_string(), node.value.clone());
                }
            }
            insert(prefix, component, applys);
        }
    }
}

impl PropMapImpl for PropMap {
    fn get_theme_then(&self, default: Theme) -> Theme {
        self.get(THEME)
            .map_or_else(|| default, |v| (v, default).into())
    }
    fn diff(&self, other: &Self) -> Self {
        if self.len() < other.len() {
            other
                .clone()
                .into_iter()
                .filter(|(k, v)| !self.contains_key(k) || self.get(k) != Some(v))
                .collect()
        } else {
            self.clone()
                .into_iter()
                .filter(|(k, v)| !other.contains_key(k) || other.get(k) != Some(v))
                .collect()
        }
    }
}
