mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    animation_open_then_redraw, components::{
        label::GLabel,
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop, SlotComponent},
    }, error::Error, lifecycle, play_animation, prop::{manuel::{ACTIVE, BASIC, DISABLED, HOVER}, ApplySlotMap, ApplyStateMap}, pure_after_apply, set_index, set_scope_path, shader::{draw_radio::DrawRadio, draw_view::DrawView}, themes::Conf, visible, ComponentAnInit
};

live_design! {
    link luna_basic;
    use link::luna_animation_prop::*;

    pub GRadioBase = {{GRadio}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_container: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                        ease: InOutQuad,
                    },
                    apply: {
                       draw_container: <AN_DRAW_VIEW> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_container: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
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
    apply_slot_map: ApplySlotMap<RadioState, RadioPart>,
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

impl Widget for GRadio {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        if self.visible {
            let state = self.current_state();
            let prop = self.prop.get(state);

            self.draw_container
                .begin(cx, prop.container.walk(), prop.container.layout());
            self.draw_radio
                .begin(cx, prop.radio.walk(), prop.radio.layout());
            self.draw_radio.end(cx);
            if self.label.visible {
                let _ = self.label.draw_walk(cx, scope, prop.label.walk());
            }
            self.draw_container.end(cx);
        }

        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        self.set_animation(cx);
        cx.global::<ComponentAnInit>().radio = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl LiveHook for GRadio {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            nodes,
            index,
            &RadioBasicProp::live_props(),
            [live_id!(basic), live_id!(hover), live_id!(active), live_id!(disabled)],
            [
                RadioPart::Container,
                RadioPart::Radio,
                RadioPart::Label,
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(RadioState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(RadioState::Hover, applys);
                }
                ACTIVE => {
                    component.apply_slot_map.insert(RadioState::Active, applys);
                }
                DISABLED => {
                    component.apply_slot_map.insert(RadioState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<RadioState> for GRadio {
    type Part = RadioPart;
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
         animation_open_then_redraw!(self, cx, event);
        match hit {
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, RadioState::Active);
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.prop.get(self.current_state()).container.cursor);
                self.switch_state_with_animation(cx, RadioState::Hover);
            }
            Hit::FingerHoverOut(e) => {}
            _ => {}
        }
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        self.draw_container.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0,
            },
        );
        self.draw_radio.apply_over(
            cx,
            live! {
                hover: 0.0,
                active: 0.0
            },
        );
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        match state {
            RadioState::Basic => {
                self.draw_container.state_basic();
                self.draw_radio.state_basic();
            }
            RadioState::Hover => {
                self.draw_container.state_hover();
                self.draw_radio.state_hover();
            }
            RadioState::Active => {
                self.draw_container.state_pressed();
                self.draw_radio.state_active();
            }
            RadioState::Disabled => {}
        }
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
         if !self.animation_open || self.disabled {
            return;
        }
         self.switch_state(state);
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        ()
    }

    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}
