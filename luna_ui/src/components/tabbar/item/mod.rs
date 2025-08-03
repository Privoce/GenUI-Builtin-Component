mod prop;

use makepad_widgets::*;
pub use prop::*;

use crate::{
    components::{
        label::{GLabel, LabelBasicProp},
        lifecycle::LifeCycle,
        svg::{GSvg, SvgBasicProp, SvgState},
        traits::{BasicProp, Component, Prop, SlotComponent, SlotProp},
        view::ViewBasicProp,
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        manuel::{ACTIVE, BASIC, DISABLED, HOVER}, traits::ToFloat, ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, DeferWalks, SlotDrawer
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    themes::Conf,
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
                        draw_item: {hover: 0.0, pressed: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        active: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                        draw_item: {hover: 1.0, pressed: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_item: {focus: 1.0, pressed: 0.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GTabbarItem {
    #[live]
    pub prop: TabbarItemProp,
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
        let prop = self.prop.get(self.state);
        prop.walk()
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
        let prop = self.prop.get(self.state);

        let _ = self.draw_item.begin(cx, prop.walk(), prop.layout());
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible() {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().tabbar_item = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
        // default_handle_animation!(self, cx, event);

        // match event.hits(cx, self.area()) {
        //     Hit::FingerDown(_) => {
        //         if self.grab_key_focus {
        //             cx.set_key_focus(self.area());
        //         }
        //         if !self.active {
        //             // self.play_animation(cx, id!(hover.focus));
        //             self.animate_focus_on(cx);
        //         }
        //     }
        //     Hit::FingerHoverIn(e) => {
        //         let _ = set_cursor(cx, self.cursor.as_ref());
        //         if !self.active {
        //             self.play_animation(cx, id!(hover.on));
        //             self.active_hover_in(cx, e);
        //         }
        //     }
        //     Hit::FingerHoverOut(_) => {
        //         if !self.active {
        //             self.play_animation(cx, id!(hover.off));
        //         }
        //     }
        //     Hit::FingerUp(e) => {
        //         if e.is_over {
        //             if !self.active {
        //                 self.active(cx);
        //                 self.active_clicked(cx, e);
        //             }
        //         }
        //     }
        //     _ => (),
        // }
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
                (TabbarItemPart::Icon, &SvgBasicProp::live_props()),
                (TabbarItemPart::Text, &LabelBasicProp::live_props()),
                (TabbarItemPart::Container, &ViewBasicProp::live_props()),
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
        let prop = &cx.global::<Conf>().components.tabbar_item;
        self.prop = prop.clone();
        self.icon.prop.basic = self.prop.basic.icon;
        self.icon.prop.hover = self.prop.hover.icon;
        self.icon.prop.pressed = self.prop.active.icon;
        self.icon.prop.disabled = self.prop.disabled.icon;
        self.text.prop.basic = self.prop.basic.text;
        self.text.prop.disabled = self.prop.disabled.text;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let prop = self.prop.get(self.state);
        self.draw_item.merge(&prop.container);
        let _ = self.icon.render(cx)?;
        let _ = self.text.render(cx)?;
        Ok(())
    }



    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {}

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(self.prop.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        self.draw_item.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0,
            },
        );
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        // match state {
        //     TabbarItemState::Basic => {
        //         self.draw_item.state_basic();
        //     }
        //     TabbarItemState::Hover => {
        //         self.draw_item.state_hover();
        //     }
        //     TabbarItemState::Active => {
        //         self.draw_item.state_pressed();
        //     }
        //     TabbarItemState::Disabled => {}
        // }
        self.state = state;
        self.icon.switch_state(state.into());
        self.text.switch_state(state.into());
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
        let mut crossed_map = self.apply_slot_map.cross();
        // let mut icon_slot_map = self.apply_slot_map.iter().map(|(k, v)| {
        //     let icon_part_map = v.get(&TabbarItemPart::Icon).cloned().unwrap_or_default();

        //     (SvgState::from(*k),)
        // });
        // dbg!(&self.apply_slot_map);
        // crossed_map.remove(&TabbarItemPart::Icon).map(|map| {
        //     // let map = map.into_iter().map(|(k, v)| (k.into(), v)).collect();
        //     self.icon.apply_slot_map.merge(map);
        // });

        crossed_map.remove(&TabbarItemPart::Text).map(|map| {
            let map = map.into_iter().map(|(k, v)| (k.into(), v)).collect();
            self.text.apply_state_map.merge(map);
        });

        self.prop.sync_slot(&self.apply_slot_map);
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
            let basic_prop = self.prop.get(TabbarItemState::Basic);
            let hover_prop = self.prop.get(TabbarItemState::Hover);
            let active_prop = self.prop.get(TabbarItemState::Active);
            let (mut basic_index, mut hover_index, mut active_index) = (None, None, None);
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

            set_animation! {
                nodes: draw_container = {
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
                    }
                }
            }
        } else {
            let state = self.state;
            let prop = self.prop.get(state);
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
                _ => None,
            };
            set_animation! {
                nodes: draw_item = {
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
