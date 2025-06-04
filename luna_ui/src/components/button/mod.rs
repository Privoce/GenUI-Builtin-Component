mod prop;

pub use prop::*;

use makepad_widgets::*;

use crate::{
    animation_open_then_redraw, components::traits::{Component, Prop}, error::Error, hit_finger_down, hit_finger_up, hit_hover_in, hit_hover_out, pure_after_apply, set_scope_path, shader::draw_view::DrawView, themes::{Conf, Theme}, utils::BoolToF32
};

live_design! {
    link luna_basic;

    pub LButtonBase = {{LButton}} {
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (0.25)}}
                    apply: {
                        draw_button: {hover: 0.0, pressed: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (0.25)},
                        pressed: Forward {duration: (0.25)}
                    }
                    apply: {
                        draw_button: {hover: 1.0, pressed: 0.0}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (0.25)}}
                    apply: {
                        draw_button: {hover: 0.0, pressed: 1.0}
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
    pub grap_key_focus: bool,
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
    #[live]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
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
        let focus_area = self.area();
        let hit = event.hits(cx, focus_area);
        self.handle_widget_event(cx, event, hit, focus_area);
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
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerHoverIn(e) => {
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                hit_finger_up!(self, cx, e);
            }
            _ => (),
        }
    }

    set_scope_path!();
}
