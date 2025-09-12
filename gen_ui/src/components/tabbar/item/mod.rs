mod prop;

use makepad_widgets::*;
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        label::{GLabel, LabelBasicStyle},
        lifecycle::LifeCycle,
        svg::{GSvg, SvgBasicStyle},
        tabbar::{TabbarItemClicked, TabbarItemEvent, TabbarItemHoverIn, TabbarItemHoverOut},
        traits::{BasicStyle, Component, Style, SlotComponent, SlotStyle},
        view::ViewBasicStyle,
    },
    error::Error,
    event_option, hit_hover_in, hit_hover_out, lifecycle, play_animation,
    prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER},
        traits::ToFloat,
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, SlotDrawer,
        ToSlotMap, ToStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GTabbarItemBase = {{GTabbarItem}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    apply: {
                        draw_item: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        active: Forward {duration: (AN_DURATION)},
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
pub struct GTabbarItem {
    #[live]
    pub style: TabbarItemProp,
    // --- draw ----------------------
    #[live]
    pub draw_item: DrawView,
    // --- slots ----------------------
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    // --- other ----------------------
    #[live(false)]
    pub disabled: bool,
    #[live(false)]
    pub grab_key_focus: bool,
    #[rust]
    pub apply_slot_map: ApplySlotMap<TabbarItemState, TabbarItemPart>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    animator: Animator,
    #[live]
    pub active: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    defer_walks: DeferWalks,
    #[rust]
    pub state: TabbarItemState,
    #[live]
    pub value: String,
}

impl WidgetNode for GTabbarItem {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        let icon_ref = self.icon.uid_to_widget(uid);
        let text_ref = self.text.uid_to_widget(uid);
        match (icon_ref.is_empty(), text_ref.is_empty()) {
            (true, true) => WidgetRef::empty(),
            (true, false) => icon_ref,
            (false, true) => text_ref,
            (false, false) => unreachable!("GTabbarItem can not both have slot uid_to_widget"),
        }
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
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
        if self.icon.visible {
            self.icon.redraw(cx);
        }
        if self.text.visible {
            self.text.redraw(cx);
        }
        self.draw_item.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    visible!();
}

impl Widget for GTabbarItem {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible() {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        let _ = self.draw_item.begin(cx, walk, style.layout());
        let _ = SlotDrawer::new(
            [
                (live_id!(icon), (&mut self.icon).into()),
                (live_id!(text), (&mut self.text).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);

        let _ = self.draw_item.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.handle_event_mixin(
            cx,
            event,
            scope,
            Option::<fn(&mut Cx, TabbarItemClicked)>::None,
        );
    }
}

impl LiveHook for GTabbarItem {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            nodes,
            index,
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(active),
                live_id!(disabled),
            ],
            [
                (TabbarItemPart::Icon, &SvgBasicStyle::live_props()),
                (TabbarItemPart::Text, &LabelBasicStyle::live_props()),
                (TabbarItemPart::Container, &ViewBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_slot_map
                        .insert(TabbarItemState::Basic, applys);
                }
                HOVER => {
                    component
                        .apply_slot_map
                        .insert(TabbarItemState::Hover, applys);
                }
                ACTIVE => {
                    component
                        .apply_slot_map
                        .insert(TabbarItemState::Active, applys);
                }
                DISABLED => {
                    component
                        .apply_slot_map
                        .insert(TabbarItemState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<TabbarItemState> for GTabbarItem {
    type Part = TabbarItemPart;
}

impl Component for GTabbarItem {
    type Error = Error;

    type State = TabbarItemState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.tabbar_item;
        self.style = style.clone();
        self.icon.style.basic = self.style.basic.icon;
        self.icon.style.hover = self.style.hover.icon;
        self.icon.style.pressed = self.style.active.icon;
        self.icon.style.disabled = self.style.disabled.icon;
        self.text.style.basic = self.style.basic.text;
        self.text.style.disabled = self.style.disabled.text;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let state = if self.disabled {
            TabbarItemState::Disabled
        } else {
            if self.active {
                TabbarItemState::Active
            } else {
                TabbarItemState::Basic
            }
        };
        self.switch_state(state);
        let style = self.style.get(self.state);
        self.draw_item.merge(&style.container);
        let _ = self.icon.render(cx)?;
        let _ = self.text.render(cx)?;
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        self.handle_widget_event_mixin(
            cx,
            event,
            hit,
            area,
            Option::<fn(&mut Cx, TabbarItemClicked)>::None,
        );
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, TabbarItemState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
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
        self.icon.switch_state_with_animation(cx, state.into());
        self.text.switch_state_with_animation(cx, state.into());
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();

        crossed_map.remove(&TabbarItemPart::Icon).map(|map| {
            self.icon.apply_slot_map.merge_slot(map.to_slot());
            self.icon.focus_sync();
        });

        crossed_map.remove(&TabbarItemPart::Text).map(|map| {
            self.text.apply_state_map.merge(map.to_state());
            self.text.focus_sync();
        });

        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().tabbar_item;
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
            let basic_prop = self.style.get(TabbarItemState::Basic);
            let hover_prop = self.style.get(TabbarItemState::Hover);
            let active_prop = self.style.get(TabbarItemState::Active);
            let disabled_prop = self.style.get(TabbarItemState::Disabled);
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
                        border_color => basic_prop.container.border_color,
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
                TabbarItemState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                TabbarItemState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                TabbarItemState::Active => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(active).as_instance(),
                    ],
                ),
                TabbarItemState::Disabled => nodes.child_by_path(
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

impl GTabbarItem {
    active_event! {
        active_hover_in: TabbarItemEvent::HoverIn |meta: FingerHoverEvent| => TabbarItemHoverIn { meta },
        active_hover_out: TabbarItemEvent::HoverOut |meta: FingerHoverEvent| => TabbarItemHoverOut { meta }
    }
    event_option! {
        hover_in: TabbarItemEvent::HoverIn => TabbarItemHoverIn,
        hover_out: TabbarItemEvent::HoverOut => TabbarItemHoverOut,
        clicked: TabbarItemEvent::Clicked => TabbarItemClicked
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    TabbarItemEvent::Clicked(TabbarItemClicked {
                        active: self.active,
                        value: self.value.to_string(),
                        meta,
                    }),
                );
            });
        }
    }
    pub fn toggle(&mut self, cx: &mut Cx, active: bool, init: bool) -> () {
        self.toggle_mixin(cx, active, init, false);
    }
    pub fn toggle_mixin(&mut self, cx: &mut Cx, active: bool, init: bool, mixin: bool) -> () {
        self.active = active;
        let (state, hover_id) = match (active, init) {
            (true, false) => (TabbarItemState::Active, Some(id!(hover.active))),
            (true, true) => (TabbarItemState::Active, None),
            (false, true) => (TabbarItemState::Basic, None),
            (false, false) => (TabbarItemState::Basic, Some(id!(hover.off))),
        };
        self.switch_state_with_animation(cx, state);
        if let Some(hover_id) = hover_id {
            self.play_animation(cx, hover_id);
        }
        if !mixin {
            self.active_clicked(cx, None);
        }
    }
    pub fn handle_event_mixin<F>(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        mixin: Option<F>,
    ) -> bool
    where
        F: FnOnce(&mut Cx, TabbarItemClicked) -> (),
    {
        if !self.visible() {
            return false;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().tabbar_item = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            return self.handle_widget_event_mixin(cx, event, hit, area, mixin);
        }
        return false;
    }
    fn handle_widget_event_mixin<F>(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        hit: Hit,
        area: Area,
        mixin: Option<F>,
    ) -> bool
    where
        F: FnOnce(&mut Cx, TabbarItemClicked) -> (),
    {
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
                    self.switch_state_with_animation(cx, TabbarItemState::Hover);
                    hit_hover_in!(self, cx, e);
                }
                Hit::FingerHoverOut(e) => {
                    self.switch_state_with_animation(cx, TabbarItemState::Basic);
                    hit_hover_out!(self, cx, e);
                }
                Hit::FingerUp(e) => {
                    if e.is_over {
                        if e.has_hovers() {
                            self.active = true;
                            self.switch_state_with_animation(cx, TabbarItemState::Active);
                            self.play_animation(cx, id!(hover.active));
                        } else {
                            self.switch_state_with_animation(cx, TabbarItemState::Basic);
                            self.play_animation(cx, id!(hover.off));
                        }
                        if let Some(mixin) = mixin {
                            mixin(
                                cx,
                                TabbarItemClicked {
                                    active: self.active,
                                    value: self.value.to_string(),
                                    meta: Some(e),
                                },
                            );
                            return true;
                        } else {
                            self.active_clicked(cx, Some(e));
                        }
                    } else {
                        self.switch_state_with_animation(cx, TabbarItemState::Basic);
                    }
                }
                _ => {}
            }
        }
        return false;
    }
}
