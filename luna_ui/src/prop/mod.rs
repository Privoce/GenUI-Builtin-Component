pub mod manuel;
mod radius;
pub mod traits;
use std::{borrow::Cow, collections::HashMap, hash::Hash};

use makepad_widgets::{
    live_id, LiveId, LiveIdAsProp, LiveNode, LiveNodeSliceApi, LiveProp, LiveValue,
};
pub use radius::Radius;

use crate::{
    components::traits::{BasicProp, Component, Part},
    prop::manuel::THEME,
    themes::Theme,
};

/// PropMap is a mapping from a property name to a LiveValue, used for storing properties in components
pub type PropMap = HashMap<String, LiveValue>;
/// SlotMap need to use in Component which has slots, like: Card (header, body, footer), etc.
pub type SlotMap<P> = HashMap<P, PropMap>;
/// ApplyMap is a mapping from a state to a LiveValue, used for applying properties in animations or props
/// means: if in Button, use ApplyMap<ButtonState>
pub type ApplyStateMap<K> = HashMap<K, PropMap>;
/// ApplySlotMap is a mapping from a key to a SlotMap, used for applying slots in components
pub type ApplySlotMap<K, P> = HashMap<K, SlotMap<P>>;

pub trait PropMapImpl {
    fn get_theme_then(&self, default: Theme) -> Theme;
    /// ## diff returns a new ApplyStateMap with only the keys that are in `other` but not in `self`
    fn diff(&self, other: &Self) -> Self;
}

pub trait SlotMapImpl {}

pub trait ApplyStateMapImpl<S> {
    fn set_map<'m, C, LP, P, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        LP: IntoIterator<Item = &'m (LiveId, Option<Vec<LiveId>>)> + Copy,
        P: IntoIterator<Item = LiveId>,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, PropMap) -> () + Copy;

    fn sync<'p, P, IS>(&'p self, prop: &mut P, basic_state: S, states: IS) -> ()
    where
        P: BasicProp<State = S> + 'p,
        IS: IntoIterator<Item = (S, &'p mut P)>;
}

pub trait ApplySlotMapImpl<S, IS, PT>
where
    S: Hash + Eq + Copy + Into<IS>,
    PT: Part<State = IS>,
{
    fn set_map<'m, C, LP, P, P2, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        parts: P2,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        LP: IntoIterator<Item = &'m (LiveId, Option<Vec<LiveId>>)> + Copy,
        P: IntoIterator<Item = LiveId>,
        P2: IntoIterator<Item = PT> + Copy,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, SlotMap<PT>) -> () + Copy;
    fn sync<'p, P, SS, PS>(&'p self, basic_state: S, states: SS, parts: PS) -> ()
    where
        P: BasicProp<State = IS> + 'p,
        SS: IntoIterator<Item = S> + Copy,
        PS: IntoIterator<Item = (PT, &'p mut P)>;
}

impl<S, IS, PT> ApplySlotMapImpl<S, IS, PT> for ApplySlotMap<S, PT>
where
    S: Hash + Eq + Copy + Into<IS>,
    PT: Part<State = IS>,
{
    fn set_map<'m, C, LP, P, P2, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        parts: P2,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        LP: IntoIterator<Item = &'m (LiveId, Option<Vec<LiveId>>)> + Copy,
        P: IntoIterator<Item = LiveId>,
        P2: IntoIterator<Item = PT> + Copy,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, SlotMap<PT>) -> () + Copy,
    {
        if component.lifecycle().is_created() {
            component.set_index(index);
            next_or(component);
        }

        for prefix in prefixs {
            let mut slots = HashMap::new();
            for part in parts {
                let mut applys = HashMap::new();
                let live_part = part.to_live_id();
                for (state, fields) in live_props {
                    let mut paths = vec![
                        live_id!(prop).as_field(),
                        prefix.as_field(),
                        live_part.as_field(),
                        state.as_field(),
                    ];
                    if let Some(fields) = fields {
                        for field in fields {
                            paths.push(field.as_field());
                        }
                        // do loop
                        insert_map(nodes, index, &mut applys, &paths);
                    } else {
                        insert_map(nodes, index, &mut applys, &paths);
                    }
                }
                slots.insert(part, applys);
            }
            insert(prefix, component, slots);
        }
    }
    fn sync<'p, P, SS, PS>(&'p self, basic_state: S, states: SS, parts: PS) -> ()
    where
        P: BasicProp<State = IS> + 'p,
        SS: IntoIterator<Item = S> + Copy,
        PS: IntoIterator<Item = (PT, &'p mut P)>,
    {
        if let Some(basic_props) = self.get(&basic_state) {
            for (part, part_prop) in parts {
                if let Some(part_props) = basic_props.get(&part) {
                    let mut parts = Cow::Borrowed(part_props);
                    if parts.contains_key(THEME) {
                        let parts = parts.to_mut();
                        if let Some(value) = parts.remove(THEME) {
                            part_prop.set_from_str(THEME, &value, basic_state.into());
                        }
                    } else {
                        // 如果没有theme，则使用组件的theme
                        part_prop.sync(basic_state.into());
                    }
                    // 处理其他
                    for (k, v) in parts.iter() {
                        part_prop.set_from_str(&k, &v, basic_state.into());
                    }

                    for state in states {
                        // diff
                        let mut diff_props = basic_props.get(&part).map_or_else(
                            || part_props.clone(),
                            |apply_props| apply_props.diff(&part_props),
                        );
                        // remove theme
                        if diff_props.contains_key(THEME) {
                            if let Some(value) = diff_props.remove(THEME) {
                                part_prop.set_from_str(THEME, &value, state.into());
                            } else {
                                // if no theme, use self.theme
                                part_prop.sync(state.into());
                            }
                        }
                        // set from str
                        for (k, v) in diff_props.iter() {
                            part_prop.set_from_str(&k, &v, state.into());
                        }
                    }
                }
            }
        }
    }
}

impl<S> ApplyStateMapImpl<S> for ApplyStateMap<S>
where
    S: Hash + Eq + Copy,
{
    fn sync<'p, P, IS>(&'p self, prop: &mut P, basic_state: S, states: IS) -> ()
    where
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
            } else {
                // 如果没有theme，则使用组件的theme
                prop.sync(basic_state);
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

    fn set_map<'m, C, LP, P, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        LP: IntoIterator<Item = &'m (LiveId, Option<Vec<LiveId>>)> + Copy,
        P: IntoIterator<Item = LiveId>,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, PropMap) -> () + Copy,
    {
        if component.lifecycle().is_created() {
            component.set_index(index);
            next_or(component);
        }

        for prefix in prefixs {
            let mut applys = PropMap::new();
            for (state, fields) in live_props {
                let mut paths = vec![
                    live_id!(prop).as_field(),
                    prefix.as_field(),
                    state.as_field(),
                ];
                if let Some(fields) = fields {
                    for field in fields {
                        paths.push(field.as_field());
                    }
                    // do loop
                    insert_map(nodes, index, &mut applys, &paths);
                } else {
                    insert_map(nodes, index, &mut applys, &paths);
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

fn insert_map(
    nodes: &[LiveNode],
    index: usize,
    applys: &mut HashMap<String, LiveValue>,
    paths: &Vec<LiveProp>,
) {
    if let Some(i) = nodes.child_by_path(index, paths) {
        let node = &nodes[i];
        applys.insert(node.id.to_string(), node.value.clone());
    }
}
