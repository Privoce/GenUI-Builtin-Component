use makepad_widgets::{error, Area, Cx, Event, HeapLiveIdPath, Hit, LiveId, Widget, WidgetNode};

use crate::themes::Theme;

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
}

/// # Prop
/// trait for component properties
pub trait Prop: Default {
    type State;
    type Basic;
    fn get(&self, state: Self::State) -> &Self::Basic;
    fn len() -> usize;
}

/// # BasicProp
/// trait for basic properties of a component
pub trait BasicProp: Default {
    type State;
    type Colors;

    fn from_state(theme: Theme, state: Self::State) -> Self;
    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors;
    /// ## get length of the basic properties
    fn len() -> usize;
}
