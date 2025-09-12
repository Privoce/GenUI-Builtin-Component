mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    active_event, animation_open_then_redraw, area, area_ref, components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, Style, SlotComponent, SlotStyle},
        view::{GView, ViewBasicStyle, ViewState},
    }, error::Error, event_option, event_option_ref, getter_setter_ref, hit_hover_in, hit_hover_out, lifecycle, play_animation, prop::{
        manuel::{BASIC, HOVER},
        traits::ToFloat,
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, DeferWalks, SlotDrawer, ToStateMap,
    }, pure_after_apply, set_animation, set_index, set_scope_path, shader::draw_view::DrawView, sync, themes::conf::Conf, visible, ComponentAnInit
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GCardBase = {{GCard}} {
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
                    from: {all: Forward {duration: (AN_DURATION),},},
                    ease: InOutQuad,
                    apply: {
                       draw_card: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GCard {
    #[live]
    pub style: CardProp,
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
    #[live(true)]
    pub animation_spread: bool,
    // --- slots -------------------
    #[live]
    pub header: GView,
    #[live]
    pub body: GView,
    #[live]
    pub footer: GView,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[live]
    pub draw_card: DrawView,
    // --- draw  --------------------
    #[rust]
    defer_walks: DeferWalks,
    #[rust]
    pub state: CardState,
}

impl WidgetNode for GCard {
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
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_card.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_card.redraw(cx);
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

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GCard {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let state = self.state;
        let style = self.style.get(state);

        let _ = self.draw_card.begin(cx, walk, style.layout());
        let _ = SlotDrawer::new(
            [
                (live_id!(header), (&mut self.header).into()),
                (live_id!(body), (&mut self.body).into()),
                (live_id!(footer), (&mut self.footer).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);

        self.draw_card.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().card = true;

        // handle slot events
        let mut is_slot_hover = false;
        for slot in [&mut self.header, &mut self.body, &mut self.footer] {
            let slot_state = slot.state;

            slot.handle_event(cx, event, scope);
            match slot_state {
                ViewState::Hover | ViewState::Pressed => {
                    is_slot_hover = true;
                }
                _ => {
                    is_slot_hover = false;
                }
            }
        }

        if is_slot_hover {
            self.switch_state_with_animation(cx, CardState::Hover);
        } else {
            self.switch_state_with_animation(cx, CardState::Basic);
        }

        let area = self.area();
        let hit = event.hits(cx, area);
        self.handle_widget_event(cx, event, hit, area);
    }
}

impl LiveHook for GCard {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicStyle::live_props();
        self.set_apply_slot_map(
            nodes,
            index,
            [live_id!(basic), live_id!(hover)],
            [
                (CardPart::Container, &live_props),
                (CardPart::Header, &live_props),
                (CardPart::Body, &live_props),
                (CardPart::Footer, &live_props),
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

impl Component for GCard {
    type Error = Error;

    type State = CardState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.card;
        self.style = style.clone();
        self.header.style.basic = self.style.basic.header.into();
        self.header.style.hover = self.style.hover.header.into();
        self.body.style.basic = self.style.basic.body.into();
        self.body.style.hover = self.style.hover.body.into();
        self.footer.style.basic = self.style.basic.footer.into();
        self.footer.style.hover = self.style.hover.footer.into();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.state;
        let style = self.style.get(state);
        self.draw_card.merge(&style.container);
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, _area: Area) {
        animation_open_then_redraw!(self, cx, event);

        match hit {
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.style.get(self.state).container.cursor);
                self.switch_state_with_animation(cx, CardState::Hover);
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, CardState::Basic);
                hit_hover_out!(self, cx, e);
            }
            _ => {}
        };
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.header.switch_state(state.into());
        self.body.switch_state(state.into());
        self.footer.switch_state(state.into());
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
            (CardPart::Header, &mut self.header),
            (CardPart::Body, &mut self.body),
            (CardPart::Footer, &mut self.footer),
        ] {
            crossed_map.remove(&part).map(|map| {
                slot.apply_state_map.merge(map.to_state());
            });

            slot.focus_sync();
        }

        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
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
            let basic_prop = self.style.get(CardState::Basic);
            let hover_prop = self.style.get(CardState::Hover);
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
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
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
        }
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl SlotComponent<CardState> for GCard {
    type Part = CardPart;
}

impl GCard {
    active_event! {
        active_hover_in: CardEvent::HoverIn |meta: FingerHoverEvent| => CardHoverIn { meta },
        active_hover_out: CardEvent::HoverOut |meta: FingerHoverEvent| => CardHoverOut { meta }
    }
    event_option! {
        hover_in: CardEvent::HoverIn => CardHoverIn,
        hover_out: CardEvent::HoverOut => CardHoverOut
    }
    area! {
        area_header, header,
        area_body, body,
        area_footer, footer
    }
}

impl GCardRef {
    event_option_ref! {
        hover_in => CardHoverIn,
        hover_out => CardHoverOut
    }
    area_ref! {
        area_header,
        area_body,
        area_footer
    }
    getter_setter_ref! {}
}
