mod event;
mod prop;

pub use event::*;
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop, SlotComponent, SlotProp},
        view::ViewBasicProp,
    },
    error::Error,
    event_option, hit_finger_down, hit_finger_up, hit_hover_in, hit_hover_out, lifecycle,
    makepad_derive_widget::*,
    makepad_draw::*,
    play_animation,
    prop::{
        manuel::{BASIC, HOVER, PRESSED},
        traits::ToFloat,
        ApplySlotMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::{draw_svg::DrawSvg, draw_view::DrawView},
    themes::Conf,
    visible,
    widget::*,
    ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSvgBase = {{GSvg}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_svg: <AN_DRAW_SVG> {},
                        draw_svg_container: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_svg: <AN_DRAW_SVG> {},
                       draw_svg_container: <AN_DRAW_VIEW> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_svg: <AN_DRAW_SVG> {},
                        draw_svg_container: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSvg {
    #[live]
    pub prop: SvgProp,
    #[live]
    pub src: LiveDependency,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub disabled: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<SvgState, SvgPart>,
    // --- draw ----------------------
    #[live]
    pub draw_svg: DrawSvg,
    #[live]
    pub draw_svg_container: DrawView,
    // --- init ----------------------
    #[rust]
    lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    // --- animator ----------------
    // default animation is off
    #[live(false)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    #[live(true)]
    pub event_open: bool,
    #[live]
    pub grab_key_focus: bool,
    #[rust]
    pub state: SvgState,
}

impl Widget for GSvg {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().svg = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let prop = self.prop.get(self.state);
        self.draw_svg_container.begin(cx, prop.container.walk(), prop.container.layout());
        self.draw_svg.draw_walk(cx, prop.svg.walk());
        self.draw_svg_container.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
}

impl WidgetNode for GSvg {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.state);
        prop.walk()
    }

    fn area(&self) -> Area {
        self.draw_svg.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_svg.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl LiveHook for GSvg {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            nodes,
            index,
            [live_id!(basic), live_id!(hover), live_id!(pressed)],
            [
                (SvgPart::Container, &ViewBasicProp::live_props()),
                (SvgPart::Svg, &SvgPartProp::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(SvgState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(SvgState::Hover, applys);
                }
                PRESSED => {
                    component.apply_slot_map.insert(SvgState::Pressed, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<SvgState> for GSvg {
    type Part = SvgPart;
}

impl Component for GSvg {
    type Error = Error;

    type State = SvgState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.svg;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let prop = self.prop.get(self.state);
        self.draw_svg_container.merge(&prop.container);
        self.draw_svg.merge(&prop.svg);
        self.draw_svg.svg_file = self.src.clone();
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, SvgState::Pressed);
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.prop.get(self.state).container.cursor);
                self.switch_state_with_animation(cx, SvgState::Hover);
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, SvgState::Basic);
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, SvgState::Hover);
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.switch_state_with_animation(cx, SvgState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.switch_state_with_animation(cx, SvgState::Basic);
                    hit_finger_up!(self, cx, e);
                }
            }
            _ => {}
        };
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(self.prop.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        self.draw_svg.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0,
            },
        );
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        // match state {
        //     SvgState::Basic => {
        //         self.draw_svg.state_basic();
        //     }
        //     SvgState::Hover => {
        //         self.draw_svg.state_hover();
        //     }
        //     SvgState::Pressed => {
        //         self.draw_svg.state_pressed();
        //     }
        //     SvgState::Disabled => {}
        // }
        self.state = state;
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
        self.prop.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().svg;

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
            let basic_prop = self.prop.get(SvgState::Basic);
            let hover_prop = self.prop.get(SvgState::Hover);
            let pressed_prop = self.prop.get(SvgState::Pressed);
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
                nodes: draw_svg = {
                    basic_index => {
                        color => basic_prop.svg.color
                    },
                    hover_index => {
                        color => hover_prop.svg.color
                    },
                    pressed_index => {
                        color => pressed_prop.svg.color
                    }
                }
            }
            set_animation! {
                nodes: draw_svg_container = {
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
                    pressed_index => {
                        background_color => pressed_prop.container.background_color,
                        border_color => pressed_prop.container.border_color,
                        border_radius => pressed_prop.container.border_radius,
                        border_width => (pressed_prop.container.border_width as f64),
                        shadow_color => pressed_prop.container.shadow_color,
                        spread_radius => (pressed_prop.container.spread_radius as f64),
                        blur_radius => (pressed_prop.container.blur_radius as f64),
                        shadow_offset => pressed_prop.container.shadow_offset,
                        background_visible => pressed_prop.container.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let prop = self.prop.get(state);
            let index = match state {
                SvgState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                SvgState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                SvgState::Pressed => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(pressed).as_instance(),
                    ],
                ),
                SvgState::Disabled => None,
            };
            set_animation! {
                nodes: draw_svg = {
                    index => {
                        color => prop.svg.color
                    }
                }
            }
            set_animation! {
                nodes: draw_svg_container = {
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

    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl GSvg {
    active_event! {
        active_hover_in: SvgEvent::HoverIn |meta: FingerHoverEvent| => SvgHoverIn { meta },
        active_hover_out: SvgEvent::HoverOut |meta: FingerHoverEvent| => SvgHoverOut { meta },
        active_finger_up: SvgEvent::FingerUp |meta: FingerUpEvent| => SvgFingerUp { meta },
        active_finger_down: SvgEvent::FingerDown |meta: FingerDownEvent| => SvgFingerDown { meta },
        active_clicked: SvgEvent::Clicked |meta: FingerUpEvent| => SvgClicked { meta }
    }
    event_option! {
        hover_in: SvgEvent::HoverIn => SvgHoverIn,
        hover_out: SvgEvent::HoverOut => SvgHoverOut,
        finger_up: SvgEvent::FingerUp => SvgFingerUp,
        finger_down: SvgEvent::FingerDown => SvgFingerDown,
        clicked: SvgEvent::Clicked => SvgClicked
    }
}
