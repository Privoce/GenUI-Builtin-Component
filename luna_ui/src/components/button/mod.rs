mod event;
mod prop;
pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::traits::{Component, Prop},
    error::Error,
    hit_finger_down, hit_finger_up, hit_hover_in, hit_hover_out, play_animation, pure_after_apply,
    set_scope_path,
    shader::draw_view::DrawView,
    themes::{Conf, Theme},
    utils::BoolToF32,
};

live_design! {
    link luna_basic;

    pub LButtonBase = {{LButton}} {
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (0.65)}}
                    apply: {
                        draw_button: {background_color:#fff}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (0.65)},
                        pressed: Forward {duration: (0.65)}
                    }
                    apply: {
                       draw_button: {background_color:#000}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (0.65)}}
                    apply: {
                        draw_button: {background_color:#f00}
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
        self.setup_animations(cx);
        let mut target_nodes = LiveNodeVec::new();
        target_nodes.push_live(live_object! {
            draw_button: {
                background_color: (vec4(0.0, 0.0, 1.0, 1.0)),
            }
        });
        if let Some(animator_state) = &mut self.animator.state {
            animator_state.replace_or_insert_last_node_by_path(
                0,
                &[
                    live_id!(hover).as_field(),
                    live_id!(on).as_field(),
                    live_id!(apply).as_field(),
                ],
                &target_nodes,
            );
        }
        let focus_area = self.area();
        let hit = event.hits(cx, focus_area);
        self.handle_widget_event(cx, event, hit, focus_area);
    }
}

impl LiveHook for LButton {
    // pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        self.render_after_apply(cx);
        
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
                // hit_finger_down!(self, cx, area, e);
                self.animator_play(cx, id!(hover.pressed));
                self.switch_state_and_redraw(cx, ButtonState::Pressed);
            }
            Hit::FingerHoverIn(e) => {
                let prop = self.prop.get(self.current_state());
                cx.set_cursor(prop.cursor);
                cx.seconds_since_app_start();
                // hit_hover_in!(self, cx, e);

                self.animator_play(cx, id!(hover.on));
                self.switch_state_and_redraw(cx, ButtonState::Hover);
            }
            Hit::FingerHoverOut(e) => {
                // hit_hover_out!(self, cx, e);
                self.animator_play(cx, id!(hover.off));
                self.switch_state_and_redraw(cx, ButtonState::Basic);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                        self.switch_state_and_redraw(cx, ButtonState::Hover);
                    } else {
                        self.play_animation(cx, id!(hover.off));
                        self.switch_state_and_redraw(cx, ButtonState::Basic);
                    }
                    self.active_clicked(cx, e);
                } else {
                    hit_finger_up!(self, cx, e);
                    self.switch_state_and_redraw(cx, ButtonState::Basic);
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

    fn switch_state_and_redraw(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        // dbg!(state);
        // self.animate_to_state(cx, state);
        // if let ButtonState::Hover = state {
        //     dbg!(&self.animator.state);
        // }
        // self.redraw(cx);
        // match state {
        //     ButtonState::Basic => {
        //         if self.draw_button.hover != 0.0 || self.draw_button.pressed != 0.0 {
        //             self.draw_button.hover = 0.0;
        //             self.draw_button.pressed = 0.0;
        //         }
        //     }
        //     ButtonState::Hover => {
        //         if self.draw_button.hover != 1.0 {
        //             self.draw_button.hover = 1.0;
        //             self.draw_button.pressed = 0.0;
        //         }
        //     }
        //     ButtonState::Pressed => {
        //         if self.draw_button.pressed != 1.0 {
        //             self.draw_button.hover = 0.0;
        //             self.draw_button.pressed = 1.0;
        //         }
        //     }
        //     ButtonState::Disabled => {}
        // }

        // let _ = self.render(cx);
        // self.redraw(cx);
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

    /// 使用动画过渡到新状态
    fn animate_to_state(&mut self, cx: &mut Cx, state: ButtonState) -> () {
        let target_prop = self.prop.get(state);

        // 动态构建动画目标状态
        let mut target_nodes = LiveNodeVec::new();
        target_nodes.push_live(live_object! {
            draw_button: {
                background_color: (target_prop.background_color),
                border_color: (target_prop.border_color),
                border_radius: (Vec4::from(target_prop.border_radius)),
                border_width: (target_prop.border_width),
                shadow_color: (Vec4::from(target_prop.shadow_color)),
                spread_radius: (target_prop.spread_radius),
                blur_radius: (target_prop.blur_radius),
                shadow_offset: (target_prop.shadow_offset),
                background_visible: (target_prop.background_visible.to_f32())
            }
        });

        // 根据状态设置动画状态值和播放对应动画
        match state {
            ButtonState::Basic => {
                // 更新动画器状态
                if let Some(animator_state) = &mut self.animator.state {
                    animator_state.replace_or_insert_last_node_by_path(
                        0,
                        &[
                            live_id!(hover).as_field(),
                            live_id!(off).as_field(),
                            live_id!(apply).as_field(),
                        ],
                        &target_nodes,
                    );
                }

                self.animator_play(cx, id!(hover.off));
            }
            ButtonState::Hover => {
                if let Some(animator_state) = &mut self.animator.state {
                    animator_state.replace_or_insert_last_node_by_path(
                        0,
                        &[
                            live_id!(hover).as_field(),
                            live_id!(on).as_field(),
                            live_id!(apply).as_field(),
                        ],
                        &target_nodes,
                    );
                }

                self.animator_play(cx, id!(hover.on));
            }
            ButtonState::Pressed => {
                if let Some(animator_state) = &mut self.animator.state {
                    animator_state.replace_or_insert_last_node_by_path(
                        0,
                        &[
                            live_id!(hover).as_field(),
                            live_id!(pressed).as_field(),
                            live_id!(apply).as_field(),
                        ],
                        &target_nodes,
                    );
                }

                self.animator_play(cx, id!(hover.pressed));
            }
            ButtonState::Disabled => {
                // 禁用状态可能不需要动画，直接应用
                // self.apply_state_immediately(cx, state);
                return;
            }
        }
    }

    // 预设动画配置到动画器
    pub fn setup_animations(&mut self, cx: &mut Cx) {
        // 为每个状态预设基础的动画配置
        let basic_prop = self.prop.get(ButtonState::Basic);
        let hover_prop = self.prop.get(ButtonState::Hover);
        let pressed_prop = self.prop.get(ButtonState::Pressed);

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

        if let Some(index) = basic_index {
            if let Some(v_index) = nodes.child_by_path(index, &[
                live_id!(apply).as_field(),
                live_id!(draw_button).as_field(),
                live_id!(background_color).as_field(),
            ]) {
                nodes[v_index].value = LiveValue::Color(vec4_to_u32(basic_prop.background_color));
            }
        }

        if let Some(index) = hover_index {
            if let Some(v_index) = nodes.child_by_path(index, &[
                live_id!(apply).as_field(),
                live_id!(draw_button).as_field(),
                live_id!(background_color).as_field(),
            ]) {
                nodes[v_index].value = LiveValue::Color(vec4_to_u32(hover_prop.background_color));
            }
        }

        if let Some(index) = pressed_index {
            if let Some(v_index) = nodes.child_by_path(index, &[
                live_id!(apply).as_field(),
                live_id!(draw_button).as_field(),
                live_id!(background_color).as_field(),
            ]) {
                nodes[v_index].value = LiveValue::Color(vec4_to_u32(pressed_prop.background_color));
            }
        }

        // 设置基础状态的动画配置
        // if let Some(animator_state) = &mut self.animator.state {
        // animator_state.push_live(live!{
        //     state: {
        //         draw_button: {background_color: #f00}
        //     }
        // });

        // let mut basic_nodes = LiveNodeVec::new();
        // basic_nodes.push_live(live_object! {
        //     draw_button: {
        //         background_color: (basic_prop.background_color),
        //         border_color: (basic_prop.border_color),
        //     }
        // });

        // let mut hover_nodes = LiveNodeVec::new();
        // hover_nodes.push_live(live_object! {
        //     draw_button: {
        //         background_color: (hover_prop.background_color),
        //         border_color: (hover_prop.border_color),
        //     }
        // });

        // let mut pressed_nodes = LiveNodeVec::new();
        // pressed_nodes.push_live(live_object! {
        //     draw_button: {
        //         background_color: (pressed_prop.background_color),
        //         border_color: (pressed_prop.border_color),
        //     }
        // });

        // // 将这些配置设置到动画器中
        // animator_state.replace_or_insert_last_node_by_path(
        //     0,
        //     &[
        //         live_id!(hover).as_field(),
        //         live_id!(off).as_field(),
        //         live_id!(apply).as_field(),
        //     ],
        //     &basic_nodes,
        // );

        // animator_state.replace_or_insert_last_node_by_path(
        //     0,
        //     &[
        //         live_id!(hover).as_field(),
        //         live_id!(on).as_field(),
        //         live_id!(apply).as_field(),
        //     ],
        //     &hover_nodes,
        // );

        // animator_state.replace_or_insert_last_node_by_path(
        //     0,
        //     &[
        //         live_id!(hover).as_field(),
        //         live_id!(pressed).as_field(),
        //         live_id!(apply).as_field(),
        //     ],
        //     &pressed_nodes,
        // );
        // }
    }
}

fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}

fn vec4_to_u32(v: Vec4) -> u32 {
    rgba_to_u32(
        (v.x * 255.0) as u8,
        (v.y * 255.0) as u8,
        (v.z * 255.0) as u8,
        (v.w * 255.0) as u8,
    )
}