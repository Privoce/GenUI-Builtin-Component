mod prop;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        label::{GLabel, LabelBasicStyle},
        lifecycle::LifeCycle,
        menu::event::{MenuItemClicked, MenuItemEvent, MenuItemHoverIn, MenuItemHoverOut},
        svg::{GSvg, SvgBasicStyle},
        traits::{BasicStyle, Component, Style, SlotComponent, SlotStyle},
        view::{GView, ViewBasicStyle},
    },
    error::Error,
    event_option, getter, hit_hover_in, hit_hover_out, lifecycle, play_animation,
    prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER},
        traits::ToFloat,
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, SlotDrawer,
        ToSlotMap, ToStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path, setter,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible, ComponentAnInit,
};
use makepad_widgets::*;
pub use prop::*;

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GMenuItemBase = {{GMenuItem}}{
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_item: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_item: <AN_DRAW_VIEW> {}
                    }
                }

                active = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_item: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_item: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GMenuItem {
    #[live]
    pub style: MenuItemProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<MenuItemState, MenuItemPart>,
    // --- draw ----------------------
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    #[live]
    pub extra: GView,
    #[live]
    pub draw_item: DrawView,
    // --- animator ----------------
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
    #[rust]
    pub state: MenuItemState,
    #[rust]
    defer_walks: DeferWalks,
    #[live]
    pub active: bool,
    #[live]
    pub value: String,
}

impl WidgetNode for GMenuItem {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for (_, child) in &self.extra.children {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for (_, child) in &self.extra.children {
            child.find_widgets(path, cached, results);
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_item.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_item.redraw(cx);
        if self.icon.visible {
            self.icon.redraw(cx);
        }
        if self.text.visible {
            self.text.redraw(cx);
        }
        if self.extra.visible {
            self.extra.redraw(cx);
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

impl Widget for GMenuItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let state = self.state;
        let style = self.style.get(state);

        let _ = self.draw_item.begin(cx, walk, style.layout());

        let _ = SlotDrawer::new(
            [
                (live_id!(icon), (&mut self.icon).into()),
                (live_id!(text), (&mut self.text).into()),
                (live_id!(extra), (&mut self.extra).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);

        self.draw_item.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().menu_item = true;

        // handle slot events
        let is_slot_hover = false;
        self.icon.handle_event(cx, event, scope);
        self.text.handle_event(cx, event, scope);
        self.extra.handle_event(cx, event, scope);
        // let super_state: MenuItemState = self.icon.state.into();

        if is_slot_hover {
            self.switch_state_with_animation(cx, MenuItemState::Hover);
        } else {
            self.switch_state_with_animation(cx, MenuItemState::Basic);
        }

        let area = self.area();
        let hit = event.hits(cx, area);
        self.handle_widget_event(cx, event, hit, area);
    }
}

impl LiveHook for GMenuItem {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicStyle::live_props();
        self.set_apply_slot_map(
            nodes,
            index,
            [live_id!(basic), live_id!(hover), live_id!(active), live_id!(disabled)],
            [
                (MenuItemPart::Container, &live_props),
                (MenuItemPart::Icon, &SvgBasicStyle::live_props()),
                (MenuItemPart::Text, &LabelBasicStyle::live_props()),
                (MenuItemPart::Extra, &live_props),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(MenuItemState::Basic, applys);
                }
                HOVER => {
                    component
                        .apply_slot_map
                        .insert(MenuItemState::Hover, applys);
                }
                ACTIVE => {
                    component
                        .apply_slot_map
                        .insert(MenuItemState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(MenuItemState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GMenuItem {
    type Error = Error;

    type State = MenuItemState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.menu_item;
        self.style = style.clone();
        self.icon.style.basic = self.style.basic.icon;
        self.icon.style.hover = self.style.hover.icon;
        self.icon.style.pressed = self.style.active.icon;
        self.icon.style.disabled = self.style.disabled.icon;
        self.text.style.basic = self.style.basic.text;
        self.text.style.disabled = self.style.disabled.text;
        self.extra.style.basic = self.style.basic.extra;
        self.extra.style.hover = self.style.hover.extra;
        self.extra.style.pressed = self.style.active.extra;
        self.extra.style.disabled = self.style.disabled.extra;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let state = if self.disabled {
            MenuItemState::Disabled
        } else {
            if self.active {
                MenuItemState::Active
            } else {
                MenuItemState::Basic
            }
        };
        self.switch_state(state);
        let style = self.style.get(self.state);
        self.draw_item.merge(&style.container);
        let _ = self.icon.render(cx)?;
        let _ = self.text.render(cx)?;
        let _ = self.extra.render(cx)?;
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);
        if !self.active {
            match hit {
                Hit::FingerDown(_) => {
                    if self.grab_key_focus {
                        cx.set_key_focus(area);
                    }
                }
                Hit::FingerHoverIn(e) => {
                    cx.set_cursor(self.style.get(self.state).container.cursor);
                    self.switch_state_with_animation(cx, MenuItemState::Hover);
                    hit_hover_in!(self, cx, e);
                }
                Hit::FingerHoverOut(e) => {
                    self.switch_state_with_animation(cx, MenuItemState::Basic);
                    hit_hover_out!(self, cx, e);
                }
                Hit::FingerUp(e) => {
                    if e.is_over {
                        if e.has_hovers() {
                            self.active = true;
                            self.switch_state_with_animation(cx, MenuItemState::Active);
                            self.play_animation(cx, id!(hover.active));
                        } else {
                            self.switch_state_with_animation(cx, MenuItemState::Basic);
                            self.play_animation(cx, id!(hover.off));
                        }
                        self.active_clicked(cx, Some(e));
                    } else {
                        self.switch_state_with_animation(cx, MenuItemState::Basic);
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, MenuItemState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.icon.switch_state(state.into());
        self.text.switch_state(state.into());
        self.extra.switch_state(state.into());
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
        crossed_map.remove(&MenuItemPart::Icon).map(|map| {
            self.icon.apply_slot_map.merge_slot(map.to_slot());
            self.icon.focus_sync();
        });
        crossed_map.remove(&MenuItemPart::Text).map(|map| {
            self.text.apply_state_map.merge(map.to_state());
            self.text.focus_sync();
        });
        crossed_map.remove(&MenuItemPart::Extra).map(|map| {
            self.extra.apply_state_map.merge(map.to_state());
            self.extra.focus_sync();
        });

        // sync state if is not Basic
        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().menu_item;

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
            let basic_prop = self.style.get(MenuItemState::Basic);
            let hover_prop = self.style.get(MenuItemState::Hover);
            let active_prop = self.style.get(MenuItemState::Active);
            let disabled_prop = self.style.get(MenuItemState::Disabled);
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
                nodes: draw_item = {
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
            let style = self.style.get(state);
            let index = match state {
                MenuItemState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                MenuItemState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                MenuItemState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(active).as_instance(),
                    ],
                ),
                MenuItemState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_item = {
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

impl SlotComponent<MenuItemState> for GMenuItem {
    type Part = MenuItemPart;
}

impl GMenuItem {
    active_event! {
        active_hover_in: MenuItemEvent::HoverIn |meta: FingerHoverEvent| => MenuItemHoverIn { meta },
        active_hover_out: MenuItemEvent::HoverOut |meta: FingerHoverEvent| => MenuItemHoverOut { meta }
    }
    event_option! {
        hover_in: MenuItemEvent::HoverIn => MenuItemHoverIn,
        hover_out: MenuItemEvent::HoverOut => MenuItemHoverOut,
        clicked: MenuItemEvent::Clicked => MenuItemClicked
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    MenuItemEvent::Clicked(MenuItemClicked {
                        active: self.active,
                        value: self.value.to_string(),
                        meta,
                    }),
                );
            });
        }
    }
    pub fn toggle(&mut self, cx: &mut Cx, active: bool, init: bool) -> () {
        self.active = active;
        let (state, hover_id) = match (active, init) {
            (true, false) => (MenuItemState::Active, Some(id!(hover.active))),
            (true, true) => (MenuItemState::Active, None),
            (false, true) => (MenuItemState::Basic, None),
            (false, false) => (MenuItemState::Basic, Some(id!(hover.off))),
        };
        self.switch_state(state);
        if let Some(hover_id) = hover_id {
            self.play_animation(cx, hover_id);
        }
        self.active_clicked(cx, None);
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
        GMenuItem {
            get_active(bool) {|c| {c.active}}
        }
    }
    setter! {
        GMenuItem {
            set_active(active: bool) {|c, cx| {c.active = active; c.redraw(cx); Ok(())}}
        }
    }
}
