use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

use makepad_widgets::{
    error, live_id, Area, Cx, Cx2d, DVec2, Event, HeapLiveIdPath, Hit, Layout, LiveId,
    LiveIdAsProp, LiveNode, LiveValue, Scope, Walk, Widget, WidgetNode,
};

use crate::{
    components::{
        lifecycle::LifeCycle,
        live_props::{LiveProps, LivePropsValue},
    },
    error::Error,
    prop::{
        insert_map, ApplySlotMap, ApplySlotMapImpl, ApplyStateMap, ApplyStateMapImpl, Applys,
        Position, PropMap, SlotMap,
    },
    themes::Theme,
};

pub trait PopupComponent
where
    Self::Error: std::fmt::Debug,
{
    type Error;
    type State;
    /// ## render component after prop apply
    /// this function should use in LiveHook trait : `fn after_apply_from_doc`
    fn render_after_apply(&mut self, cx: &mut Cx) -> () {
        if !self.visible() {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("{} render error: {:?}", std::any::type_name::<Self>(), e);
        }
    }
    /// ## merge component props from config theme toml
    fn merge_conf_prop(&mut self, cx: &mut Cx) -> ();
    /// ## render component
    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error>;
    /// ## Is the component visible?
    fn visible(&self) -> bool;
    /// ## set apply state map
    fn set_apply_state_map<'m, LP, P, NF, IF>(
        &mut self,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        next_or: NF,
        insert: IF,
    ) -> ()
    where
        LP: IntoIterator<Item = &'m (LiveId, LivePropsValue)> + Copy,
        P: IntoIterator<Item = LiveId>,
        NF: FnOnce(&mut Self) -> (),
        IF: FnOnce(LiveId, &mut Self, PropMap) -> () + Copy,
        Self: Sized,
        Self::State: Eq + Hash + Copy,
    {
        if self.lifecycle().is_created() {
            self.set_index(index);
            next_or(self);
        }

        for prefix in prefixs {
            let mut applys = PropMap::new();
            for (state, fields) in live_props {
                let mut paths = vec![
                    live_id!(prop).as_field(),
                    prefix.as_field(),
                    state.as_field(),
                ];

                fields.build_paths_and_insert(&mut paths, &mut |paths, deep_fields| {
                    insert_map(nodes, index, &mut applys, paths, deep_fields);
                });
            }
            insert(prefix, self, applys);
        }
    }
    /// ## sync component properties
    /// do before render component
    fn sync(&mut self) -> ();
    fn lifecycle(&self) -> LifeCycle;
    fn set_index(&mut self, index: usize) -> ();
    /// ## get current state of component
    fn current_state(&self) -> Self::State;
    /// ## Begin to draw popup
    /// this method is used to begin drawing the popup
    fn begin(&mut self, cx: &mut Cx2d) -> ();
    fn end(&mut self, cx: &mut Cx2d, scope: &mut Scope, shift_area: Area, shift: DVec2) -> ();
    fn redraw(&mut self, cx: &mut Cx) -> ();
    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> ();
    fn draw_popup(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Option<Position>,
        angle_offset: f32,
        redraw: &mut bool,
    ) -> ();
}

/// # Component Trait
/// Each Component should implement this trait
pub trait Component: Widget + WidgetNode
where
    Self::Error: std::fmt::Debug,
{
    type Error;
    type State;
    /// ## render component after prop apply
    /// this function should use in LiveHook trait : `fn after_apply_from_doc`
    fn render_after_apply(&mut self, cx: &mut Cx) -> () {
        if !self.visible() {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("{} render error: {:?}", std::any::type_name::<Self>(), e);
        }
    }
    /// ## merge component props from config theme toml
    fn merge_conf_prop(&mut self, cx: &mut Cx) -> ();
    /// ## render component
    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error>;
    // fn area(&self) -> Area;
    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> ();
    /// ## handle event for component
    /// from `fn handle_event()` in `impl Widget for $Component`
    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area);
    /// ## handle event when component is disabled
    /// this function should be called when component is disabled and event is not handled
    /// ```rust
    /// if self.disabled {
    ///     self.handle_when_disabled(cx, event, hit);
    /// } else {
    ///     self.handle_widget_event(cx, event, hit, area);
    /// }
    /// ```
    fn handle_when_disabled(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit) -> () {
        ()
    }
    /// ## play animation if component has
    /// depend on component struct `#[animator] animator: Animator`
    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> ();
    /// ## clear animation if component has
    fn clear_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }
    /// only switch state
    fn switch_state(&mut self, state: Self::State) -> ();
    /// ## switch state and redraw component
    /// if component has animation or event which may change state, this function should be called
    fn switch_state_and_redraw(&mut self, cx: &mut Cx, state: Self::State) -> () {
        self.switch_state(state);
        let _ = self.render(cx);
        self.redraw(cx);
    }
    /// ## switch state with animation
    /// if you not define #[animator] in component struct, do not care about this function
    /// this function should be called when you want to switch state with animation
    /// ### steps:
    /// 1. call animation_enabled or return
    /// 2. call switch_state fn
    /// 3. call play animation
    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> ();
    /// ## sync component properties
    /// do before render component
    fn sync(&mut self) -> ();
    /// ## focus do sync fn again
    fn focus_sync(&mut self) -> ();
    fn set_animation(&mut self, cx: &mut Cx) -> ();
    fn lifecycle(&self) -> LifeCycle;
    fn set_index(&mut self, index: usize) -> ();
    /// ## set apply state map
    fn set_apply_state_map<'m, LP, P, NF, IF>(
        &mut self,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        next_or: NF,
        insert: IF,
    ) -> ()
    where
        LP: IntoIterator<Item = &'m (LiveId, LivePropsValue)> + Copy,
        P: IntoIterator<Item = LiveId>,
        NF: FnOnce(&mut Self) -> (),
        IF: FnOnce(LiveId, &mut Self, PropMap) -> () + Copy,
        Self: Sized,
        Self::State: Eq + Hash + Copy,
    {
        // ApplyStateMap::<Self::State>::set_map(
        //     self, nodes, index, live_props, prefixs, next_or, insert,
        // );
        <ApplyStateMap<Self::State> as ApplyStateMapImpl<Self::State>>::set_map(
            self, nodes, index, live_props, prefixs, next_or, insert,
        );
    }
}

pub trait Part: Hash + Eq + Copy + FromStr<Err = Error> + Display + Debug {
    type State;
    fn to_live_id(&self) -> LiveId;
}

/// # SlotComponent
/// trait for component which has slots, like: Card (header, body, footer), etc
/// ## attention:
/// - `IS`: `InnerState` is the state of the slot, which may different from the component state
/// because the slot may have different state than the component itself (each container as slot always use ViewState)
pub trait SlotComponent<IS>: Component
where
    IS: Eq + Hash + Copy,
{
    type Part: Part<State = IS>;

    fn set_apply_slot_map<'m, P, LP, PP, NF, IF>(
        &mut self,
        nodes: &[LiveNode],
        index: usize,
        prefixs: P,
        part_props: PP,
        next_or: NF,
        insert: IF,
    ) -> ()
    where
        P: IntoIterator<Item = LiveId>,
        LP: IntoIterator<Item = &'m (LiveId, LivePropsValue)>,
        PP: IntoIterator<Item = (Self::Part, LP)> + Copy,
        NF: FnOnce(&mut Self) -> (),
        IF: FnOnce(LiveId, &mut Self, SlotMap<Self::Part>) -> () + Copy,
        Self: Sized,
        Self::State: Eq + Hash + Copy + Into<IS>,
    {
        <ApplySlotMap<Self::State, Self::Part> as ApplySlotMapImpl<
            Self::State,
            IS,
            Self::Part,
        >>::set_map(
            self, nodes, index,  prefixs, part_props, next_or, insert,
        );
    }
}

/// # Style
/// trait for component properties
pub trait Style: Default {
    type State;
    type Basic;
    fn get(&self, state: Self::State) -> &Self::Basic;
    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic;
    /// ## get length of the properties
    /// ### example:
    /// ```rust
    /// ABasicStyle{
    ///     background_color: Color,
    ///     border_color: Color,
    ///     border_width: f32,
    /// }
    /// AProp {
    ///     basic: ABasicStyle,
    ///     hover: ABasicStyle,
    /// }
    /// ```
    /// **`len()` will return 2 * 3**
    fn len() -> usize;
    /// ## sync from Basic State what apply from map if not set in DSL
    /// this function should be called when you want to sync properties from Basic State
    /// in crate, this function is used in Component `sync` function, if Component live prop `sync` is true.
    /// this function can let other state properties sync from Basic State.
    fn sync(&mut self, map: &ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + Hash + Copy;
}

pub trait SlotStyle: Style {
    type Part: Part;
    fn sync_slot(&mut self, map: &ApplySlotMap<Self::State, Self::Part>) -> ();
}

/// # BasicStyle
/// trait for basic properties of a component
pub trait BasicStyle: Default + Debug {
    type State;
    type Colors;

    fn from_state(theme: Theme, state: Self::State) -> Self;
    /// ## return state colors
    /// which depend on theme and Component Self
    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors;
    /// ## get length of the basic properties
    fn len() -> usize;
    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> ();
    /// ## sync from Basic State what apply from map if not set in DSL from (super Style trait)
    /// unlike Style trait, this function only sync theme colors, and use in `set_from_str()`
    fn sync(&mut self, state: Self::State) -> ();
    fn live_props() -> LiveProps;
    fn walk(&self) -> Walk;
    fn layout(&self) -> Layout;
}

pub trait SlotBasicStyle: BasicStyle {
    type Part: Part;

    /// value: 当涉及到更深的层级时就会含有一个Some(Applys)，这个Applys只会是Applys::Deep且层级至少为2
    /// 此时说明当前组件下需要设置属性值的属性部分依然需要调用`set_from_str_slot`来处理
    /// 那么其中的part就需要从Applys中获取来进行转换（可能含有多个part），见`Applys::key_to_parts()`
    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> ();

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> ();
}

pub trait ComponentState: Display + Eq + Hash + Copy + From<String> {
    fn is_disabled(&self) -> bool;
}
