mod container;
mod prop;
 mod register;

pub use prop::*;
pub use container::*;
pub use register::register as popup_register;

use makepad_widgets::*;
use std::cell::RefCell;

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, PopupComponent, Style},
        view::DrawState,
    },
    error::Error,
    lifecycle,
    prop::{manuel::BASIC, ApplyStateMap, CloseMode, DeferWalks, Position},
    pure_after_apply, set_index, set_scope_path,
    shader::draw_popup::DrawPopup,
    themes::conf::Conf,
};

live_design! {
    link genui_basic;

    pub GPopupBase = {{GPopup}}{}
}

#[derive(Live, LiveRegister)]
pub struct GPopup {
    #[live]
    pub style: PopupProp,
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
    #[rust(true)]
    pub sync: bool,
    // --- popup ---------------------
    #[live]
    pub close_mode: CloseMode,
    // --- draw ----------------------
    #[live]
    pub draw_popup: DrawPopup,
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
    #[live(false)]
    pub block_signal_event: bool,
    #[live]
    pub event_order: EventOrder,
}

impl LiveHook for GPopup {
    pure_after_apply!();
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
            &PopupBasicStyle::live_props(),
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
        let style = &cx.global::<Conf>().components.popup;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let style = self.style.get(self.current_state());
        self.draw_popup.merge(&(*style).into());
        Ok(())
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        self.style.sync(&self.apply_state_map);
    }

    fn current_state(&self) -> Self::State {
        PopupState::Basic
    }

    fn begin(&mut self, _cx: &mut Cx2d) -> () {}

    fn end(&mut self, _cx: &mut Cx2d, _scope: &mut Scope, _shift_area: Area, _shift: DVec2) -> () {}

    fn draw_popup(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Option<Position>,
        angle_offset: f32,
        redraw: &mut bool,
    ) -> () {
        let _ = position.map(|position| {
            self.draw_popup.position = position;
        });
        self.draw_popup.angle_offset = angle_offset;
        // draw the popup ------------------------------------------------------------------------
        self.draw_walk(cx, scope, None);
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
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible && event.requires_visibility() {
            return;
        }

        // animation_open_then_redraw!(self, cx, event);

        if self.block_signal_event {
            if let Event::Signal = event {
                return;
            }
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            let mut actions = Vec::new();
            scroll_bars.handle_main_event(cx, event, scope, &mut actions);
            if actions.len() > 0 {
                cx.redraw_area_and_children(self.area);
            };
        }

        // If the UI tree has changed significantly (e.g. AdaptiveView varaints changed),
        // we need to clear the cache and re-query widgets.
        if cx.widget_query_invalidation_event.is_some() {
            self.find_cache.borrow_mut().clear();
        }

        match &self.event_order {
            EventOrder::Up => {
                for (id, child) in self.children.iter_mut().rev() {
                    scope.with_id(*id, |scope| {
                        child.handle_event_with(cx, event, scope, sweep_area);
                    });
                }
            }
            EventOrder::Down => {
                for (id, child) in self.children.iter_mut() {
                    scope.with_id(*id, |scope| {
                        child.handle_event_with(cx, event, scope, sweep_area);
                    });
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some((_, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                        scope.with_id(*id, |scope| {
                            child.handle_event_with(cx, event, scope, sweep_area);
                        });
                    }
                }
            }
        }

        // match event.hit_designer(cx, self.area) {
        //     HitDesigner::DesignerPick(_e) => {
        //         cx.widget_action(uid, &scope.path, WidgetDesignAction::PickedBody)
        //     }
        //     _ => (),
        // }

        // if self.visible || self.animator.live_ptr.is_some() {
        //     let hit = event.hits_with_capture_overload(cx, self.area(), self.capture_overload);
        //     if self.disabled {
        //         self.handle_when_disabled(cx, event, hit);
        //     } else {
        //         self.handle_widget_event(cx, event, hit, self.area());
        //     }
        // }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
    pub fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Option<Walk>) {
        let style = self.style.get(self.current_state());

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

            let layout = style.layout().with_scroll(scroll);
            let walk = walk.unwrap_or(style.walk());
            if style.background_visible {
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
                            let _ = scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk));
                        } else if let Some(fw) = cx.defer_walk(walk) {
                            self.defer_walks.push((*id, fw));
                        } else {
                            self.draw_state.set(DrawState::Drawing(step, true));
                            let _ = scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk));
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
                    let _ = scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk));
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                };

                if style.background_visible {
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

    pub fn draw_container_drawer(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Position,
        proportion: f32,
        redraw: &mut bool,
    ) {
        self.draw_popup.position = position;
        let w = Walk {
            height: Size::All,
            width: Size::All,
            ..Default::default()
        };
        let popup_size = cx.peek_walk_turtle(w).size;
        // now get virtual box as rect
        let (adjust_size, adjust_pos) = match position {
            Position::Left | Position::LeftTop | Position::LeftBottom => {
                let x = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.x
                };
                let size = DVec2 { x, y: popup_size.y };
                let pos = DVec2 { x: 0.0, y: 0.0 };
                (size, pos)
            }
            Position::Right | Position::RightTop | Position::RightBottom => {
                let x = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.x
                };
                let size = DVec2 { x, y: popup_size.y };
                let pos = DVec2 {
                    x: (1.0 - proportion) as f64 * popup_size.x,
                    y: 0.0,
                };
                (size, pos)
            }
            Position::Top | Position::TopLeft | Position::TopRight => {
                let y = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.y
                };
                let size = DVec2 { x: popup_size.x, y };
                let pos = DVec2 { x: 0.0, y: 0.0 };
                (size, pos)
            }
            Position::Bottom | Position::BottomLeft | Position::BottomRight => {
                let y = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.y
                };
                let size = DVec2 { x: popup_size.x, y };
                let pos = DVec2 {
                    x: 0.0,
                    y: (1.0 - proportion) as f64 * popup_size.y,
                };
                (size, pos)
            }
        };

        let walk = Walk {
            abs_pos: Some(adjust_pos),
            width: Size::Fixed(adjust_size.x),
            height: Size::Fixed(adjust_size.y),
            ..Default::default()
        };

        self.draw_walk(cx, scope, Some(walk));

        if *redraw {
            self.draw_popup.redraw(cx);
            *redraw = !*redraw;
        }
    }

    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_popup.area().clipped_rect(cx).contains(pos)
    }
    pub fn container_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.area.clipped_rect(cx).contains(pos)
    }
}
