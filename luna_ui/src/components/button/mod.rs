mod event;
mod prop;
use std::collections::HashMap;

pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        lifecycle::LifeCycle,
        traits::{Component, Prop},
    },
    error::Error,
    hit_finger_down, hit_finger_up, hit_hover_in, hit_hover_out, play_animation,
    prop::{
        manuel::{BASIC, HOVER, PRESSED},
        traits::ToFloat,
        ApplyStateMap,
    },
    pure_after_apply, set_animation, set_scope_path,
    shader::draw_view::DrawView,
    themes::Conf,
    ComponentAnInit,
};

live_design! {
    link luna_basic;
    use link::luna_animation_prop::*;

    pub LButtonBase = {{LButton}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_button: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                        ease: InOutQuad,
                    },
                    apply: {
                       draw_button: <AN_DRAW_VIEW> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_button: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct LButton {
    #[live]
    pub prop: ButtonProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    // --- others -------------------
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    apply_state_map: ApplyStateMap<ButtonState>,
    // --- draw ----------------------
    #[live]
    pub slot: WidgetRef,
    #[live]
    pub draw_button: DrawView,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    /// sync other state props (except related to theme) from `basic` state]
    /// means: if you set basic prop that `border_radius: 10.0`, then other state like `hover` or `pressed`
    /// will have the same `border_radius: 10.0` if you set this to true. (default is true)
    #[live(true)]
    pub sync: bool,
}

impl WidgetNode for LButton {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        self.slot.uid_to_widget(uid)
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.slot.find_widgets(path, cached, results);
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.current_state());
        Walk {
            margin: prop.margin,
            width: prop.width,
            height: prop.height,
            ..Default::default()
        }
    }

    fn area(&self) -> Area {
        self.draw_button.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_button.redraw(cx);
        if self.slot.visible() {
            self.slot.redraw(cx);
        }
    }
}

impl Widget for LButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let state = self.current_state();
        let prop = self.prop.get(state);
        let _ = self.draw_button.begin(
            cx,
            Walk {
                margin: prop.margin,
                width: prop.width,
                height: prop.height,
                ..Default::default()
            },
            Layout {
                clip_x: false,
                clip_y: false,
                padding: prop.padding,
                align: prop.align,
                flow: prop.flow,
                spacing: prop.spacing,
                ..Default::default()
            },
        );

        if self.slot.visible() {
            let slot_walk = self.slot.walk(cx);
            let _ = self.slot.draw_walk(cx, scope, slot_walk);
        }

        self.draw_button.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().button = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        self.handle_widget_event(cx, event, hit, area);
    }
}

impl LiveHook for LButton {
    // pure_after_apply!();
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        self.sync();
        self.render_after_apply(cx);
    }

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if !self.lifecycle.is_created() {
            return;
        }

        self.index = index;
        let live_props = [
            live_id!(theme),
            live_id!(background_color),
            live_id!(border_color),
            live_id!(border_radius),
            live_id!(border_width),
            live_id!(shadow_color),
            live_id!(spread_radius),
            live_id!(blur_radius),
            live_id!(shadow_offset),
            live_id!(background_visible),
        ];
        for prefix in [live_id!(basic), live_id!(hover), live_id!(pressed)] {
            let mut applys = HashMap::new();
            for path in live_props {
                if let Some(i) = nodes.child_by_path(
                    index,
                    &[
                        live_id!(prop).as_field(),
                        prefix.as_field(),
                        path.as_field(),
                    ],
                ) {
                    let node = &nodes[i];
                    applys.insert(node.id.to_string(), node.value.clone());
                }
            }
            match prefix.to_string().as_str() {
                BASIC => {
                    self.apply_state_map.insert(ButtonState::Basic, applys);
                }
                HOVER => {
                    self.apply_state_map.insert(ButtonState::Hover, applys);
                }
                PRESSED => {
                    self.apply_state_map.insert(ButtonState::Pressed, applys);
                }
                _ => {}
            }
        }
    }
}

impl Component for LButton {
    type Error = Error;

    type State = ButtonState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.button;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.current_state();

        self.draw_button.background_color = self.prop.get(state).background_color.into();
        self.draw_button.border_color = self.prop.get(state).border_color.into();
        self.draw_button.shadow_color = self.prop.get(state).shadow_color.into();
        self.draw_button.border_radius = self.prop.get(state).border_radius.into();
        self.draw_button.border_width = self.prop.get(state).border_width;
        self.draw_button.spread_radius = self.prop.get(state).spread_radius;
        self.draw_button.blur_radius = self.prop.get(state).blur_radius;
        self.draw_button.shadow_offset = self.prop.get(state).shadow_offset;
        self.draw_button.background_visible = self.prop.get(state).background_visible.to_f32();

        Ok(())
    }

    fn current_state(&self) -> Self::State {
        if self.disabled {
            ButtonState::Disabled
        } else {
            self.draw_button.current_state().into()
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, ButtonState::Pressed);
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.prop.get(self.current_state()).cursor);
                self.switch_state_with_animation(cx, ButtonState::Hover);
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, ButtonState::Basic);
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, ButtonState::Hover);
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.switch_state_with_animation(cx, ButtonState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.switch_state_with_animation(cx, ButtonState::Basic);
                    hit_finger_up!(self, cx, e);
                }
            }
            _ => {}
        };
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0
            },
        );
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        match state {
            ButtonState::Basic => {
                if self.draw_button.hover != 0.0 || self.draw_button.pressed != 0.0 {
                    self.draw_button.hover = 0.0;
                    self.draw_button.pressed = 0.0;
                }
            }
            ButtonState::Hover => {
                if self.draw_button.hover != 1.0 {
                    self.draw_button.hover = 1.0;
                    self.draw_button.pressed = 0.0;
                }
            }
            ButtonState::Pressed => {
                if self.draw_button.pressed != 1.0 {
                    self.draw_button.hover = 0.0;
                    self.draw_button.pressed = 1.0;
                }
            }
            ButtonState::Disabled => {}
        }
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    // sync props if not set in DSL, depend on `self.sync` is true
    fn sync(&mut self) {
        if !self.sync {
            return;
        }
        // sync state if is not Basic
        self.prop.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().button;

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
            let basic_prop = self.prop.get(ButtonState::Basic);
            let hover_prop = self.prop.get(ButtonState::Hover);
            let pressed_prop = self.prop.get(ButtonState::Pressed);
            let (mut basic_index, mut hover_index, mut pressed_index) = (None, None, None);
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
                    live_id!(pressed).as_instance(),
                ],
            ) {
                pressed_index = Some(index);
            }

            set_animation! {
                nodes: draw_button = {
                    basic_index => {
                        background_color => basic_prop.background_color,
                        border_color =>basic_prop.border_color,
                        border_radius => basic_prop.border_radius,
                        border_width =>(basic_prop.border_width as f64),
                        shadow_color => basic_prop.shadow_color,
                        spread_radius => (basic_prop.spread_radius as f64),
                        blur_radius => (basic_prop.blur_radius as f64),
                        shadow_offset => basic_prop.shadow_offset,
                        background_visible => basic_prop.background_visible.to_f64()
                    },
                    hover_index => {
                        background_color => hover_prop.background_color,
                        border_color => hover_prop.border_color,
                        border_radius => hover_prop.border_radius,
                        border_width => (hover_prop.border_width as f64),
                        shadow_color => hover_prop.shadow_color,
                        spread_radius => (hover_prop.spread_radius as f64),
                        blur_radius => (hover_prop.blur_radius as f64),
                        shadow_offset => hover_prop.shadow_offset,
                        background_visible => hover_prop.background_visible.to_f64()
                    },
                    pressed_index => {
                        background_color => pressed_prop.background_color,
                        border_color => pressed_prop.border_color,
                        border_radius => pressed_prop.border_radius,
                        border_width => (pressed_prop.border_width as f64),
                        shadow_color => pressed_prop.shadow_color,
                        spread_radius => (pressed_prop.spread_radius as f64),
                        blur_radius => (pressed_prop.blur_radius as f64),
                        shadow_offset => pressed_prop.shadow_offset,
                        background_visible => pressed_prop.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.current_state();
            let prop = self.prop.get(state);
            let index = match state {
                ButtonState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                ButtonState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                ButtonState::Pressed => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(pressed).as_instance(),
                    ],
                ),
                ButtonState::Disabled => None,
            };
            set_animation! {
                nodes: draw_button = {
                    index => {
                        background_color => prop.background_color,
                        border_color => prop.border_color,
                        border_radius => prop.border_radius,
                        border_width => (prop.border_width as f64),
                        shadow_color => prop.shadow_color,
                        spread_radius => (prop.spread_radius as f64),
                        blur_radius => (prop.blur_radius as f64),
                        shadow_offset => prop.shadow_offset,
                        background_visible => prop.background_visible.to_f64()
                    }
                }
            }
        }
    }

    play_animation!();
    set_scope_path!();
}

impl LButton {
    active_event! {
        active_hover_in: ButtonEvent::HoverIn |meta: FingerHoverEvent| => ButtonHoverIn { meta },
        active_hover_out: ButtonEvent::HoverOut |meta: FingerHoverEvent| => ButtonHoverOut { meta },
        active_finger_up: ButtonEvent::FingerUp |meta: FingerUpEvent| => ButtonFingerUp { meta },
        active_finger_down: ButtonEvent::FingerDown |meta: FingerDownEvent| => ButtonFingerDown { meta },
        active_clicked: ButtonEvent::Clicked |meta: FingerUpEvent| => ButtonClicked { meta }
    }
}
