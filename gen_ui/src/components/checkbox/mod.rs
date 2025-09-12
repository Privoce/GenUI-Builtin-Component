mod event;
mod group;
mod prop;
mod register;

pub use event::*;
pub use group::*;
pub use prop::*;
pub use register::register as checkbox_register;

use makepad_widgets::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, Style, SlotComponent, SlotStyle},
        view::{GView, ViewBasicStyle},
    },
    error::Error,
    event_option, lifecycle, play_animation,
    prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER},
        traits::ToFloat,
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ToStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::{draw_checkbox::DrawCheckbox, draw_view::DrawView},
    sync,
    themes::conf::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GCheckboxBase = {{GCheckbox}} {
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
                    },
                    ease: InOutQuad,
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

                disabled = {
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
pub struct GCheckbox {
    // --- prop -------------------
    #[live]
    pub style: CheckboxProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub checkbox_visible: bool,
    #[live]
    pub reverse: bool,
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
    apply_slot_map: ApplySlotMap<CheckboxState, CheckboxPart>,
    // --- draw -------------------
    #[live]
    pub draw_checkbox: DrawCheckbox,
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
    #[rust]
    pub state: CheckboxState,
}

impl WidgetNode for GCheckbox {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.container.walk()
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
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GCheckbox {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.visible {
            let state = self.state;
            let style = self.style.get(state);

            self.draw_container.begin(cx, walk, style.container.layout());
            if !self.reverse {
                if self.checkbox_visible {
                    self.draw_checkbox
                        .begin(cx, style.checkbox.walk(), style.checkbox.layout());
                    self.draw_checkbox.end(cx);
                }
            }
            if self.extra.visible {
                self.extra.disabled = self.disabled;
                let _ = self.extra.draw_walk(cx, scope, style.extra.walk());
            }

            if self.reverse {
                if self.checkbox_visible {
                    self.draw_checkbox
                        .begin(cx, style.checkbox.walk(), style.checkbox.layout());
                    self.draw_checkbox.end(cx);
                }
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

impl LiveHook for GCheckbox {
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
                (CheckboxPart::Container, &ViewBasicStyle::live_props()),
                (CheckboxPart::Checkbox, &CheckboxPartProp::live_props()),
                (CheckboxPart::Extra, &ViewBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(CheckboxState::Basic, applys);
                }
                HOVER => {
                    component
                        .apply_slot_map
                        .insert(CheckboxState::Hover, applys);
                }
                ACTIVE => {
                    component
                        .apply_slot_map
                        .insert(CheckboxState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(CheckboxState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<CheckboxState> for GCheckbox {
    type Part = CheckboxPart;
}

impl Component for GCheckbox {
    type Error = Error;

    type State = CheckboxState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.checkbox;
        self.style = style.clone();
        self.extra.style.basic = self.style.basic.extra;
        self.extra.style.hover = self.style.hover.extra;
        self.extra.style.pressed = self.style.active.extra;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(CheckboxState::Disabled);
        } else {
            if self.active {
                self.switch_state(CheckboxState::Active);
            } else {
                self.switch_state(CheckboxState::Basic);
            }
        }
        let state = self.state;
        let style = self.style.get(state);
        self.draw_container.merge(&style.container);
        self.draw_checkbox.merge(&style.checkbox);
        let _ = self.extra.render(cx)?;
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, CheckboxState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
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
                cx.set_cursor(self.style.get(self.state).container.cursor);
                if !self.active {
                    self.switch_state_with_animation(cx, CheckboxState::Hover);
                    self.play_animation(cx, id!(hover.on));
                }
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                if !self.active {
                    self.switch_state_with_animation(cx, CheckboxState::Basic);
                    self.play_animation(cx, id!(hover.off));
                }
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        let (state_an, state) = if self.active {
                            (id!(hover.off), CheckboxState::Basic)
                        } else {
                            (id!(hover.active), CheckboxState::Active)
                        };
                        self.active = !self.active;
                        self.switch_state_with_animation(cx, state);
                        self.play_animation(cx, state_an);
                    } else {
                        self.switch_state_with_animation(cx, CheckboxState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, Some(e));
                } else {
                    self.switch_state_with_animation(cx, CheckboxState::Basic);
                }
            }
            _ => {}
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.extra.switch_state(state.into());
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.draw_checkbox
            .apply_type(self.style.get(state).checkbox.mode);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();
        for (part, slot) in [(CheckboxPart::Extra, &mut self.extra)] {
            crossed_map.remove(&part).map(|map| {
                slot.apply_state_map.merge(map.to_state());
            });

            slot.focus_sync();
        }

        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
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
            let basic_prop = self.style.get(CheckboxState::Basic);
            let hover_prop = self.style.get(CheckboxState::Hover);
            let active_prop = self.style.get(CheckboxState::Active);
            let disabled_prop = self.style.get(CheckboxState::Disabled);
            let (mut basic_index, mut hover_index, mut active_index, mut disabled_index) =
                (None, None, None, None);
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

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
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
                    },
                    disabled_index => {
                        background_color => disabled_prop.container.background_color,
                        border_color => disabled_prop.container.border_color,
                        border_radius => disabled_prop.container.border_radius,
                        border_width => (disabled_prop.container.border_width as f64),
                        shadow_color => disabled_prop.container.shadow_color,
                        spread_radius => (disabled_prop.container.spread_radius as f64),
                        blur_radius => (disabled_prop.container.blur_radius as f64),
                        shadow_offset => disabled_prop.container.shadow_offset,
                        background_visible => disabled_prop.container.background_visible.to_f64()
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
                    },
                    disabled_index => {
                        background_color => disabled_prop.checkbox.background_color,
                        background_visible => disabled_prop.checkbox.background_visible.to_f64(),
                        border_color => disabled_prop.checkbox.border_color,
                        border_width => (disabled_prop.checkbox.border_width as f64),
                        size => (disabled_prop.checkbox.size as f64),
                        mode => disabled_prop.checkbox.mode,
                        stroke_color => disabled_prop.checkbox.stroke_color
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                CheckboxState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                CheckboxState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                CheckboxState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(active).as_instance(),
                    ],
                ),
                CheckboxState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_container = {
                    index => {
                        background_color => style.container.background_color,
                        border_color => style.container.border_color,
                        border_radius => style.container.border_radius,
                        border_width => (style.container.border_width as f64),
                        shadow_color => style.container.shadow_color,
                        spread_radius => (style.container.spread_radius as f64),
                        blur_radius => (style.container.blur_radius as f64),
                        shadow_offset => style.container.shadow_offset,
                        background_visible => style.container.background_visible.to_f64()
                    }
                }
            }
            set_animation! {
                nodes: draw_checkbox = {
                    index => {
                        background_color => style.checkbox.background_color,
                        background_visible => style.checkbox.background_visible.to_f64(),
                        border_color => style.checkbox.border_color,
                        border_width => (style.checkbox.border_width as f64),
                        size => (style.checkbox.size as f64),
                        mode => style.checkbox.mode,
                        stroke_color => style.checkbox.stroke_color
                    }
                }
            }
        }
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl GCheckbox {
    active_event! {
        active_hover_in: CheckboxEvent::HoverIn |meta: FingerHoverEvent| => CheckboxHoverIn { meta },
        active_hover_out: CheckboxEvent::HoverOut |meta: FingerHoverEvent| => CheckboxHoverOut { meta }
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    CheckboxEvent::Clicked(CheckboxClicked {
                        active: self.active,
                        value: self.value.to_string(),
                        meta,
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: CheckboxEvent::HoverIn => CheckboxHoverIn,
        hover_out: CheckboxEvent::HoverOut => CheckboxHoverOut,
        clicked: CheckboxEvent::Clicked => CheckboxClicked
    }
    pub fn toggle(&mut self, cx: &mut Cx, active: bool, init: bool) -> () {
        self.active = active;
        let (state, hover_id) = match (active, init) {
            (true, false) => (CheckboxState::Active, Some(id!(hover.active))),
            (true, true) => (CheckboxState::Active, None),
            (false, true) => (CheckboxState::Basic, None),
            (false, false) => (CheckboxState::Basic, Some(id!(hover.off))),
        };
        self.switch_state(state);
        if let Some(hover_id) = hover_id {
            self.play_animation(cx, hover_id);
        }
        self.active_clicked(cx, None);
    }
}
