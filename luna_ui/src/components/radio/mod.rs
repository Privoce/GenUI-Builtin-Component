mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::traits::{BasicProp, Component, Prop},
    error::Error,
    prop::ApplyStateMap,
    shader::{draw_radio::DrawRadio, draw_view::DrawView},
};

live_design! {
    link luna_basic;
    use link::luna_animation_prop::*;

    pub GRadio = <GRadio> {

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GRadio {
    // --- prop -------------------
    #[live]
    pub props: RadioProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<RadioState>,
    // --- draw -------------------
    #[live]
    pub draw_radio: DrawRadio,
    #[live]
    pub draw_text: DrawText,
    #[live]
    pub draw_container: DrawView,
}

impl WidgetNode for GRadio {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.props.get(self.current_state());
        prop.container.walk()
    }

    fn area(&self) -> Area {
        todo!()
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        todo!()
    }
}

impl Widget for GRadio {}

impl LiveHook for GRadio {}

impl Component for GRadio {
    type Error = Error;

    type State = RadioState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> () {
        todo!()
    }

    fn current_state(&self) -> Self::State {
        todo!()
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
        todo!()
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        todo!()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        todo!()
    }

    fn sync(&mut self) -> () {
        todo!()
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn lifecycle(&self) -> super::lifecycle::LifeCycle {
        todo!()
    }

    fn set_index(&mut self, index: usize) -> () {
        todo!()
    }
}
