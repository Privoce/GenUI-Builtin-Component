use crate::{
    components::traits::{BasicProp, Component, Part, SlotBasicProp},
    prop::manuel::THEME,
    themes::Theme,
};
use makepad_widgets::{
    live_id, LiveId, LiveIdAsProp, LiveNode, LiveNodeSliceApi, LiveProp, LiveValue,
};
use std::{borrow::Cow, collections::HashMap, hash::Hash};

/// PropMap is a mapping from a property name to a LiveValue, used for storing properties in components
pub type PropMap = HashMap<String, LiveValue>;
/// SlotMap need to use in Component which has slots, like: Card (header, body, footer), etc.
pub type SlotMap<P> = HashMap<P, PropMap>;
/// ApplyMap is a mapping from a state to a LiveValue, used for applying properties in animations or props
/// means: if in Button, use ApplyMap<ButtonState>
pub type ApplyStateMap<K> = HashMap<K, PropMap>;
/// ApplySlotMap is a mapping from a key to a SlotMap, used for applying slots in components
pub type ApplySlotMap<K, P> = HashMap<K, SlotMap<P>>;

pub trait ApplyMapImpl {
    /// ## merge
    /// merge other with self, if the key exists in self, it will ignore
    fn merge(&mut self, other: Self) -> ();
}

pub trait PropMapImpl: ApplyMapImpl {
    fn get_theme_then(&self, default: Theme) -> Theme;
    /// ## diff returns a new ApplyStateMap with only the keys that are in `other` but not in `self`
    fn diff(&self, other: &Self) -> Self;
}

/// # ApplyStateMapImpl
pub trait ApplyStateMapImpl<S>: ApplyMapImpl {
    /// ## set_map
    /// use to set map when in `after_apply()`
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

    /// ## sync
    /// sync the properties of the component with the given basic state and states
    /// - `prop`: the main properties to sync (basic properties)
    fn sync<'p, P, IS>(&'p self, prop: &mut P, basic_state: S, states: IS) -> ()
    where
        P: BasicProp<State = S> + 'p,
        IS: IntoIterator<Item = (S, &'p mut P)>;
}

pub trait ApplySlotMapImpl<S, IS, PT>: ApplyMapImpl
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
    // fn sync<'p, P, SS, PS>(&'p self, basic_state: S, states: SS, parts: PS) -> ()
    // where
    //     P: BasicProp<State = IS> + 'p,
    //     SS: IntoIterator<Item = S> + Copy,
    //     PS: IntoIterator<Item = (PT, &'p mut P)>;
    fn sync<'p, P, SS, PS>(
        &'p self,
        basic_prop: &mut P,
        basic_state: S,
        states: SS,
        parts: PS,
    ) -> ()
    where
        P: SlotBasicProp<Part = PT, State = S> + 'p,
        SS: IntoIterator<Item = (S, &'p mut P)>,
        PS: IntoIterator<Item = PT>;
    /// ## cross
    /// Used to intersect the outermost state component properties with the composition properties
    /// Meaning: convert `Map<State, Map<Part, Map<String, LiveValue>>>` to `Map<Part, Map<State, Map<String, LiveValue>>>`
    fn cross(&self) -> ApplySlotMap<PT, IS>;
}

impl<S, IS, PT> ApplySlotMapImpl<S, IS, PT> for ApplySlotMap<S, PT>
where
    IS: Hash + Eq + Copy,
    S: Hash + Eq + Copy + Into<IS>,
    PT: Part<State = IS>,
{
    fn cross(&self) -> ApplySlotMap<PT, IS> {
        let mut cross_map = ApplySlotMap::new();
        for (state, slots) in self {
            for (part, props) in slots {
                cross_map
                    .entry(*part)
                    .or_default()
                    .entry((*state).into())
                    .or_default()
                    .extend(props.clone());
            }
        }
        cross_map
    }

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
    fn sync<'p, P, SS, PS>(
        &'p self,
        basic_prop: &mut P,
        basic_state: S,
        states: SS,
        parts: PS,
    ) -> ()
    where
        P: SlotBasicProp<Part = PT, State = S> + 'p,
        SS: IntoIterator<Item = (S, &'p mut P)>,
        PS: IntoIterator<Item = PT>,
    {
        if let Some(basic_props) = self.get(&basic_state) {
            let mut states_vec: Vec<_> = states.into_iter().collect();
            for part in parts {
                if let Some(part_props) = basic_props.get(&part) {
                    let mut parts = Cow::Borrowed(part_props);
                    if parts.contains_key(THEME) {
                        let parts = parts.to_mut();
                        if let Some(value) = parts.remove(THEME) {
                            basic_prop.set_from_str_slot(THEME, &value, basic_state, part);
                        }
                    } else {
                        // 如果没有theme，则使用组件的theme
                        basic_prop.sync_slot(basic_state, part);
                    }
                    // 处理其他
                    for (k, v) in parts.iter() {
                        basic_prop.set_from_str_slot(&k, &v, basic_state, part);
                    }

                    for (state, props) in states_vec.iter_mut() {
                        self.get(&state).map(|state_map| {
                            let mut diff_props = state_map.get(&part).map_or_else(
                                || part_props.clone(),
                                |apply_props| apply_props.diff(&part_props),
                            );

                            // remove theme
                            if diff_props.contains_key(THEME) {
                                if let Some(value) = diff_props.remove(THEME) {
                                    props.set_from_str_slot(THEME, &value, *state, part);
                                } else {
                                    // if no theme, use self.theme
                                    props.sync_slot(*state, part);
                                }
                            }
                            // set from str
                            for (k, v) in diff_props.iter() {
                                props.set_from_str_slot(&k, &v, *state, part);
                            }
                        });
                    }
                }
            }
        }
    }
}

impl<S> ApplyMapImpl for ApplyStateMap<S>
where
    S: Hash + Eq + Copy,
{
    fn merge(&mut self, other: Self) -> () {
        for (state, props) in other {
            self.entry(state).or_default().merge(props);
        }
    }
}
impl<S, IS, PT> ApplyMapImpl for ApplySlotMap<S, PT>
where
    S: Hash + Eq + Copy + Into<IS>,
    PT: Part<State = IS>,
{
    fn merge(&mut self, other: Self) -> () {
        for (state, slots) in other {
            self.entry(state).or_default().merge(slots);
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
                    |apply_props| {
                        apply_props.diff(&basic_props)
                    },
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
        self.iter()
            .filter_map(|(k, v)| {
                match other.get(k) {
                    Some(other_v) if other_v == v => None, // 值相等，跳过
                    _ => Some((k.clone(), v.clone())),     // 不存在或值不同
                }
            })
            .chain(
                other
                    .iter()
                    .filter(|(k, _)| !self.contains_key(*k))
                    .map(|(k, v)| (k.clone(), v.clone())),
            )
            .collect()
    }
}

impl ApplyMapImpl for PropMap {
    fn merge(&mut self, other: Self) -> () {
        for (k, v) in other {
            self.entry(k).or_insert(v);
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge() {
        let mut a_map = super::ApplyStateMap::<i32>::from([
            (
                1,
                super::PropMap::from([("a".to_string(), LiveValue::Float64(1.0))]),
            ),
            (
                2,
                super::PropMap::from([("b".to_string(), LiveValue::Float64(2.0))]),
            ),
        ]);
        let b_map = super::ApplyStateMap::<i32>::from([
            (
                1,
                super::PropMap::from([
                    ("a".to_string(), LiveValue::Float64(10.0)),
                    ("b".to_string(), LiveValue::Float64(20.0)),
                    ("c".to_string(), LiveValue::Float64(3.0)),
                ]),
            ),
            (
                3,
                super::PropMap::from([("d".to_string(), LiveValue::Float64(4.0))]),
            ),
        ]);
        a_map.merge(b_map);
        dbg!(a_map);
    }

    #[test]
    fn test_prop_merge() {
        let mut a_map = super::PropMap::from([("a".to_string(), LiveValue::Float64(10.0))]);
        let b_map = super::PropMap::from([
            ("a".to_string(), LiveValue::Float64(1.0)),
            ("b".to_string(), LiveValue::Float64(2.0)),
            ("c".to_string(), LiveValue::Float64(3.0)),
        ]);
        a_map.merge(b_map);
        dbg!(a_map);
    }
}
