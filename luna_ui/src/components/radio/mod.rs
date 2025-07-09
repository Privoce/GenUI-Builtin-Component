mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        label::GLabel,
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop},
    },
    error::Error,
    lifecycle, play_animation,
    prop::ApplyStateMap,
    pure_after_apply, set_index, set_scope_path,
    shader::{draw_radio::DrawRadio, draw_view::DrawView},
    themes::Conf,
    visible,
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
    pub prop: RadioProp,
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
    pub label: GLabel,
    #[live]
    pub draw_container: DrawView,
    // --- animation ---------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
}

impl WidgetNode for GRadio {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.current_state());
        prop.container.walk()
    }

    fn area(&self) -> Area {
        self.draw_container.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        if self.visible {
            let _ = self.render(cx);
            self.draw_container.redraw(cx);
            self.draw_radio.redraw(cx);
            if self.label.visible {
                self.label.redraw(cx);
            }
        }
    }

    fn state(&self) -> String {
        self.current_state().to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GRadio {}

impl LiveHook for GRadio {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
}

impl Component for GRadio {
    type Error = Error;

    type State = RadioState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.radio;
        self.prop = prop.clone();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.current_state();
        let prop = self.prop.get(state);
        self.draw_container.merge(&prop.container);
        self.draw_radio.merge(&prop.radio);
        let _ = self.label.render(cx)?;
        Ok(())
    }

    fn current_state(&self) -> Self::State {
        if self.disabled {
            RadioState::Disabled
        } else {
            self.draw_container.current_state().into()
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
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

    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}
