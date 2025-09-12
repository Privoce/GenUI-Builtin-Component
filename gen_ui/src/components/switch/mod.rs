mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    active_event, animation_open_then_redraw, components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, Style},
    }, error::Error, event_option, lifecycle, play_animation, prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER_ACTIVE, HOVER_BASIC},
        traits::ToFloat,
        ApplyStateMap,
    }, pure_after_apply, set_animation, set_index, set_scope_path, shader::draw_switch::DrawSwitch, sync, themes::conf::Conf, visible, ComponentAnInit
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSwitchBase = {{GSwitch}} {
        animator: {
            active = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: Linear,
                    apply: {
                        draw_switch: <AN_DRAW_SWITCH> {}
                    }
                }

                on = {
                    from: {all: Forward {duration: (AN_DURATION),}},
                    ease: Linear,
                    apply: {
                       draw_switch: <AN_DRAW_SWITCH> {}
                    }
                }

                off_hover = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: Linear,
                    apply: {
                        draw_switch: <AN_DRAW_SWITCH> {}
                    }
                }

                on_hover = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: Linear,
                    apply: {
                        draw_switch: <AN_DRAW_SWITCH> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: Linear,
                    apply: {
                        draw_switch: <AN_DRAW_SWITCH> {}
                    }
                }
            },
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSwitch {
    // --- prop -------------------
    #[live]
    pub style: SwitchProp,
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
    apply_state_map: ApplyStateMap<SwitchState>,
    // --- draw -------------------
    #[live]
    pub draw_switch: DrawSwitch,
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
    #[live(false)]
    pub value: bool,
    #[rust]
    pub state: SwitchState,
}

impl WidgetNode for GSwitch {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_switch.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        if self.visible {
            let _ = self.render(cx);
            self.draw_switch.redraw(cx);
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

impl Widget for GSwitch {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.visible {
            let state = self.state;
            let style = self.style.get(state);

            self.draw_switch.begin(cx, walk, style.layout());
            self.draw_switch.end(cx);
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
        self.set_apply_state_map(
            nodes,
            index,
            &SwitchBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(hover_basic),
                live_id!(hover_active),
                live_id!(active),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(SwitchState::Basic, applys);
                }
                HOVER_BASIC => {
                    component
                        .apply_state_map
                        .insert(SwitchState::HoverBasic, applys);
                }
                HOVER_ACTIVE => {
                    component
                        .apply_state_map
                        .insert(SwitchState::HoverActive, applys);
                }
                ACTIVE => {
                    component
                        .apply_state_map
                        .insert(SwitchState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(SwitchState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GSwitch {
    type Error = Error;

    type State = SwitchState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.switch;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = if self.disabled {
            SwitchState::Disabled
        } else {
            if self.value {
                SwitchState::Active
            } else {
                SwitchState::Basic
            }
        };
        self.switch_state(state);
        let state = self.state;
        let style = self.style.get(state);
        self.draw_switch.merge(&style);
        self.draw_switch.active = self.value.to_f32();
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, SwitchState::Disabled);
                cx.set_cursor(self.style.get(self.state).cursor);
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
                cx.set_cursor(self.style.get(self.state).cursor);
                let (state, state_an) = if self.value {
                    (SwitchState::HoverActive, id!(active.on_hover))
                } else {
                    (SwitchState::HoverBasic, id!(active.off_hover))
                };
                self.switch_state_with_animation(cx, state);
                self.play_animation(cx, state_an);
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                let (state, state_an) = if self.value {
                    (SwitchState::Active, id!(active.on))
                } else {
                    (SwitchState::Basic, id!(active.off))
                };
                self.switch_state_with_animation(cx, state);
                self.play_animation(cx, state_an);
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        let (state_an, state) = if self.value {
                            (id!(active.off), SwitchState::Basic)
                        } else {
                            (id!(active.on), SwitchState::Active)
                        };
                        self.value = !self.value;
                        self.switch_state_with_animation(cx, state);
                        self.play_animation(cx, state_an);
                    } else {
                        self.switch_state_with_animation(cx, SwitchState::Basic);
                        self.play_animation(cx, id!(active.off));
                    }
                    self.active_clicked(cx, e.clone());
                    self.active_changed(cx, Some(e));
                } else {
                    self.switch_state_with_animation(cx, SwitchState::Basic);
                }
            }
            _ => {}
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
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
            let basic_prop = self.style.get(SwitchState::Basic);
            let hover_basic_prop = self.style.get(SwitchState::HoverBasic);
            let hover_active_prop = self.style.get(SwitchState::HoverActive);
            let active_prop = self.style.get(SwitchState::Active);
            let disabled_prop = self.style.get(SwitchState::Disabled);
            let (
                mut basic_index,
                mut hover_basic_index,
                mut hover_active_index,
                mut active_index,
                mut disabled_index,
            ) = (None, None, None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(off).as_instance(),
                ],
            ) {
                basic_index = Some(index);
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

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(off_hover).as_instance(),
                ],
            ) {
                hover_basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(on_hover).as_instance(),
                ],
            ) {
                hover_active_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(active).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_switch = {
                    basic_index => {
                        background_color => basic_prop.background_color,
                        border_color => basic_prop.border_color,
                        stroke_color => basic_prop.stroke_color,
                        border_radius => basic_prop.border_radius,
                        border_width => (basic_prop.border_width as f64),
                        background_visible => basic_prop.background_visible.to_f64(),
                        active => self.value.to_f64()
                    },
                    hover_basic_index => {
                        background_color => hover_basic_prop.background_color,
                        border_color => hover_basic_prop.border_color,
                        stroke_color => hover_basic_prop.stroke_color,
                        border_radius => hover_basic_prop.border_radius,
                        border_width => (hover_basic_prop.border_width as f64),
                        background_visible => hover_basic_prop.background_visible.to_f64(),
                        active => self.value.to_f64()
                    },
                    hover_active_index => {
                        background_color => hover_active_prop.background_color,
                        border_color => hover_active_prop.border_color,
                        stroke_color => hover_active_prop.stroke_color,
                        border_radius => hover_active_prop.border_radius,
                        border_width => (hover_active_prop.border_width as f64),
                        background_visible => hover_active_prop.background_visible.to_f64(),
                        active => self.value.to_f64()
                    },
                    active_index => {
                        background_color => active_prop.background_color,
                        border_color => active_prop.border_color,
                        stroke_color => active_prop.stroke_color,
                        border_radius => active_prop.border_radius,
                        border_width => (active_prop.border_width as f64),
                        background_visible => active_prop.background_visible.to_f64(),
                        active => self.value.to_f64()
                    },
                    disabled_index => {
                        background_color => disabled_prop.background_color,
                        border_color => disabled_prop.border_color,
                        stroke_color => disabled_prop.stroke_color,
                        border_radius => disabled_prop.border_radius,
                        border_width => (disabled_prop.border_width as f64),
                        background_visible => disabled_prop.background_visible.to_f64(),
                        active => self.value.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                SwitchState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                SwitchState::HoverBasic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(off_hover).as_instance(),
                    ],
                ),
                SwitchState::HoverActive => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(on_hover).as_instance(),
                    ],
                ),
                SwitchState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                SwitchState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_switch = {
                    index => {
                        background_color => style.background_color,
                        border_color => style.border_color,
                        border_radius => style.border_radius,
                        border_width => (style.border_width as f64),
                        stroke_color => style.stroke_color,
                        background_visible => style.background_visible.to_f64(),
                        active => self.value.to_f64()
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

impl GSwitch {
    active_event! {
        active_hover_in: SwitchEvent::HoverIn |meta: FingerHoverEvent| => SwitchHoverIn { meta },
        active_hover_out: SwitchEvent::HoverOut |meta: FingerHoverEvent| => SwitchHoverOut { meta }
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, meta: FingerUpEvent) -> () {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    SwitchEvent::Clicked(SwitchClicked {
                        value: self.value,
                        meta,
                    }),
                );
            });
        }
    }
    /// This function is called when the switch value changes. (happend when setter is called)
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) -> () {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    SwitchEvent::Changed(SwitchChanged {
                        value: self.value,
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
    // pub fn toggle(&mut self, cx: &mut Cx, active: bool, init: bool) -> () {
    //     self.active = active;
    //     let (state, hover_id) = match (active, init) {
    //         (true, false) => (SwitchState::Active, Some(id!(hover.active))),
    //         (true, true) => (SwitchState::Active, None),
    //         (false, true) => (SwitchState::Basic, None),
    //         (false, false) => (SwitchState::Basic, Some(id!(hover.off))),
    //     };
    //     self.switch_state(state);
    //     if let Some(hover_id) = hover_id {
    //         self.play_animation(cx, hover_id);
    //     }
    //     self.active_clicked(cx, None);
    // }
}
