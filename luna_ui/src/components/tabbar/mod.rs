mod event;
mod item;
mod prop;
mod register;
// pub mod virt;
mod schema;

use std::cell::RefCell;

use crate::{
    animation_open_then_redraw,
    components::{BasicProp, Component, DrawState, LifeCycle, Prop},
    error::Error,
    event_option, event_option_ref, impl_view_trait_live_hook, impl_view_trait_widget_node,
    lifecycle, play_animation,
    prop::{
        manuel::{BASIC, DISABLED},
        traits::ToFloat,
        ApplyStateMap, DeferWalks,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    ComponentAnInit,
};
pub use event::*;
pub use item::*;
pub use prop::*;
pub use register::register as tabbar_register;
pub use schema::*;

use makepad_widgets::*;

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GTabbarBase = {{GTabbar}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_tabbar: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_tabbar: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GTabbar {
    #[live]
    pub prop: TabbarProp,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    #[live]
    pub draw_tabbar: DrawView,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub area: Area,
    #[rust]
    pub defer_walks: DeferWalks,
    #[rust]
    pub children: SmallVec<[(LiveId, WidgetRef); 2]>,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    #[rust]
    find_cache: RefCell<SmallVec<[(u64, WidgetSet); 3]>>,
    // --- lifecycle --------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub apply_state_map: ApplyStateMap<TabbarState>,
    #[rust]
    pub state: TabbarState,
    #[live]
    pub active: Option<String>,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    #[live(true)]
    pub event_open: bool,
}

impl_view_trait_widget_node!(GTabbar, draw_tabbar);

impl Widget for GTabbar {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let prop = self.prop.get(self.state);
        // the beginning state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                self.draw_state.end();
                self.set_scope_path(&scope.path);
                return DrawStep::done();
            }
            self.defer_walks.clear();

            let layout = prop.layout();

            if prop.background_visible {
                self.draw_tabbar.begin(cx, walk, layout);
            } else {
                cx.begin_turtle(walk, layout);
            }
        }

        while let Some(DrawState::Drawing(step, resume)) = self.draw_state.get() {
            if step < self.children.len() {
                if let Some((id, child)) = self.children.get_mut(step) {
                    if child.visible() {
                        let walk = child.walk(cx);
                        child.set_disabled(cx, self.disabled);

                        if resume {
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                        } else if let Some(fw) = cx.defer_walk(walk) {
                            self.defer_walks.push((*id, fw));
                        } else {
                            self.draw_state.set(DrawState::Drawing(step, true));
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                        }
                    }
                }
                self.draw_state.set(DrawState::Drawing(step + 1, false));
            } else {
                self.draw_state.set(DrawState::DeferWalk(0));
            }
        }

        while let Some(DrawState::DeferWalk(step)) = self.draw_state.get() {
            if step < self.defer_walks.len() {
                let (id, dw) = &mut self.defer_walks[step];
                if let Some((id, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                    let walk = dw.resolve(cx);
                    child.set_disabled(cx, self.disabled);
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                if prop.background_visible {
                    self.draw_tabbar.end(cx);
                    self.area = self.draw_tabbar.area();
                } else {
                    cx.end_turtle_with_area(&mut self.area);
                };

                self.draw_state.end();
            }
        }
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible && event.requires_visibility() {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().tabbar = true;

        animation_open_then_redraw!(self, cx, event);

        if let Event::Actions(actions) = event {
            let mut active_index = None;
            let mut active_value = None;
            let mut active_event = None;
            for (index, (_id, child)) in self.children.iter().enumerate() {
                let _ = child.as_gtabbar_item().borrow().map(|radio| {
                    if let Some(param) = radio.clicked(&actions) {
                        if param.active && active_index.is_none() && active_event.is_none() {
                            active_value.replace(param.value);
                            active_index = Some(index);
                            active_event = param.meta;
                        }
                    }
                });
                if active_index.is_some() {
                    break;
                }
            }
            if active_index.is_some() && active_value.is_some() {
                let _ = self.toggle(cx, active_value.clone(), false);
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    TabbarEvent::Changed(TabbarChanged {
                        meta: active_event,
                        value: active_value,
                        index: active_index.unwrap() as i32,
                    }),
                );
            }
        }
    }
}

impl LiveHook for GTabbar {
    pure_after_apply!();
    impl_view_trait_live_hook!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if apply.from.is_update_from_doc() {
            //livecoding
            // update/delete children list
            for (idx, id) in self.live_update_order.iter().enumerate() {
                // lets remove this id from the childlist
                if let Some(pos) = self.children.iter().position(|(i, _v)| *i == *id) {
                    // alright so we have the position its in now, and the position it should be in
                    self.children.swap(idx, pos);
                }
            }
            // if we had more truncate
            self.children.truncate(self.live_update_order.len());
        }
        self.set_apply_state_map(
            nodes,
            index,
            &TabbarBasicProp::live_props(),
            [live_id!(basic), live_id!(disabled)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(TabbarState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(TabbarState::Disabled, applys);
                }
                _ => {}
            },
        );
        if let Some(active) = self.active.as_ref() {
            self.set_active(cx, Some(active.to_string()));
        } else {
            self.find_active();
        }
    }
}

impl Component for GTabbar {
    type Error = Error;

    type State = TabbarState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.tabbar;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let prop = self.prop.get(self.state);
        self.draw_tabbar.merge(&(*prop).into());
        if self.disabled {
            self.switch_state(TabbarState::Disabled);
        }
        Ok(())
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        self.prop.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().view;

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
            let basic_prop = self.prop.get(TabbarState::Basic);
            let disabled_prop = self.prop.get(TabbarState::Disabled);
            let (mut basic_index, mut disabled_index) = (None, None);
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
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_tabbar = {
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
                    disabled_index => {
                        background_color => disabled_prop.background_color,
                        border_color => disabled_prop.border_color,
                        border_radius => disabled_prop.border_radius,
                        border_width => (disabled_prop.border_width as f64),
                        shadow_color => disabled_prop.shadow_color,
                        spread_radius => (disabled_prop.spread_radius as f64),
                        blur_radius => (disabled_prop.blur_radius as f64),
                        shadow_offset => disabled_prop.shadow_offset,
                        background_visible => disabled_prop.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let prop = self.prop.get(state);
            let index = match state {
                TabbarState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                TabbarState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_tabbar = {
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

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl GTabbar {
    pub fn find_active(&mut self) -> () {
        if self.active.is_some() {
            return;
        }

        let mut active_value = None;
        self.children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gtabbar_item().borrow_mut() {
                    // 判断tabbar的value是否为空，如果是就按照iter的index设置
                    if child.value.is_empty() {
                        child.value = index.to_string();
                    }
                    if child.active && active_value.is_none() {
                        active_value.replace(child.value.to_string());
                    } else if child.active && active_value.is_some() {
                        panic!(
                            "GTabbar can only have one active GTabbarItem, but found multiple: {}",
                            child.value
                        );
                    }
                } else {
                    panic!("GTabbar only allows GTabbarItem as child!");
                }
            });

        if let Some(active_value) = active_value {
            self.active.replace(active_value);
        }
    }
    /// if active is not set(None) in the group: find the active radio in the group
    /// else: set the active radio depending on the value of `active`
    pub fn set_active(&mut self, cx: &mut Cx, active_value: Option<String>) -> () {
        self.toggle(cx, active_value, true);
    }
    pub fn toggle(&mut self, cx: &mut Cx, active_value: Option<String>, init: bool) -> () {
        self.active = active_value;

        self.children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gtabbar_item().borrow_mut() {
                    if child.value.is_empty() {
                        child.value = index.to_string();
                    }
                    let active = child.value.eq(self.active.as_ref().unwrap());
                    child.toggle(cx, active, init);
                } else {
                    panic!("GTabbar only allows GTabbarItem as child!")
                }
            });
    }
    pub fn set_active_index(&mut self, cx: &mut Cx, index: usize) -> () {
        let mut active_value = None;
        self.children
            .iter()
            .enumerate()
            .for_each(|(i, (_id, child))| {
                if let Some(mut child) = child.as_gtabbar_item().borrow_mut() {
                    let active = i == index;
                    if active {
                        active_value.replace(child.value.to_string());
                    }
                    child.toggle(cx, active, false);
                } else {
                    panic!("GTabbar only allows GTabbarItem as child!")
                }
            });

        if let Some(active_value) = active_value {
            self.active.replace(active_value);
        }
    }
}

impl GTabbar {
    event_option! {
        changed: TabbarEvent::Changed => TabbarChanged
    }
}

impl GTabbarRef {
    event_option_ref! {
        changed => TabbarChanged
    }
}
