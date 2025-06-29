mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop, SlotComponent, SlotProp},
        view::{LView, ViewState},
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        manuel::{BASIC, HOVER},
        traits::ToFloat,
        ApplySlotMap, ApplyStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    themes::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link luna_basic;
    use link::luna_animation_prop::*;

    pub LCardBase = {{LCard}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_card: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        ease: InOutQuad,
                    },
                    apply: {
                       draw_card: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct LCard {
    #[live]
    pub prop: CardProp,
    // --- others -------------------
    #[live(true)]
    pub visible: bool,
    // card can not be disabled, cause it is a container
    // #[live]
    // pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    apply_slot_map: ApplySlotMap<CardState, CardPart>,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    // --- slots -------------------
    #[live]
    pub header: LView,
    #[live]
    pub body: LView,
    #[live]
    pub footer: LView,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[live]
    pub draw_card: DrawView,
}

impl WidgetNode for LCard {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for slot in [&self.header, &self.body, &self.footer] {
            for (_, child) in &slot.children {
                let x = child.uid_to_widget(uid);
                if !x.is_empty() {
                    return x;
                }
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for slot in [&self.header, &self.body, &self.footer] {
            for (_, child) in &slot.children {
                child.find_widgets(path, cached, results);
            }
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.current_state());
        Walk {
            abs_pos: prop.outer.abs_pos,
            margin: prop.outer.margin,
            width: prop.outer.width,
            height: prop.outer.height,
        }
    }

    fn area(&self) -> Area {
        self.draw_card.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);

        for (visible, slot) in [
            (self.header.visible, &mut self.header),
            (self.body.visible, &mut self.body),
            (self.footer.visible, &mut self.footer),
        ] {
            if visible {
                slot.redraw(cx);
            }
        }
    }
    visible!();
}

impl Widget for LCard {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let state = self.current_state();
        let prop = self.prop.get(state);

       

        let _ = self.draw_card.begin(
            cx,
            Walk {
                margin: prop.outer.margin,
                width: prop.outer.width,
                height: prop.outer.height,
                abs_pos: prop.outer.abs_pos,
            },
            Layout {
                clip_x: false,
                clip_y: false,
                padding: prop.outer.padding,
                align: prop.outer.align,
                flow: prop.outer.flow,
                spacing: prop.outer.spacing,
                ..Default::default()
            },
        );

         if self.header.visible {
            let walk = prop.body.walk();
            dbg!(prop.header.walk());
            let _ = self.header.draw_walk(
                cx,
                scope,
                prop.header.walk(),
            );
        }
        // for (visible, slot, walk) in [
        //     (self.header.visible, &mut self.header, prop.header.walk()),
        //     (self.body.visible, &mut self.body, prop.body.walk()),
        //     (self.footer.visible, &mut self.footer, prop.footer.walk()),
        // ] {
        //     if visible {
        //         let _ = slot.draw_walk(cx, scope, walk);
        //     }
        // }

        if self.body.visible {
            let walk = prop.body.walk();
            dbg!(walk);
            let _ = self.body.draw_walk(
                cx,
                scope,
                walk,
            );
        }

        if self.footer.visible {
            let _ = self.footer.draw_walk(
                cx,
                scope,
                prop.footer.walk(),
            );
        }

        self.draw_card.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl LiveHook for LCard {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            nodes,
            index,
            &CardBasicProp::live_props(),
            [live_id!(basic), live_id!(hover)],
            [
                CardPart::Outer,
                CardPart::Header,
                CardPart::Body,
                CardPart::Footer,
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(CardState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(CardState::Hover, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for LCard {
    type Error = Error;

    type State = CardState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.card;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.current_state();
        self.draw_card.background_color = self.prop.get(state).outer.background_color;
        self.draw_card.border_color = self.prop.get(state).outer.border_color;
        self.draw_card.border_width = self.prop.get(state).outer.border_width;
        self.draw_card.border_radius = self.prop.get(state).outer.border_radius.into();
        self.draw_card.shadow_color = self.prop.get(state).outer.shadow_color;
        self.draw_card.spread_radius = self.prop.get(state).outer.spread_radius;
        self.draw_card.blur_radius = self.prop.get(state).outer.blur_radius;
        self.draw_card.shadow_offset = self.prop.get(state).outer.shadow_offset;
        self.draw_card.background_visible = self.prop.get(state).outer.background_visible.to_f32();
        self.draw_card.rotation = self.prop.get(state).outer.rotation;
        self.draw_card.scale = self.prop.get(state).outer.scale;
        Ok(())
    }

    fn current_state(&self) -> Self::State {
        self.draw_card.current_state().into()
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        self.draw_card.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        match state {
            CardState::Basic => {
                if self.draw_card.hover != 0.0 || self.draw_card.pressed != 0.0 {
                    self.draw_card.hover = 0.0;
                    self.draw_card.pressed = 0.0;
                }
            }
            CardState::Hover => {
                if self.draw_card.hover != 1.0 {
                    self.draw_card.hover = 1.0;
                    self.draw_card.pressed = 0.0;
                }
            }
        }
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        // sync state if is not Basic
        self.prop.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().card;

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
            let basic_prop = self.prop.get(CardState::Basic);
            let hover_prop = self.prop.get(CardState::Hover);
            let (mut basic_index, mut hover_index) = (None, None);
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

            set_animation! {
                nodes: draw_card = {
                    basic_index => {
                        background_color => basic_prop.outer.background_color,
                        border_color =>basic_prop.outer.border_color,
                        border_radius => basic_prop.outer.border_radius,
                        border_width =>(basic_prop.outer.border_width as f64),
                        shadow_color => basic_prop.outer.shadow_color,
                        spread_radius => (basic_prop.outer.spread_radius as f64),
                        blur_radius => (basic_prop.outer.blur_radius as f64),
                        shadow_offset => basic_prop.outer.shadow_offset,
                        background_visible => basic_prop.outer.background_visible.to_f64()
                    },
                    hover_index => {
                        background_color => hover_prop.outer.background_color,
                        border_color => hover_prop.outer.border_color,
                        border_radius => hover_prop.outer.border_radius,
                        border_width => (hover_prop.outer.border_width as f64),
                        shadow_color => hover_prop.outer.shadow_color,
                        spread_radius => (hover_prop.outer.spread_radius as f64),
                        blur_radius => (hover_prop.outer.blur_radius as f64),
                        shadow_offset => hover_prop.outer.shadow_offset,
                        background_visible => hover_prop.outer.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.current_state();
            let prop = self.prop.get(state);
            let index = match state {
                CardState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                CardState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_card = {
                    index => {
                        background_color => prop.outer.background_color,
                        border_color => prop.outer.border_color,
                        border_radius => prop.outer.border_radius,
                        border_width => (prop.outer.border_width as f64),
                        shadow_color => prop.outer.shadow_color,
                        spread_radius => (prop.outer.spread_radius as f64),
                        blur_radius => (prop.outer.blur_radius as f64),
                        shadow_offset => prop.outer.shadow_offset,
                        background_visible => prop.outer.background_visible.to_f64()
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

impl SlotComponent<ViewState> for LCard {
    type Part = CardPart;
}
