mod event;
mod prop;
pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::traits::{Component, Prop},
    error::Error,
    hit_finger_down, hit_finger_up, hit_hover_in, hit_hover_out, play_animation,
    prop::traits::{ToFloat, ToU32},
    pure_after_apply, set_animation, set_scope_path,
    shader::draw_view::DrawView,
    themes::{Conf, Theme},
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
    // --- draw ----------------------
    // #[find]
    // #[redraw]
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
    pub init: bool,
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
        let area = self.area();
        let hit = event.hits(cx, area);
        self.handle_widget_event(cx, event, hit, area);
    }
}

impl LiveHook for LButton {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
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
        self.draw_button.background_color = self.prop.get(state).background_color;
        self.draw_button.border_color = self.prop.get(state).border_color;
        self.draw_button.border_radius = self.prop.get(state).border_radius.into();
        self.draw_button.border_width = self.prop.get(state).border_width;
        self.draw_button.shadow_color = self.prop.get(state).shadow_color.into();
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

    pub fn set_animation(&mut self, cx: &mut Cx) {
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
        if !self.init {
            self.init = true;
            let basic_prop = self.prop.get(ButtonState::Basic);
            let hover_prop = self.prop.get(ButtonState::Hover);
            let pressed_prop = self.prop.get(ButtonState::Pressed);

            let (basic_index, hover_index, pressed_index) = nodes.iter().enumerate().fold(
                (None, None, None),
                |(mut basic_index, mut hover_index, mut pressed_index), (index, node)| {
                    if !matches!(node.value, LiveValue::Close) {
                        match node.id {
                            live_id!(off) => {
                                basic_index = Some(index);
                            }
                            live_id!(on) => {
                                hover_index = Some(index);
                            }
                            live_id!(pressed) => {
                                pressed_index = Some(index);
                            }
                            _ => {}
                        }
                    }
                    (basic_index, hover_index, pressed_index)
                },
            );
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
            let live_id = match state {
                ButtonState::Basic => live_id!(off),
                ButtonState::Hover => live_id!(on),
                ButtonState::Pressed => live_id!(pressed),
                ButtonState::Disabled => unreachable!("Disabled state should not be animated"),
            };
            let index = nodes
                .iter()
                .enumerate()
                .fold(None, |mut index_node, (index, node)| {
                    if !matches!(node.value, LiveValue::Close) {
                        if node.id == live_id {
                            index_node = Some(index);
                        }
                    }
                    index_node
                });

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
}
