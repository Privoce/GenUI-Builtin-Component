use crate::{
    components::{
        LivePropsValue, {BasicStyle, Component, Part, SlotBasicStyle},
    },
    error::Error,
    prop::{manuel::THEME, prop_converter::PropVecConverter},
    themes::Theme,
};
use makepad_widgets::{
    live_id, LiveId, LiveIdAsProp, LiveNode, LiveNodeSliceApi, LiveProp, LiveValue,
};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

/// PropMap is a mapping from a property name to a LiveValue, used for storing properties in components
pub type PropMap = HashMap<String, LiveValue>;
/// SlotMap need to use in Component which has slots, like: Card (header, body, footer), etc.
pub type SlotMap<P> = HashMap<P, Applys>;
/// ApplyMap is a mapping from a state to a LiveValue, used for applying properties in animations or props
/// means: if in Button, use ApplyMap<ButtonState>
pub type ApplyStateMap<K> = HashMap<K, PropMap>;
/// ApplySlotMap is a mapping from a key to a SlotMap, used for applying slots in components
pub type ApplySlotMap<K, P> = HashMap<K, SlotMap<P>>;
/// 一个可以深度寻址的Applys，因为被应用到属性实际上可能非常深，但前2层永远是固定的[prop, state]
/// 后续的层级将会出现：
/// 1. 最基础的情况，直接应用的属性和值: [prop_key, prop_value] (`[String, LiveValue]`)
/// 2. 带有深度的属性和值例如`Margin`: [prop_key, [prop_value_key, prop_value_value]] (`[String, [String, LiveValue]]`)
/// 3. 出现插槽: [part, [...]], 这会混合前两种情况，并延伸更多的层级，例如：
/// ```
/// [prop, state, [part, [slot, [prop_key, [prop_value_key, prop_value_value]]]]]
/// 对应如下：
/// [prop, basic, [icon, [svg, [margin, [top, 10.0]]]]]
/// ```
/// 这个Applys不需要考虑前2层，只需要一个可以递归的结构来处理后续会扩展出的层级
#[derive(Debug, Clone)]
pub enum Applys {
    /// 表示最基础的节点
    Value(LiveValue),
    /// 表示可深度嵌套的节点
    Deep(HashMap<String, Applys>),
}

impl Default for Applys {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplyMapImpl for Applys {
    fn merge<O>(&mut self, other: O) -> ()
    where
        O: Into<Self>,
    {
        match (self, other.into()) {
            (Applys::Deep(map1), Applys::Deep(map2)) => {
                for (key, value) in map2 {
                    map1.entry(key).or_insert(value);
                }
            }
            _ => {}
        }
    }
}

impl<P> ApplySlotMergeImpl<P> for Applys
where
    P: Display,
{
    fn merge_slot(&mut self, other: SlotMap<P>) -> () {
        if let Applys::Deep(map) = self {
            for (k, v) in other {
                map.entry(k.to_string()).or_insert(v);
            }
        } else {
            panic!("Cannot merge a SlotMap into a non-Deep Applys");
        }
    }
}

impl From<&Applys> for LiveValue {
    fn from(value: &Applys) -> Self {
        match value {
            Applys::Value(live_value) => live_value.clone(),
            Applys::Deep(_map) => {
                unreachable!("Cannot convert a Deep Applys to LiveValue directly, expected Value");
            }
        }
    }
}

impl From<&Applys> for PropMap {
    fn from(value: &Applys) -> Self {
        match value {
            Applys::Value(_) => {
                panic!("Cannot convert a Value Applys to PropMap directly, expected Deep");
            }
            Applys::Deep(hash_map) => hash_map
                .iter()
                .map(|(k, v)| (k.to_string(), LiveValue::from(v)))
                .collect(),
        }
    }
}

impl Applys {
    /// 创建一个最简单的Applys
    pub fn new() -> Self {
        Applys::Deep(HashMap::new())
    }
    pub fn is_value(&self) -> bool {
        matches!(self, Applys::Value(_))
    }
    pub fn is_deep(&self) -> bool {
        matches!(self, Applys::Deep(_))
    }
    pub fn key_to_parts<PT>(&self) -> Option<Vec<PT>>
    where
        PT: Part,
    {
        match self {
            Applys::Value(_) => None,
            Applys::Deep(hash_map) => {
                let mut parts = Vec::new();
                hash_map.keys().for_each(|key| {
                    if let Ok(part) = key.parse::<PT>() {
                        parts.push(part);
                    }
                });
                if parts.is_empty() {
                    return None;
                }
                return Some(parts);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Applys::Value(_) => false,
            Applys::Deep(map) => map.is_empty(),
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            Applys::Value(_) => false,
            Applys::Deep(map) => map.contains_key(key),
        }
    }

    pub fn extend(&mut self, other: Self) {
        match (self, other) {
            (Applys::Deep(map1), Applys::Deep(map2)) => {
                map1.extend(map2);
            }
            _ => {}
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Applys)> {
        match self {
            Applys::Value(_) => panic!("Cannot iterate over a Value Applys"),
            Applys::Deep(map) => map.iter(),
        }
    }

    pub fn entry(&mut self, key: String) -> &mut Applys {
        match self {
            Applys::Value(_) => panic!("Cannot insert into a Value Applys"),
            Applys::Deep(map) => map.entry(key).or_insert(Applys::new()),
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<Applys> {
        match self {
            Applys::Value(_) => panic!("Cannot remove from a Value Applys"),
            Applys::Deep(map) => map.remove(key),
        }
    }

    /// 计算两个Applys之间的差异返回一个新Applys，进行diff只会是同类型，也就是Deep和Deep｜Value和Value
    /// - 实际上Value和Value不存在这种情况
    /// - Deep和Deep的情况则是：self中如果没有other的key，那么就保留，如果有则需要比较值，值相同则去除，只保留值不同的
    pub fn diff(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Applys::Value(lv), Applys::Value(rv)) => {
                if lv == rv {
                    None // 值相同，去除
                } else {
                    Some(Applys::Value(lv.clone())) // 值不同，保留自己
                }
            }
            (Applys::Deep(lmap), Applys::Deep(rmap)) => {
                let filtered = lmap
                    .iter()
                    .filter_map(|(k, lv)| match rmap.get(k) {
                        Some(rv) => lv.diff(rv).map(|v| (k.clone(), v)),
                        None => Some((k.clone(), lv.clone())),
                    })
                    .chain(
                        rmap.iter()
                            .filter(|(k, _)| !lmap.contains_key(*k))
                            .map(|(k, v)| (k.clone(), v.clone())),
                    )
                    .collect::<HashMap<String, Applys>>();

                (!filtered.is_empty()).then_some(Applys::Deep(filtered))
            }
            _ => panic!("Cannot diff between Value and Deep Applys"),
        }
    }

    pub fn as_kvs(&self) -> impl Iterator<Item = (&str, &Applys)> {
        match self {
            Applys::Deep(map) => map.iter().map(|(k, v)| (k.as_str(), v)),
            _ => panic!("Cannot convert a Value Applys to key-value pairs, expected Deep"),
        }
    }
}

pub trait ApplyMapImpl {
    /// ## merge
    /// merge other with self, if the key exists in self, it will ignore
    fn merge<O>(&mut self, other: O) -> ()
    where
        O: Into<Self>,
        Self: Sized;
}

pub trait ApplySlotMergeImpl<P>
where
    P: Display,
{
    fn merge_slot(&mut self, other: SlotMap<P>) -> ();
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
        LP: IntoIterator<Item = &'m (LiveId, LivePropsValue)> + Copy,
        P: IntoIterator<Item = LiveId>,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, PropMap) -> () + Copy;

    /// ## sync
    /// sync the properties of the component with the given basic state and states
    /// - `prop`: the main properties to sync (basic properties)
    fn sync<'p, P, IS>(&'p self, prop: &mut P, basic_state: S, states: IS) -> ()
    where
        P: BasicStyle<State = S> + 'p,
        IS: IntoIterator<Item = (S, &'p mut P)>;
}

pub trait ApplySlotMapImpl<S, IS, PT>: ApplyMapImpl
where
    S: Hash + Eq + Copy + Into<IS>,
    PT: Part<State = IS>,
{
    fn set_map<'m, C, SS, LP, PP, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        states: SS,
        part_props: PP,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        SS: IntoIterator<Item = LiveId>,
        LP: IntoIterator<Item = &'m (LiveId, LivePropsValue)>,
        PP: IntoIterator<Item = (PT, LP)> + Copy,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, SlotMap<PT>) -> () + Copy;
    // fn sync<'p, P, SS, PS>(&'p self, basic_state: S, states: SS, parts: PS) -> ()
    // where
    //     P: BasicStyle<State = IS> + 'p,
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
        P: SlotBasicStyle<Part = PT, State = S> + 'p,
        SS: IntoIterator<Item = (S, &'p mut P)>,
        PS: IntoIterator<Item = PT>;
    /// ## cross
    /// Used to intersect the outermost state component properties with the composition properties
    /// Meaning: convert `Map<State, Map<Part, Map<String, LiveValue>>>` to `Map<Part, Map<State, Map<String, LiveValue>>>`
    /// - only use to slot
    /// ### example
    /// ```
    /// let mut crossed_map = self.apply_slot_map.cross();
    /// for (part, slot) in [(CheckboxPart::Extra, &mut self.extra)] {
    ///     crossed_map.remove(&part).map(|map| {
    ///         let map = map.into_iter().map(|(k, v)| (k.into(), v)).collect();
    ///         slot.apply_state_map.merge(map);
    ///     });
    ///
    ///     slot.prop.sync(&slot.apply_state_map);
    /// }
    /// ```
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

    fn set_map<'m, C, SS, LP, PP, NF, IF>(
        component: &mut C,
        nodes: &[LiveNode],
        index: usize,
        states: SS,
        part_props: PP,
        next_or: NF,
        insert: IF,
    ) where
        C: Component,
        SS: IntoIterator<Item = LiveId>,
        LP: IntoIterator<Item = &'m (LiveId, LivePropsValue)>,
        PP: IntoIterator<Item = (PT, LP)> + Copy,
        NF: FnOnce(&mut C) -> (),
        IF: FnOnce(LiveId, &mut C, SlotMap<PT>) -> () + Copy,
    {
        if component.lifecycle().is_created() {
            component.set_index(index);
            next_or(component);
        }

        for state in states {
            let mut slots = HashMap::new();
            for (part, live_props) in part_props {
                let mut applys = Applys::new();
                let live_part = part.to_live_id();
                // let mut slot_props = HashMap::new();
                for (key, fields) in live_props {
                    let mut paths = vec![
                        live_id!(style).as_field(),
                        state.as_field(),
                        live_part.as_field(),
                        key.as_field(),
                    ];
                    fields.build_paths_and_insert(&mut paths, &mut |paths, deep_fields| {
                        // 返回一个Applys，因为我们无法在创建时知道这个Applys的深度
                        build_applys(nodes, index, &mut applys, paths, deep_fields);
                    });
                }
                slots.insert(part, applys);
            }
            insert(state, component, slots);
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
        P: SlotBasicStyle<Part = PT, State = S> + 'p,
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
                            if let Some(mut diff_props) = state_map.get(&part).map_or_else(
                                || Some(part_props.clone()),
                                |apply_props| apply_props.diff(&part_props),
                            ) {
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
                            }
                        });
                    }
                }
            }
        }
    }
}

impl<P> ApplySlotMergeImpl<P> for PropMap
where
    P: Display,
{
    fn merge_slot(&mut self, other: SlotMap<P>) -> () {
        for (k, v) in other {
            match v {
                Applys::Value(live_value) => {
                    self.entry(k.to_string()).or_insert(live_value);
                }
                Applys::Deep(_) => {
                    panic!("Cannot merge a deep Applys into a PropMap, expected Value");
                }
            }
        }
    }
}

impl ApplyMapImpl for PropMap {
    fn merge<O>(&mut self, other: O) -> ()
    where
        O: Into<Self>,
    {
        for (k, v) in other.into() {
            self.entry(k).or_insert(v);
        }
    }
}

impl<S> ApplyMapImpl for ApplyStateMap<S>
where
    S: Hash + Eq + Copy,
{
    fn merge<O>(&mut self, other: O) -> ()
    where
        O: Into<Self>,
    {
        for (state, props) in other.into() {
            self.entry(state).or_default().merge(props);
        }
    }
}

impl<S> ApplySlotMergeImpl<S> for SlotMap<S>
where
    S: Hash + Eq + Copy + Display,
{
    fn merge_slot(&mut self, other: SlotMap<S>) -> () {
        for (k, v) in other {
            // self.entry(k).or_default().merge_slot(v.into());
            match v {
                Applys::Value(live_value) => {
                    self.entry(k).or_default().merge(Applys::Value(live_value));
                }
                Applys::Deep(hash_map) => {
                    self.entry(k).or_default().merge_slot(hash_map.into());
                }
            }
        }
    }
}

impl<S> ApplyMapImpl for SlotMap<S>
where
    S: Hash + Eq + Copy,
{
    fn merge<O>(&mut self, other: O) -> ()
    where
        O: Into<Self>,
    {
        for (k, v) in other.into() {
            <Applys as ApplyMapImpl>::merge::<Applys>(&mut self.entry(k).or_default(), v);
        }
    }
}

impl<S, IS, PT> ApplySlotMergeImpl<S> for ApplySlotMap<S, PT>
where
    S: Hash + Eq + Copy + Into<IS> + Display,
    PT: Part<State = IS>,
{
    fn merge_slot(&mut self, other: SlotMap<S>) -> () {
        for (k, v) in other {
            match v {
                Applys::Value(live_value) => {
                    self.entry(k).or_default().merge(Applys::Value(live_value));
                }
                Applys::Deep(hash_map) => {
                    self.entry(k)
                        .or_default()
                        .merge_slot(Applys::Deep(hash_map).into());
                }
            }
        }
    }
}

impl<S, IS, PT> ApplyMapImpl for ApplySlotMap<S, PT>
where
    S: Hash + Eq + Copy + Into<IS>,
    PT: Part<State = IS>,
{
    fn merge<O>(&mut self, other: O) -> ()
    where
        O: Into<Self>,
    {
        for (state, slots) in other.into() {
            self.entry(state).or_default().merge(slots);
        }
    }
}

impl<P, IS> From<Applys> for SlotMap<P>
where
    P: Part<State = IS> + FromStr<Err = Error> + Display,
{
    fn from(value: Applys) -> Self {
        match value {
            Applys::Value(_) => {
                panic!("Cannot convert a Value Applys to SlotMap directly, expected Deep");
            }
            Applys::Deep(hash_map) => hash_map
                .into_iter()
                .map(|(k, v)| (k.parse::<P>().unwrap(), v.into()))
                .collect(),
        }
    }
}

impl<S> ApplyStateMapImpl<S> for ApplyStateMap<S>
where
    S: Hash + Eq + Copy,
{
    fn sync<'p, P, IS>(&'p self, prop: &mut P, basic_state: S, states: IS) -> ()
    where
        P: BasicStyle<State = S> + 'p,
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
        LP: IntoIterator<Item = &'m (LiveId, LivePropsValue)> + Copy,
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
                    live_id!(style).as_field(),
                    prefix.as_field(),
                    state.as_field(),
                ];
                fields.build_paths_and_insert(&mut paths, &mut |paths, deep_fields| {
                    insert_map(nodes, index, &mut applys, paths, deep_fields);
                });
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

pub fn insert_map(
    nodes: &[LiveNode],
    index: usize,
    applys: &mut HashMap<String, LiveValue>,
    paths: &Vec<LiveProp>,
    deep_fields: Option<&Vec<LiveId>>,
) {
    if let Some(i) = nodes.child_by_path(index, paths) {
        let node = &nodes[i];
        if let Some(deep_fields) = deep_fields {
            // 深度字段需要查找当前node的id是否在deep_fields中，如果在，那么实际上应该以父paths作为key，将值类型处理为Vec2/Vec4
            if deep_fields.contains(&node.id) {
                let father_prop = paths
                    .get(paths.len() - 2)
                    .expect("Expected at least one path element for father path");
                let father_key = father_prop.0.to_string();
                // 构建一个prop converter, 他可以告诉我们应该如何处理这个深度属性
                let converter = PropVecConverter::new(&father_key, node.id, node.value.clone());
                // 没有则添加，有则修改
                applys
                    .entry(father_key)
                    .and_modify(|v| {
                        *v = converter.value(Some(v.clone()));
                    })
                    .or_insert(converter.value(None));
                return;
            } else {
                panic!("Deep field not found in deep_fields");
            }
        } else {
            applys.insert(node.id.to_string(), node.value.clone());
        }
    }
}

/// 构建Applys，由于无法在外部知道这个Applys的深度，所以在这个方法里实际上就要对Applys进行构建(变更/增加节点)
/// 关键点在于paths, 我们知道paths的前3层是固定的[prop, state, part]，后续的层级会根据组件的不同而变化
pub fn build_applys(
    nodes: &[LiveNode],
    index: usize,
    applys: &mut Applys,
    paths: &Vec<LiveProp>,
    deep_fields: Option<&Vec<LiveId>>,
) -> () {
    // 去除前3层的paths [prop, state, part]
    // 和insert_map类似来获取最终的节点值，但需要从第三层开始进行扩展, 首先保证splited_paths的长度大于等于1，因为最小的情况都需要有值的KV
    // 为空了说明没有足够的层级，这一般是不可能的，除非是错误的路径，这里直接不处理
    let splited_paths = paths[3..].to_vec();

    if splited_paths.is_empty() {
        return;
    }

    if let Some(i) = nodes.child_by_path(index, paths) {
        insert_value_at_path(applys, &splited_paths, &nodes[i], deep_fields);
    }
}

fn insert_value_at_path(
    applys: &mut Applys,
    paths: &[LiveProp],
    node: &LiveNode,
    deep_fields: Option<&Vec<LiveId>>,
) {
    if paths.is_empty() {
        return;
    }

    // 确保根节点是 Deep 类型
    match applys {
        Applys::Value(_) => *applys = Applys::new(),
        Applys::Deep(_) => {}
    }

    let mut current = applys;

    // 遍历路径，为每一层创建必要的结构
    for (i, LiveProp(prop_key, ..)) in paths.iter().enumerate() {
        let key = prop_key.to_string();
        let is_last = i == paths.len() - 1;

        if let Applys::Deep(map) = current {
            if is_last {
                // 最后一个键，设置值
                map.insert(key, Applys::Value(node.value.clone()));
                break;
            } else {
                // 处理需要深度属性的情况
                if let Some(deep_fields) = deep_fields {
                    if deep_fields.contains(&node.id) {
                        let father_prop = paths.get(paths.len() - 2).unwrap();
                        let father_key = father_prop.0.to_string();
                        if father_key == key {
                            let converter =
                                PropVecConverter::new(&father_key, node.id, node.value.clone());
                            map.entry(father_key)
                                .and_modify(|v| {
                                    *v = Applys::Value(
                                        converter.value(Some(LiveValue::from(&v.clone()))),
                                    );
                                })
                                .or_insert(Applys::Value(converter.value(None)));
                            return;
                        }
                    }
                }

                // 中间键，确保有 Deep 结构继续向下
                current = map.entry(key).or_insert_with(|| Applys::new());

                // 确保当前节点是 Deep 类型，以便继续嵌套
                if let Applys::Value(_) = current {
                    *current = Applys::new();
                }
            }
        } else {
            // 这不应该发生，但为了安全起见
            break;
        }
    }
}

pub trait ToStateMap<S>
where
    S: Hash + Eq + Copy,
{
    fn to_state(self) -> HashMap<S, PropMap>;
}

impl<K, S> ToStateMap<S> for HashMap<K, Applys>
where
    K: Into<S> + Hash + Eq + Copy,
    S: Hash + Eq + Copy,
{
    fn to_state(self) -> HashMap<S, PropMap> {
        self.into_iter()
            .filter_map(|(k, v)| {
                if v.is_empty() {
                    None
                } else {
                    Some((k.into(), PropMap::from(&v)))
                }
            })
            .collect()
    }
}

pub trait ToSlotMap<S>
where
    S: Hash + Eq + Copy,
{
    fn to_slot(self) -> HashMap<S, Applys>;
}

impl<K, S> ToSlotMap<S> for HashMap<K, Applys>
where
    K: Into<S> + Hash + Eq + Copy,
    S: Hash + Eq + Copy,
{
    fn to_slot(self) -> HashMap<S, Applys> {
        self.into_iter()
            .filter_map(|(k, v)| {
                if v.is_empty() {
                    None
                } else {
                    Some((k.into(), v))
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_applys_diff() {
        let a = Applys::Deep(HashMap::from([
            ("a".to_string(), Applys::Value(LiveValue::Float64(1.0))),
            ("b".to_string(), Applys::Value(LiveValue::Float64(2.0))),
            ("c".to_string(), Applys::Value(LiveValue::Float64(3.0))),
        ]));
        let b = Applys::Deep(HashMap::from([
            ("a".to_string(), Applys::Value(LiveValue::Float64(1.0))),
            ("b".to_string(), Applys::Value(LiveValue::Float64(2.0))),
            ("c".to_string(), Applys::Value(LiveValue::Float64(5.0))),
            ("d".to_string(), Applys::Value(LiveValue::Float64(4.0))),
        ]));
        let diff = a.diff(&b);
        dbg!(diff);
    }

    #[test]
    fn test_prop_map_diff() {
        let a_map = PropMap::from([
            ("a".to_string(), LiveValue::Float64(1.0)),
            ("b".to_string(), LiveValue::Float64(2.0)),
            ("c".to_string(), LiveValue::Float64(3.0)),
        ]);
        let b_map = PropMap::from([
            ("a".to_string(), LiveValue::Float64(1.0)),
            ("b".to_string(), LiveValue::Float64(2.0)),
            ("c".to_string(), LiveValue::Float64(5.0)),
            ("d".to_string(), LiveValue::Float64(4.0)),
        ]);
        let diff_map = a_map.diff(&b_map);
        dbg!(diff_map);
    }

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

    #[test]
    fn different_between_merge_or_insert() {
        let mut a_map = HashMap::from([
            //     (
            //     "a".to_string(),
            //     HashMap::from([("a1".to_string(), "1".to_string())]),
            // ),
            (
                "b".to_string(),
                HashMap::from([("b1".to_string(), "2".to_string())]),
            ),
        ]);

        let mut merge_map = a_map.clone();
        merge_map
            .entry("a".to_string())
            .or_default()
            .entry("a1".to_string())
            .or_insert("0".to_string());
        dbg!(merge_map);
        a_map
            .entry("a".to_string())
            .or_insert(HashMap::from([("a1".to_string(), "0".to_string())]));
        dbg!(a_map);
    }
}
