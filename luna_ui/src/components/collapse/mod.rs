mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    active_event, animation_open_then_redraw, area,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop, SlotComponent, SlotProp},
        view::{GView, ViewBasicProp},
    },
    error::Error,
    event_option, hit_hover_in, lifecycle, play_animation,
    prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER},
        traits::ToFloat,
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, Position4, ToStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GCollapseBase = {{GCollapse}}{
        animator: {
            active = {
                default: off
                off = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    redraw: true
                    apply: {
                        draw_collapse: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
                on = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    redraw: true
                    apply: {
                        draw_collapse: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GCollapse {
    #[live]
    pub prop: CollapseProp,
    #[live]
    pub header: GView,
    #[live]
    pub body: GView,
    #[live]
    pub draw_collapse: DrawView,
    #[live]
    pub active: bool,
    #[live]
    pub fold: f64,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    #[rust]
    pub rect_size: f64,
    #[live]
    pub position: Position4,
    #[rust]
    pub area: Area,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    // use animation counter to prevent multiple animations
    #[rust(true)]
    animation_counter: bool,
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
    #[rust]
    pub state: CollapseState,
    #[rust]
    pub draw_state: DrawStateWrap<DrawCollapseState>,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<CollapseState, CollapsePart>,
}

impl Widget for GCollapse {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let prop = self.prop.get_mut(self.state);
        self.fold = self.active.to_f64();
        let body_walk = self.body.walk(cx);
        let header_walk = self.header.walk(cx);
        let (flow, steps) = match self.position {
            Position4::Left => (
                Flow::Right,
                [DrawCollapseState::DrawBody, DrawCollapseState::DrawHeader],
            ),
            Position4::Right => (
                Flow::Right,
                [DrawCollapseState::DrawHeader, DrawCollapseState::DrawBody],
            ),
            Position4::Top => (
                Flow::Down,
                [DrawCollapseState::DrawBody, DrawCollapseState::DrawHeader],
            ),
            Position4::Bottom => (
                Flow::Down,
                [DrawCollapseState::DrawHeader, DrawCollapseState::DrawBody],
            ),
        };

        // self.layout.flow = flow;
        prop.container.flow = flow;
        if self.draw_state.begin(cx, steps[0]) {
            if !self.active {
                match self.position {
                    Position4::Left | Position4::Right => {
                        if !walk.width.is_fixed() {
                            walk.width = header_walk.width;
                        }
                    }
                    Position4::Top | Position4::Bottom => {
                        if !walk.height.is_fixed() {
                            walk.height = header_walk.height;
                        }
                    }
                }

                cx.begin_turtle(walk, prop.layout());
            } else {
                match self.position {
                    Position4::Left | Position4::Right => {
                        if !walk.width.is_fixed() {
                            walk.width = Size::Fill;
                        }
                    }
                    Position4::Top | Position4::Bottom => {
                        if !walk.height.is_fixed() {
                            walk.height = Size::Fill;
                        }
                    }
                }

                // // if is active, walk should be Fill
                // let walk = if walk.height.is_fixed() {
                //     walk
                // } else {
                //     Walk::fill()
                // };
                cx.begin_turtle(walk, prop.layout());
            }
        }

        for (index, _) in steps.iter().enumerate() {
            let _ = self.draw_state.get().map(|state| match state {
                DrawCollapseState::DrawHeader => {
                    let _ = self.header.draw_walk(cx, scope, header_walk);
                    // check is the first step
                    if index == 0 {
                        cx.begin_turtle(
                            body_walk,
                            Layout::flow_down()
                                .with_scroll(dvec2(0.0, self.rect_size * (1.0 - self.fold))),
                        );
                        self.draw_state.set(steps[1]);
                    } else {
                        match self.position {
                            Position4::Left | Position4::Right => {
                                self.rect_size = cx.turtle().used().x;
                            }
                            Position4::Top | Position4::Bottom => {
                                self.rect_size = cx.turtle().used().y;
                            }
                        }
                        cx.end_turtle();
                        cx.end_turtle_with_area(&mut self.area);
                        self.draw_state.end();
                    }
                }
                DrawCollapseState::DrawBody => {
                    if self.fold == 1.0 {
                        self.animator_play(cx, id!(active.on));
                        let _ = self.body.draw_walk(cx, scope, body_walk);
                    }
                    // check is the last step
                    if index == 1 {
                        match self.position {
                            Position4::Left | Position4::Right => {
                                self.rect_size = cx.turtle().used().x;
                            }
                            Position4::Top | Position4::Bottom => {
                                self.rect_size = cx.turtle().used().y;
                            }
                        }
                        cx.end_turtle();
                        cx.end_turtle_with_area(&mut self.area);
                        self.draw_state.end();
                    } else {
                        cx.begin_turtle(header_walk, Layout::flow_down());
                        self.draw_state.set(steps[1]);
                    }
                }
            });
        }
        DrawStep::done()
    }
    // fn handle_event_with(
    //     &mut self,
    //     cx: &mut Cx,
    //     event: &Event,
    //     scope: &mut Scope,
    //     sweep_area: Area,
    // ) {
    //     let uid = self.widget_uid();
    //     if !self.animation_open && self.animation_counter {
    //         if self.animator_handle_event(cx, event).must_redraw() {
    //             if self.animator.is_track_animating(cx, id!(open)) {
    //                 self.area.redraw(cx);
    //                 self.animation_counter = !self.animation_counter;
    //             }
    //         }
    //     }

    //     match event.hits(cx, self.area_header()) {
    //         Hit::FingerDown(_) => {
    //             if self.grab_key_focus {
    //                 cx.set_key_focus(sweep_area);
    //             }
    //         }
    //         Hit::FingerHoverIn(f_in) => {
    //             cx.set_cursor(self.prop.get(self.state).container.cursor);
    //             // cx.widget_action(uid, &scope.path, CollapseEvent::HoverIn);
    //         }
    //         Hit::FingerHoverOut(f_out) => {
    //             // cx.widget_action(uid, &scope.path, CollapseEvent::HoverOut(f_out.clone()));
    //         }
    //         Hit::FingerUp(f_up) => {
    //             self.active = !self.active;
    //             self.fold = self.active.to_f32() as f64;

    //             if self.active {
    //                 self.animator_play(cx, id!(open.on));
    //                 cx.widget_action(uid, &scope.path, CollapseEvent::Opened(f_up.clone()));
    //             } else {
    //                 self.animator_play(cx, id!(active.off));
    //                 cx.widget_action(uid, &scope.path, CollapseEvent::Closed(f_up.clone()));
    //             }
    //             self.animation_counter = !self.animation_counter;
    //         }
    //         _ => {}
    //     }

    //     if self.active {
    //         self.body.handle_event(cx, event, scope);
    //     }
    // }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if !self.animation_open && self.animation_counter {
            if self.animator_handle_event(cx, event).must_redraw() {
                if self.animator.is_track_animating(cx, id!(active)) {
                    self.area.redraw(cx);
                    self.animation_counter = !self.animation_counter;
                }
            }
        }

        match event.hits(cx, self.area_header()) {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
            }
            Hit::FingerHoverIn(meta) => {
                self.active_hover_in(cx, meta);
            }
            Hit::FingerHoverOut(meta) => {
                self.active_hover_out(cx, meta);
            }
            Hit::FingerUp(meta) => {
                self.active = !self.active;
                self.fold = self.active.to_f32() as f64;

                if self.active {
                    self.animator_play(cx, id!(active.on));
                } else {
                    self.animator_play(cx, id!(active.off));
                }
                self.active_changed(cx, Some(meta));
                self.animation_counter = true;
            }
            _ => {}
        }

        if self.active {
            self.body.handle_event(cx, event, scope);
        }

        // self.header.handle_event(cx, event, scope);
        // if let Event::Actions(actions) = event {
        //     match actions
        //         .find_widget_action(self.header.widget(id!(fold_button)).widget_uid())
        //         .cast()
        //     {
        //         FoldButtonAction::Opening => self.animator_play(cx, id!(open.on)),
        //         FoldButtonAction::Closing => self.animator_play(cx, id!(open.off)),
        //         _ => (),
        //     }
        // }
    }
}

impl WidgetNode for GCollapse {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for slot in [&self.header, &self.body] {
            for (_, child) in slot.children.iter() {
                let x = child.uid_to_widget(uid);
                if !x.is_empty() {
                    return x;
                }
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for slot in [&self.header, &self.body] {
            for (_, child) in &slot.children {
                child.find_widgets(path, cached, results);
            }
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.state);
        prop.container.walk()
    }

    fn area(&self) -> Area {
        self.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_collapse.redraw(cx);
        for (visible, slot) in [
            (self.header.visible, &mut self.header),
            (self.body.visible, &mut self.body),
        ] {
            if visible {
                slot.redraw(cx);
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

impl LiveHook for GCollapse {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicProp::live_props();
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
                (CollapsePart::Container, &live_props),
                (CollapsePart::Header, &live_props),
                (CollapsePart::Body, &live_props),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(CollapseState::Basic, applys);
                }
                HOVER => {
                    component
                        .apply_slot_map
                        .insert(CollapseState::Hover, applys);
                }
                ACTIVE => {
                    component
                        .apply_slot_map
                        .insert(CollapseState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(CollapseState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl GCollapse {
    active_event! {
        active_hover_in: CollapseEvent::HoverIn |meta: FingerHoverEvent| => CollapseHoverIn {meta},
        active_hover_out: CollapseEvent::HoverOut |meta: FingerHoverEvent| => CollapseHoverOut {meta}
    }
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    CollapseEvent::Changed(CollapseChanged {
                        meta,
                        active: self.active,
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: CollapseEvent::HoverIn => CollapseHoverIn,
        hover_out: CollapseEvent::HoverOut => CollapseHoverOut
    }
    area! {
        area_header, header,
        area_body, body
    }
}

impl SlotComponent<CollapseState> for GCollapse {
    type Part = CollapsePart;
}

impl Component for GCollapse {
    type Error = Error;

    type State = CollapseState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.collapse;
        self.prop = prop.clone();
        self.header.prop.basic = self.prop.basic.header;
        self.header.prop.hover = self.prop.hover.header;
        self.header.prop.pressed = self.prop.active.header;
        self.header.prop.disabled = self.prop.disabled.header;
        self.body.prop.basic = self.prop.basic.body;
        self.body.prop.hover = self.prop.hover.body;
        self.body.prop.pressed = self.prop.active.body;
        self.body.prop.disabled = self.prop.disabled.body;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(CollapseState::Disabled);
        } else {
            if self.active {
                self.switch_state(CollapseState::Active);
            } else {
                self.switch_state(CollapseState::Basic);
            }
        }
        let state = self.state;
        let prop = self.prop.get(state);
        self.draw_collapse.merge(&prop.container);
        let _ = self.header.render(cx)?;
        let _ = self.body.render(cx)?;
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, CollapseState::Disabled);
                cx.set_cursor(self.prop.get(self.state).container.cursor);
            }
            _ => {}
        }
    }
    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        ()
    }

    // fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
    //     animation_open_then_redraw!(self, cx, event);
    //     if !self.active {
    //         match hit {
    //             Hit::FingerDown(_) => {
    //                 if self.grab_key_focus {
    //                     cx.set_key_focus(area);
    //                 }
    //             }
    //             Hit::FingerHoverIn(e) => {
    //                 cx.set_cursor(self.prop.get(self.state).container.cursor);
    //                 self.switch_state_with_animation(cx, CollapseState::Hover);
    //                 hit_hover_in!(self, cx, e);
    //             }
    //             Hit::FingerHoverOut(e) => {
    //                 self.switch_state_with_animation(cx, CollapseState::Basic);
    //                 hit_hover_out!(self, cx, e);
    //             }
    //             Hit::FingerUp(e) => {
    //                 if e.is_over {
    //                     if e.has_hovers() {
    //                         self.active = true;
    //                         self.switch_state_with_animation(cx, CollapseState::Active);
    //                         self.play_animation(cx, id!(hover.active));
    //                     } else {
    //                         self.switch_state_with_animation(cx, CollapseState::Basic);
    //                         self.play_animation(cx, id!(hover.off));
    //                     }
    //                     self.active_clicked(cx, Some(e));
    //                 } else {
    //                     self.switch_state_with_animation(cx, CollapseState::Basic);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.header.switch_state(state.into());
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();
        for (part, slot) in [
            (CollapsePart::Header, &mut self.header),
            (CollapsePart::Body, &mut self.body),
        ] {
            crossed_map.remove(&part).map(|map| {
                slot.apply_state_map.merge(map.to_state());
            });
            slot.focus_sync();
        }

        // sync state if is not Basic
        self.prop.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().collapse;
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
            let basic_prop = self.prop.get(CollapseState::Basic);
            let hover_prop = self.prop.get(CollapseState::Hover);
            let active_prop = self.prop.get(CollapseState::Active);
            let disabled_prop = self.prop.get(CollapseState::Disabled);
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
                nodes: draw_collapse = {
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
        } else {
            let state = self.state;
            let prop = self.prop.get(state);
            let index = match state {
                CollapseState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                CollapseState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                CollapseState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(active).as_instance(),
                    ],
                ),
                CollapseState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_collapse = {
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
        }
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

#[derive(Clone, Copy)]
pub enum DrawCollapseState {
    DrawHeader,
    DrawBody,
}
