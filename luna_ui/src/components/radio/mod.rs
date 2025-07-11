mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    animation_open_then_redraw, components::{
        label::{GLabel, LabelBasicProp},
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop, SlotComponent, SlotProp},
        view::ViewBasicProp,
    }, error::Error, hit_finger_down, lifecycle, play_animation, prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER}, traits::ToFloat, ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplyStateMap
    }, pure_after_apply, set_animation, set_index, set_scope_path, shader::{draw_radio::DrawRadio, draw_view::DrawView}, themes::Conf, visible, ComponentAnInit
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
                        draw_container: <AN_DRAW_VIEW> {},
                        draw_radio: <AN_DRAW_RADIO> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        active: Forward {duration: (AN_DURATION)},
                        ease: InOutQuad,
                    },
                    apply: {
                       draw_container: <AN_DRAW_VIEW> {},
                       draw_radio: <AN_DRAW_RADIO> {}
                    }
                }

                active = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_container: <AN_DRAW_VIEW> {},
                        draw_radio: <AN_DRAW_RADIO> {}
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
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(active),
                live_id!(disabled),
            ],
            [
                (RadioPart::Container, &ViewBasicProp::live_props()),
                (RadioPart::Radio, &RadioPartProp::live_props()),
                (RadioPart::Label, &LabelBasicProp::live_props()),
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
                    component
                        .apply_slot_map
                        .insert(RadioState::Disabled, applys);
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
                self.play_animation(cx, id!(hover.active));
                // hit_finger_down!();
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.prop.get(self.current_state()).container.cursor);
                self.switch_state_with_animation(cx, RadioState::Hover);
                self.play_animation(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, RadioState::Basic);
                self.play_animation(cx, id!(hover.off));
            }
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
        self.set_animation(cx);
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        let mut crossed_map = self.apply_slot_map.cross();
        for (part, slot) in [
            (RadioPart::Label, &mut self.label),
        ] {
            crossed_map.remove(&part).map(|map| {
                let map = map.into_iter().map(|(k, v)| (k.into(), v)).collect();
                slot.apply_state_map.merge(map);
            });

            slot.prop.sync(&slot.apply_state_map);
        }

        // sync state if is not Basic
        self.prop.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().radio;
        let live_ptr = match self.animator.live_ptr {
            Some(ptr) => ptr.file_id.0,
            None => return,
        };

        let mut registry = cx.live_registry.borrow_mut();
        let live_file = match registry.live_files.get_mut(live_ptr as usize) {
            Some(lf) => lf,
            None => return,
        };
         let nodes = &mut live_file.expanded.nodes;

        if self.lifecycle.is_created() || !init_global || self.scope_path.is_none() {
            self.lifecycle.next();
            let basic_prop = self.prop.get(RadioState::Basic);
            let hover_prop = self.prop.get(RadioState::Hover);
            let active_prop = self.prop.get(RadioState::Active);
            let (mut basic_index, mut hover_index, mut active_index) = (None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(off).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(on).as_instance(),
                ],
            ) {
                hover_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(on).as_instance(),
                ],
            ) {
                active_index = Some(index);
            }

            set_animation! {
                nodes: draw_container = {
                    basic_index => {
                        background_color => basic_prop.container.background_color,
                        border_color =>basic_prop.container.border_color,
                        border_radius => basic_prop.container.border_radius,
                        border_width =>(basic_prop.container.border_width as f64),
                        shadow_color => basic_prop.container.shadow_color,
                        spread_radius => (basic_prop.container.spread_radius as f64),
                        blur_radius => (basic_prop.container.blur_radius as f64),
                        shadow_offset => basic_prop.container.shadow_offset,
                        background_visible => basic_prop.container.background_visible.to_f64()
                    },
                    hover_index => {
                        background_color => hover_prop.container.background_color,
                        border_color => hover_prop.container.border_color,
                        border_radius => hover_prop.container.border_radius,
                        border_width => (hover_prop.container.border_width as f64),
                        shadow_color => hover_prop.container.shadow_color,
                        spread_radius => (hover_prop.container.spread_radius as f64),
                        blur_radius => (hover_prop.container.blur_radius as f64),
                        shadow_offset => hover_prop.container.shadow_offset,
                        background_visible => hover_prop.container.background_visible.to_f64()
                    },
                    active_index => {
                        background_color => active_prop.container.background_color,
                        border_color => active_prop.container.border_color,
                        border_radius => active_prop.container.border_radius,
                        border_width => (active_prop.container.border_width as f64),
                        shadow_color => active_prop.container.shadow_color,
                        spread_radius => (active_prop.container.spread_radius as f64),
                        blur_radius => (active_prop.container.blur_radius as f64),
                        shadow_offset => active_prop.container.shadow_offset,
                        background_visible => active_prop.container.background_visible.to_f64()
                    }
                }
            }

            set_animation!{
                nodes: draw_radio = {
                    basic_index => {
                        background_color => basic_prop.radio.background_color,
                        background_visible => basic_prop.radio.background_visible.to_f64(),
                        border_color => basic_prop.radio.border_color,
                        border_width => (basic_prop.radio.border_width as f64),
                        size => (basic_prop.radio.size as f64),
                        mode => basic_prop.radio.mode,
                        stroke_color => basic_prop.radio.stroke_color
                    },
                    hover_index => {
                        background_color => hover_prop.radio.background_color,
                        background_visible => hover_prop.radio.background_visible.to_f64(),
                        border_color => hover_prop.radio.border_color,
                        border_width => (hover_prop.radio.border_width as f64),
                        size => (hover_prop.radio.size as f64),
                        mode => hover_prop.radio.mode,
                        stroke_color => hover_prop.radio.stroke_color
                    },
                    active_index => {
                        background_color => active_prop.radio.background_color,
                        background_visible => active_prop.radio.background_visible.to_f64(),
                        border_color => active_prop.radio.border_color,
                        border_width => (active_prop.radio.border_width as f64),
                        size => (active_prop.radio.size as f64),
                        mode => active_prop.radio.mode,
                        stroke_color => active_prop.radio.stroke_color
                    }
                }
            }
        } else {
            let state = self.current_state();
            let prop = self.prop.get(state);
            let index = match state {
                RadioState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                RadioState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                RadioState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                _ => None
            };
            set_animation! {
                nodes: draw_container = {
                    index => {
                        background_color => prop.container.background_color,
                        border_color => prop.container.border_color,
                        border_radius => prop.container.border_radius,
                        border_width => (prop.container.border_width as f64),
                        shadow_color => prop.container.shadow_color,
                        spread_radius => (prop.container.spread_radius as f64),
                        blur_radius => (prop.container.blur_radius as f64),
                        shadow_offset => prop.container.shadow_offset,
                        background_visible => prop.container.background_visible.to_f64()
                    }
                }
            }
            set_animation!{
                nodes: draw_radio = {
                    index => {
                        background_color => prop.radio.background_color,
                        background_visible => prop.radio.background_visible.to_f64(),
                        border_color => prop.radio.border_color,
                        border_width => (prop.radio.border_width as f64),
                        size => (prop.radio.size as f64),
                        mode => prop.radio.mode,
                        stroke_color => prop.radio.stroke_color
                    }
                }
            }
        }
    }

    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}
