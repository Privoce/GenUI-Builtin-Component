use std::{fmt::Debug, hash::Hash};

use makepad_widgets::{
    error, Area, Cx, Event, HeapLiveIdPath, Hit, LiveId, LiveNode, LiveValue, Walk, Widget,
    WidgetNode,
};

use crate::{
    components::lifecycle::LifeCycle,
    prop::{ApplySlotMap, ApplySlotMapImpl, ApplyStateMap, ApplyStateMapImpl, PropMap, SlotMap},
    themes::Theme,
};

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
    /// ## get current state of component
    fn current_state(&self) -> Self::State;
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
    fn clear_animation(&mut self, cx: &mut Cx) -> ();
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
        LP: IntoIterator<Item = &'m (LiveId, Option<Vec<LiveId>>)> + Copy,
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

pub trait Part: Hash + Eq + Copy {
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

    fn set_apply_slot_map<'m, LP, P, P2, NF, IF>(
        &mut self,
        nodes: &[LiveNode],
        index: usize,
        live_props: LP,
        prefixs: P,
        parts: P2,
        next_or: NF,
        insert: IF,
    ) -> ()
    where
        LP: IntoIterator<Item = &'m (LiveId, Option<Vec<LiveId>>)> + Copy,
        P: IntoIterator<Item = LiveId>,
        P2: IntoIterator<Item = Self::Part> + Copy,
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
            self, nodes, index, live_props, prefixs, parts, next_or, insert,
        );
    }
}

/// # Prop
/// trait for component properties
pub trait Prop: Default {
    type State;
    type Basic;
    fn get(&self, state: Self::State) -> &Self::Basic;
    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic;
    /// ## get length of the properties
    /// ### example:
    /// ```rust
    /// ABasicProp{
    ///     background_color: Color,
    ///     border_color: Color,
    ///     border_width: f32,
    /// }
    /// AProp {
    ///     basic: ABasicProp,
    ///     hover: ABasicProp,
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

pub trait SlotProp: Prop {
    type Part: Part;
    fn sync_slot(&mut self, map: &ApplySlotMap<Self::State, Self::Part>) -> ();
}

/// # BasicProp
/// trait for basic properties of a component
pub trait BasicProp: Default + Debug{
    type State;
    type Colors;

    fn from_state(theme: Theme, state: Self::State) -> Self;
    /// ## return state colors
    /// which depend on theme and Component Self
    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors;
    /// ## get length of the basic properties
    fn len() -> usize;
    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> ();
    /// ## sync from Basic State what apply from map if not set in DSL from (super Prop trait)
    /// unlike Prop trait, this function only sync theme colors, and use in `set_from_str()`
    fn sync(&mut self, state: Self::State) -> ();
    fn live_props() -> Vec<(LiveId, Option<Vec<LiveId>>)>;
    fn walk(&self) -> Walk;
}


pub trait SlotBasicProp: BasicProp {
    type Part: Part;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &LiveValue,
        state: Self::State,
        part: Self::Part,
    ) -> ();

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> ();
}
