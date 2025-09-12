use makepad_widgets::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        tabbar::{
            item::{GTabbarItem, GTabbarItemRef, GTabbarItemWidgetRefExt, TabbarItemProp},
            TabbarBasicStyle, TabbarItemData, TabbarProp, TabbarState,
        },
        traits::{BasicStyle, Component, Style},
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        manuel::{BASIC, DISABLED},
        traits::ToFloat,
        ApplyStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GVTabbarBase = {{GVTabbar}} {
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
pub struct GVTabbar {
    #[live]
    pub style: TabbarProp,
    #[live]
    pub active: Option<String>,
    #[rust]
    pub children: Vec<GTabbarItemRef>,
    #[live]
    pub items: Vec<TabbarItemData>,
    #[live]
    pub item: Option<LivePtr>,
    #[live]
    pub draw_tabbar: DrawView,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<TabbarState>,
    #[rust]
    pub state: TabbarState,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    // --- animation --------------
    #[animator]
    animator: Animator,
    #[live(false)]
    pub animation_open: bool,
    #[live(true)]
    pub animation_spread: bool,
    #[live]
    pub disabled: bool,
    #[live(true)]
    pub visible: bool,
}

impl WidgetNode for GVTabbar {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        prop.walk()
    }

    fn area(&self) -> Area {
        self.draw_tabbar.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
    }
}

impl Widget for GVTabbar {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_tabbar.begin(cx, prop.walk(), prop.layout());
        self.children.clear();
        for item in self.items.iter() {
            self.children.push(WidgetRef::new_from_ptr(cx, self.item).as_gtabbar_item());
            self.children.last_mut().map(|child|{
                child.borrow_mut().map(|mut child| {
                    child.prop = item.prop.clone();
                    child.value = item.value.to_string();
                    // child.text = item.text.to_string();
                    child.icon.src = item.icon.src.clone();
                });
            });
        }

        self.draw_tabbar.end(cx);
        DrawStep::done()
    }
}

impl LiveHook for GVTabbar {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &TabbarBasicStyle::live_props(),
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
    }
}

impl Component for GVTabbar {
    type Error = Error;

    type State = TabbarState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.tabbar;
        self.style = prop.clone();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(TabbarState::Disabled);
        }
        let style = self.style.get(self.state);
        self.draw_tabbar.merge(&(*prop).into());
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
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
        self.redraw(cx);
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
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
            let basic_prop = self.style.get(TabbarState::Basic);
            let disabled_prop = self.style.get(TabbarState::Disabled);
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
                nodes: draw_view = {
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
            let style = self.style.get(state);
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
                nodes: draw_view = {
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
