pub mod manuel;
mod radius;
pub mod traits;
use std::{borrow::Cow, collections::HashMap, hash::Hash};

use makepad_widgets::{live_id, LiveId, LiveIdAsProp, LiveNode, LiveNodeSliceApi, LiveValue};
pub use radius::Radius;

use crate::{
    components::traits::{BasicProp, Component},
    prop::manuel::THEME,
    themes::Theme,
};

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

pub trait ApplyStateMapImpl<S> {
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

    fn sync<'p, 'm, P, IS>(&'m self, prop: &mut P, basic_state: S, states: IS) -> ()
    where
        'p: 'm,
        P: BasicProp<State = S> + 'p,
        IS: IntoIterator<Item = (S, &'p mut P)>;
}

impl<S> ApplyStateMapImpl<S> for ApplyStateMap<S>
where
    S: Hash + Eq + Copy,
{
    fn sync<'p, 'm, P, IS>(&'m self, prop: &mut P, basic_state: S, states: IS) -> ()
    where
        'p: 'm,
        P: BasicProp<State = S> + 'p,
        IS: IntoIterator<Item = (S, &'p mut P)>,
    {
        if let Some(basic_props) = self.get(&basic_state) {
            // 在set_from_str前需要处理同步theme颜色，其他状态也一样
            // 步骤是：作差运算 -> remove theme -> set_from_str
            let mut props = Cow::Borrowed(basic_props);
            // [basic] --------------------------------------------------------------------------------
            // 处理theme
            if props.contains_key(THEME) {
                let props = props.to_mut();
                if let Some(value) = props.remove(THEME) {
                    prop.set_from_str(THEME, &value, basic_state);
                }
            }
            // 处理其他
            for (k, v) in props.iter() {
                prop.set_from_str(&k, &v, basic_state);
            }
            // [other states] -----------------------------------------------------------------------
            for (state, props) in states {
                // diff
                let mut diff_props = self.get(&state).map_or_else(
                    || basic_props.clone(),
                    |apply_props| apply_props.diff(&basic_props),
                );
                // remove theme
                if diff_props.contains_key(THEME) {
                    if let Some(value) = diff_props.remove(THEME) {
                        props.set_from_str(THEME, &value, state);
                    } else {
                        // if no theme, use self.theme
                        props.sync(state);
                    }
                }
                // set from str
                for (k, v) in diff_props.iter() {
                    props.set_from_str(&k, &v, state);
                }
            }
        }
    }

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
