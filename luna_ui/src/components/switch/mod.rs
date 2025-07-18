mod event;
pub mod group;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop, SlotComponent, SlotProp},
        view::{GView, ViewBasicProp},
    },
    error::Error,
    event_option, lifecycle, play_animation,
    prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER},
        traits::ToFloat,
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::{draw_checkbox::DrawSwitch, draw_view::DrawView},
    themes::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSwitchBase = {{GSwitch}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_container: <AN_DRAW_VIEW> {},
                        draw_checkbox: <AN_DRAW_CHECKBOX> {}
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
                       draw_checkbox: <AN_DRAW_CHECKBOX> {}
                    }
                }

                active = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_container: <AN_DRAW_VIEW> {},
                        draw_checkbox: <AN_DRAW_CHECKBOX> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSwitch {
    // --- prop -------------------
    #[live]
    pub prop: SwitchProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live(false)]
    pub disabled: bool,
    #[live(false)]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    apply_slot_map: ApplySlotMap<SwitchState, SwitchPart>,
    // --- draw -------------------
    #[live]
    pub draw_checkbox: DrawSwitch,
    #[live]
    pub extra: GView,
    #[live]
    pub draw_container: DrawView,
    // #[rust]
    // defer_walks: DeferWalks,
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
    // --- value -------------------
    // is checkbox active? if is true, it can not be changed by user
    #[live]
    pub active: bool,
    // specific value of the checkbox, can be used to identify the checkbox
    #[live]
    pub value: String,
}

impl WidgetNode for GSwitch {
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
            self.draw_checkbox.redraw(cx);
            if self.extra.visible {
                self.extra.redraw(cx);
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

impl Widget for GSwitch {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        if self.visible {
            let state = self.current_state();
            let prop = self.prop.get(state);

            self.draw_container
                .begin(cx, prop.container.walk(), prop.container.layout());
            self.draw_checkbox
                .begin(cx, prop.checkbox.walk(), prop.checkbox.layout());
            self.draw_checkbox.end(cx);
            if self.extra.visible {
                self.extra.disabled = self.disabled;
                let _ = self.extra.draw_walk(cx, scope, prop.extra.walk());

                // let _ = SlotDrawer::new(
                //     [(live_id!(extra), (&mut self.extra).into())],
                //     &mut self.defer_walks,
                // )
                // .draw_walk(cx, scope);
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
        cx.global::<ComponentAnInit>().checkbox = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl LiveHook for GSwitch {
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
                (SwitchPart::Container, &ViewBasicProp::live_props()),
                (SwitchPart::Switch, &SwitchPartProp::live_props()),
                (SwitchPart::Extra, &ViewBasicProp::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(SwitchState::Basic, applys);
                }
                HOVER => {
                    component
                        .apply_slot_map
                        .insert(SwitchState::Hover, applys);
                }
                ACTIVE => {
                    component
                        .apply_slot_map
                        .insert(SwitchState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(SwitchState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<SwitchState> for GSwitch {
    type Part = SwitchPart;
}

impl Component for GSwitch {
    type Error = Error;

    type State = SwitchState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.checkbox;
        self.prop = prop.clone();
        self.extra.prop.basic = self.prop.basic.extra;
        self.extra.prop.hover = self.prop.hover.extra;
        self.extra.prop.pressed = self.prop.active.extra;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.current_state();
        let prop = self.prop.get(state);
        self.draw_container.merge(&prop.container);
        self.draw_checkbox.merge(&prop.checkbox);

        let _ = self.extra.render(cx)?;
        if self.active {
            self.draw_checkbox.active = 1.0;
            self.switch_state(SwitchState::Active);
        } else {
            self.draw_checkbox.active = 0.0;
        }
        Ok(())
    }

    fn current_state(&self) -> Self::State {
        if self.disabled {
            SwitchState::Disabled
        } else {
            self.draw_container.current_state().into()
        }
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(self.prop.get(self.current_state()).container.cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);
        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(area);
                }
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.prop.get(self.current_state()).container.cursor);
                if !self.active {
                    self.switch_state_with_animation(cx, SwitchState::Hover);
                    self.play_animation(cx, id!(hover.on));
                }
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                if !self.active {
                    self.switch_state_with_animation(cx, SwitchState::Basic);
                    self.play_animation(cx, id!(hover.off));
                }
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        let (state_an, state) = if self.active {
                            (id!(hover.off), SwitchState::Basic)
                        } else {
                            (id!(hover.active), SwitchState::Active)
                        };
                        self.active = !self.active;
                        self.switch_state_with_animation(cx, state);
                        self.play_animation(cx, state_an);
                    } else {
                        self.switch_state_with_animation(cx, SwitchState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, Some(e));
                } else {
                    self.switch_state_with_animation(cx, SwitchState::Basic);
                }
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
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 0.0,
                active: 0.0
            },
        );
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        match state {
            SwitchState::Basic => {
                self.draw_container.state_basic();
                self.draw_checkbox.state_basic();
            }
            SwitchState::Hover => {
                self.draw_container.state_hover();
                self.draw_checkbox.state_hover();
            }
            SwitchState::Active => {
                self.draw_container.state_pressed();
                self.draw_checkbox.state_active();
            }
            SwitchState::Disabled => {}
        }
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open || self.disabled {
            return;
        }
        self.switch_state(state);
        self.draw_checkbox
            .apply_type(self.prop.get(state).checkbox.mode);
        self.set_animation(cx);
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        let mut crossed_map = self.apply_slot_map.cross();
        for (part, slot) in [(SwitchPart::Extra, &mut self.extra)] {
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
        let init_global = cx.global::<ComponentAnInit>().checkbox;
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
            let basic_prop = self.prop.get(SwitchState::Basic);
            let hover_prop = self.prop.get(SwitchState::Hover);
            let active_prop = self.prop.get(SwitchState::Active);
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
                    live_id!(hover).as_instance(),
                    live_id!(active).as_instance(),
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

            set_animation! {
                nodes: draw_checkbox = {
                    basic_index => {
                        background_color => basic_prop.checkbox.background_color,
                        background_visible => basic_prop.checkbox.background_visible.to_f64(),
                        border_color => basic_prop.checkbox.border_color,
                        border_width => (basic_prop.checkbox.border_width as f64),
                        size => (basic_prop.checkbox.size as f64),
                        mode => basic_prop.checkbox.mode,
                        stroke_color => basic_prop.checkbox.stroke_color
                    },
                    hover_index => {
                        background_color => hover_prop.checkbox.background_color,
                        background_visible => hover_prop.checkbox.background_visible.to_f64(),
                        border_color => hover_prop.checkbox.border_color,
                        border_width => (hover_prop.checkbox.border_width as f64),
                        size => (hover_prop.checkbox.size as f64),
                        mode => hover_prop.checkbox.mode,
                        stroke_color => hover_prop.checkbox.stroke_color
                    },
                    active_index => {
                        background_color => active_prop.checkbox.background_color,
                        background_visible => active_prop.checkbox.background_visible.to_f64(),
                        border_color => active_prop.checkbox.border_color,
                        border_width => (active_prop.checkbox.border_width as f64),
                        size => (active_prop.checkbox.size as f64),
                        mode => active_prop.checkbox.mode,
                        stroke_color => active_prop.checkbox.stroke_color
                    }
                }
            }
        } else {
            let state = self.current_state();
            let prop = self.prop.get(state);
            let index = match state {
                SwitchState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                SwitchState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                SwitchState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(active).as_instance(),
                    ],
                ),
                _ => None,
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
            set_animation! {
                nodes: draw_checkbox = {
                    index => {
                        background_color => prop.checkbox.background_color,
                        background_visible => prop.checkbox.background_visible.to_f64(),
                        border_color => prop.checkbox.border_color,
                        border_width => (prop.checkbox.border_width as f64),
                        size => (prop.checkbox.size as f64),
                        mode => prop.checkbox.mode,
                        stroke_color => prop.checkbox.stroke_color
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

impl GSwitch {
    active_event! {
        active_hover_in: SwitchEvent::HoverIn |meta: FingerHoverEvent| => SwitchHoverIn { meta },
        active_hover_out: SwitchEvent::HoverOut |meta: FingerHoverEvent| => SwitchHoverOut { meta }
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    SwitchEvent::Clicked(SwitchClicked {
                        active: self.active,
                        value: self.value.to_string(),
                        meta,
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: SwitchEvent::HoverIn => SwitchHoverIn,
        hover_out: SwitchEvent::HoverOut => SwitchHoverOut,
        clicked: SwitchEvent::Clicked => SwitchClicked
    }
    pub fn toggle(&mut self, cx: &mut Cx, active: bool, init: bool) -> () {
        self.active = active;
        let (state, hover_id) = match (active, init) {
            (true, false) => (SwitchState::Active, Some(id!(hover.active))),
            (true, true) => (SwitchState::Active, None),
            (false, true) => (SwitchState::Basic, None),
            (false, false) => (SwitchState::Basic, Some(id!(hover.off))),
        };
        self.switch_state(state);
        if let Some(hover_id) = hover_id {
            self.play_animation(cx, hover_id);
        }
        self.active_clicked(cx, None);
    }
}
