mod prop;

pub use prop::*;

use makepad_widgets::*;

use crate::{
    area,
    components::{
        lifecycle::LifeCycle,
        menu::event::{SubMenuChanged, SubMenuEvent},
        traits::{BasicStyle, Component, Style, SlotComponent, SlotStyle},
        view::{GView, ViewBasicStyle},
    },
    error::Error,
    event_option, getter, lifecycle, play_animation,
    prop::{
        manuel::{ACTIVE, BASIC, DISABLED},
        traits::ToFloat,
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ToStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path, setter,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GSubMenuBase = {{GSubMenu}}{
        animator: {
            active = {
                default: off
                off = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    apply: {
                        draw_sub_menu: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
                on = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    apply: {
                        draw_sub_menu: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
                    }
                }
                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    ease: InOutQuad,
                    apply: {
                        draw_sub_menu: <AN_DRAW_VIEW> {},
                        fold: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSubMenu {
    #[live]
    pub style: SubMenuProp,
    #[live]
    pub header: GView,
    #[live]
    pub body: GView,
    #[live]
    pub draw_sub_menu: DrawView,
    #[live]
    pub active: bool,
    #[live]
    pub value: String,
    #[live]
    pub fold: f64,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
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
    pub state: SubMenuState,
    #[rust]
    pub draw_state: DrawStateWrap<DrawSubMenuState>,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<SubMenuState, SubMenuPart>,
}

impl Widget for GSubMenu {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get_mut(self.state);
        self.fold = self.active.to_f64();
        let body_walk = self.body.walk(cx);
        let header_walk = self.header.walk(cx);

        self.draw_sub_menu.begin(cx, walk, style.layout());

        if self.draw_state.begin(cx, DrawSubMenuState::DrawHeader) {
            if self.header.visible {
                let _ = self.header.draw_walk(cx, scope, header_walk);
            }
            self.draw_state.set(DrawSubMenuState::DrawBody);
        }

        if let Some(DrawSubMenuState::DrawBody) = self.draw_state.get() {
            if self.fold == 1.0 {
                self.animator_play(cx, id!(active.on));
                let _ = self.body.draw_walk(cx, scope, body_walk);
            } else {
                self.animator_play(cx, id!(active.off));
            }
        }
        self.draw_sub_menu.end(cx);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }
        self.set_animation(cx);
        cx.global::<ComponentAnInit>().sub_menu = true;
        let hit = event.hits(cx, sweep_area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, sweep_area);
            if self.active {
                self.body.handle_event(cx, event, scope);
            }
        }
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }
        self.set_animation(cx);
        cx.global::<ComponentAnInit>().sub_menu = true;
        let area = self.area_header();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
            if self.active {
                self.body.handle_event(cx, event, scope);
            }
        }
    }
}

impl WidgetNode for GSubMenu {
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
        let style = self.style.get(self.state);
        style.container.walk()
    }

    fn area(&self) -> Area {
        if self.active {
            self.draw_sub_menu.area
        } else {
            self.area_header()
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_sub_menu.redraw(cx);
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

impl LiveHook for GSubMenu {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicStyle::live_props();
        self.set_apply_slot_map(
            nodes,
            index,
            [live_id!(basic), live_id!(active), live_id!(disabled)],
            [
                (SubMenuPart::Container, &live_props),
                (SubMenuPart::Header, &live_props),
                (SubMenuPart::Body, &live_props),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(SubMenuState::Basic, applys);
                }
                ACTIVE => {
                    component
                        .apply_slot_map
                        .insert(SubMenuState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(SubMenuState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<SubMenuState> for GSubMenu {
    type Part = SubMenuPart;
}

impl Component for GSubMenu {
    type Error = Error;

    type State = SubMenuState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.sub_menu;
        self.style = style.clone();
        self.header.style.basic = self.style.basic.header;
        self.header.style.pressed = self.style.active.header;
        self.header.style.disabled = self.style.disabled.header;
        self.body.style.basic = self.style.basic.body;
        self.body.style.pressed = self.style.active.body;
        self.body.style.disabled = self.style.disabled.body;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(SubMenuState::Disabled);
        } else {
            if self.active {
                self.switch_state(SubMenuState::Active);
            } else {
                self.switch_state(SubMenuState::Basic);
            }
        }
        let state = self.state;
        let style = self.style.get(state);
        self.draw_sub_menu.merge(&style.container);
        let _ = self.header.render(cx)?;
        let _ = self.body.render(cx)?;
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, SubMenuState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }
    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        if !self.animation_open && self.animation_counter {
            if self.animator_handle_event(cx, event).must_redraw() {
                if self.animator.is_track_animating(cx, id!(active)) {
                    self.area().redraw(cx);
                    self.animation_counter = !self.animation_counter;
                }
            }
        }

        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(area);
                }
            }
            Hit::FingerHoverIn(_meta) => {
                cx.set_cursor(self.style.get(self.state).header.cursor);
                // self.switch_state_with_animation(cx, SubMenuState::Hover);
                // self.active_hover_in(cx, meta);
            }
            Hit::FingerHoverOut(_meta) => {
                // self.switch_state_with_animation(cx, SubMenuState::Basic);
                // self.active_hover_out(cx, meta);
            }
            Hit::FingerUp(meta) => {
                self.active = !self.active;
                self.fold = self.active.to_f32() as f64;
                if self.active {
                    self.switch_state_with_animation(cx, SubMenuState::Active);
                    self.animator_play(cx, id!(active.on));
                } else {
                    self.switch_state_with_animation(cx, SubMenuState::Basic);
                    self.animator_play(cx, id!(active.off));
                }
                self.active_changed(cx, Some(meta));
                self.animation_counter = true;
            }
            _ => {}
        }
    }

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
        self.redraw(cx);
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();
        for (part, slot) in [
            (SubMenuPart::Header, &mut self.header),
            (SubMenuPart::Body, &mut self.body),
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
        let init_global = cx.global::<ComponentAnInit>().sub_menu;
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
            let basic_prop = self.style.get(SubMenuState::Basic);
            let active_prop = self.style.get(SubMenuState::Active);
            let disabled_prop = self.style.get(SubMenuState::Disabled);
            let (mut basic_index, mut active_index, mut disabled_index) = (None, None, None);
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
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_sub_menu = {
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
            let style = self.style.get(state);
            let index = match state {
                SubMenuState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                SubMenuState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                SubMenuState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(active).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_sub_menu = {
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

impl GSubMenu {
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    SubMenuEvent::Changed(SubMenuChanged {
                        meta,
                        active: self.active,
                        value: self.value.to_string(),
                    }),
                );
            });
        }
    }
    event_option! {
        changed: SubMenuEvent::Changed => SubMenuChanged
    }
    area! {
        area_header, header,
        area_body, body
    }
    pub fn generate_value(&mut self, index_chain: &Vec<usize>) {
        if self.value.is_empty() {
            self.value = index_chain
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("_")
        }
    }
    getter! {
        GSubMenu {
            get_active(bool) {|c| {c.active}}
        }
    }
    setter! {
        GSubMenu {
            set_active(active: bool) {|c, cx| {c.active = active; c.redraw(cx); Ok(())}}
        }
    }
}

#[derive(Clone, Copy)]
pub enum DrawSubMenuState {
    DrawHeader,
    DrawBody,
}
