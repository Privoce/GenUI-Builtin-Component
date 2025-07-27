mod prop;

use std::cell::RefCell;

pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, PopupComponent, Prop},
        view::{needs_draw_list, DrawState},
    },
    error::Error,
    lifecycle,
    prop::{manuel::BASIC, ApplyStateMap, CloseMode, DeferWalks, PopupMode},
    pure_after_apply, set_index, set_scope_path,
    shader::{draw_popup::DrawPopup, },
    themes::Conf,
};

live_design! {
    link genui_basic;
    GPopupBase = {{GPopup}}{}
}

#[derive(Live, LiveRegister)]
pub struct GPopup {
    #[live]
    pub prop: PopupProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<PopupState>,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[rust]
    pub sync: bool,
    // --- popup ---------------------
    #[live]
    pub close_mode: CloseMode,
    // --- draw ----------------------
    #[live]
    pub draw_popup: DrawPopup,
    /// draw list is necessary!!!
    /// because we need to draw the popup on top of everything
    /// although the name of DrawList2d may let you think it's only for 2d list drawing
    /// actually it's for all the drawing that needs to be on top of everything!!!
    #[live]
    draw_list: DrawList2d,
    // --- from view -------------------
    #[rust]
    pub area: Area,
    #[live]
    pub scroll: DVec2,
    #[live]
    pub scroll_bars: Option<LivePtr>,
    #[rust]
    scroll_bars_obj: Option<Box<ScrollBars>>,
    #[rust]
    pub children: SmallVec<[(LiveId, WidgetRef); 2]>,
    #[rust]
    defer_walks: DeferWalks,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    #[rust]
    find_cache: RefCell<SmallVec<[(u64, WidgetSet); 3]>>,
}

impl LiveHook for GPopup {
    // pure_after_apply!();
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            //self.draw_order.clear();
            self.live_update_order.clear();
            self.find_cache.get_mut().clear();
        }
    }

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

        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }

        self.set_apply_state_map(
            nodes,
            index,
            &PopupBasicProp::live_props(),
            [live_id!(basic)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(PopupState::Basic, applys);
                }
                _ => {}
            },
        );
    }

    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::Animate | ApplyFrom::Over => {
                let node_id = nodes[index].id;
                if let Some((_, component)) =
                    self.children.iter_mut().find(|(id, _)| *id == node_id)
                {
                    component.apply(cx, apply, index, nodes)
                } else {
                    nodes.skip_node(index)
                }
            }
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if nodes[index].is_instance_prop() {
                    if apply.from.is_update_from_doc() {
                        //livecoding
                        self.live_update_order.push(id);
                    }
                    //self.draw_order.push(id);
                    if let Some((_, node)) = self.children.iter_mut().find(|(id2, _)| *id2 == id) {
                        node.apply(cx, apply, index, nodes)
                    } else {
                        self.children.push((id, WidgetRef::new(cx)));
                        self.children
                            .last_mut()
                            .unwrap()
                            .1
                            .apply(cx, apply, index, nodes)
                    }
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                    nodes.skip_node(index)
                }
            }
            _ => nodes.skip_node(index),
        }
    }
}

impl PopupComponent for GPopup {
    type Error = Error;
    type State = PopupState;
    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.popup;
        self.prop = prop.clone();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let prop = self.prop.get(self.current_state());
        self.draw_popup.merge(&prop.into());
        Ok(())
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        self.prop.sync(&self.apply_state_map);
    }

    fn current_state(&self) -> Self::State {
        PopupState::Basic
    }

    fn begin(&mut self, cx: &mut Cx2d) -> () {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        let prop = self.prop.get(self.current_state());
        self.draw_popup.begin(cx, prop.walk(), prop.layout());
    }

    fn end(&mut self, cx: &mut Cx2d, scope: &mut Scope, shift_area: Area, shift: DVec2) -> () {
        self.draw_popup.end(cx);
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
        self.set_scope_path(&scope.path);
    }

    fn draw_popup(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Option<crate::prop::Position>,
        angle_offset: f32,
        redraw: &mut bool,
    ) -> () {
        let _ = position.map(|position| {
            self.draw_popup.position = position;
        });
        self.draw_popup.angle_offset = angle_offset;
        // draw the popup ------------------------------------------------------------------------

        // ---------------------------------------------------------------------------------------
        if *redraw {
            self.draw_popup.redraw(cx);
            *redraw = !*redraw;
        }
    }
    fn redraw(&mut self, cx: &mut Cx) -> () {
        if self.visible {
            let _ = self.render(cx);
            self.draw_popup.redraw(cx);
            self.draw_list.redraw(cx);
            for (_, child) in &mut self.children {
                if child.visible() {
                    child.redraw(cx);
                }
            }
        }
    }
    set_index!();
    lifecycle!();
    set_scope_path!();
}

impl GPopup {
    pub fn draw_container(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        let prop = self.prop.get(self.current_state());

        // the beginning state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                self.draw_state.end();
                return;
            }
            self.defer_walks.clear();

            let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                scroll_bars.begin_nav_area(cx);
                scroll_bars.get_scroll_pos()
            } else {
                self.scroll
            };

            let layout = prop.layout().with_scroll(scroll);
            let walk = prop.walk();
            if prop.background_visible {
                self.draw_popup.begin(cx, walk, layout);
            } else {
                cx.begin_turtle(walk, layout);
            }
        }

        while let Some(DrawState::Drawing(step, resume)) = self.draw_state.get() {
            if step < self.children.len() {
                if let Some((id, child)) = self.children.get_mut(step) {
                    if child.visible() {
                        let walk = child.walk(cx);
                        // child.set_disabled(cx, self.disabled);
                        if resume {
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk));
                        } else if let Some(fw) = cx.defer_walk(walk) {
                            self.defer_walks.push((*id, fw));
                        } else {
                            self.draw_state.set(DrawState::Drawing(step, true));
                            scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk));
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
                    // child.set_disabled(cx, self.disabled);
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk));
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                };

                if prop.background_visible {
                    self.draw_popup.end(cx);
                    self.area = self.draw_popup.area();
                } else {
                    cx.end_turtle_with_area(&mut self.area);
                };

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.set_area(self.area);
                    scroll_bars.end_nav_area(cx);
                };

                self.draw_state.end();
            }
        }
    }
}
