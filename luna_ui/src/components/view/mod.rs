mod event;
mod prop;
mod rely;
use std::cell::RefCell;

use makepad_widgets::*;
pub use prop::*;

use crate::{
    components::traits::Prop, error::Error, play_animation, prop::traits::ToF32, set_scope_path,
    shader::draw_view::DrawView, themes::Conf
};
pub use rely::*;

use super::traits::Component;

live_design! {
    link luna_basic;

    pub LViewBase = {{LView}} {
        animator: {
            hover = {
                default: off,
                on = {
                    from: {
                        all: Forward {duration: (0.25)},
                        pressed: Forward {duration: (0.25)},
                    },
                    apply: {draw_view: {hover: 1.0, pressed: 0.0}}
                },
                off = {
                    from: {all: Forward {duration: (0.25)}},
                    apply: {draw_view: {hover: 0.0, pressed: 0.0}}
                },
                pressed = {
                    from: {all: Forward {duration: (0.25)}},
                    apply: {draw_view: {hover: 0.0, pressed: 1.0}}
                }
            }
        }
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct LView {
    // --- prop -------------------
    #[live]
    pub prop: ViewProp,
    // --- other props ------------
    #[live(true)]
    pub visible: bool,
    #[live]
    pub abs_pos: Option<DVec2>,
    #[live]
    pub scroll: DVec2,
    #[live]
    pub scroll_bars: Option<LivePtr>,
    #[live]
    pub dpi_factor: Option<f64>,
    #[live]
    pub optimize: ViewOptimize,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    #[live(false)]
    pub capture_overload: bool,
    #[live]
    pub event_order: EventOrder,
    // --- texture and cache ------
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    find_cache: RefCell<SmallVec<[(u64, WidgetSet); 3]>>,
    #[rust]
    scroll_bars_obj: Option<Box<ScrollBars>>,
    #[rust]
    view_size: Option<DVec2>,
    #[rust]
    area: Area,
    #[rust]
    draw_list: Option<DrawList2d>,
    #[rust]
    texture_cache: Option<ViewTextureCache>,
    #[rust]
    defer_walks: SmallVec<[(LiveId, DeferWalk); 1]>,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    children: SmallVec<[(LiveId, WidgetRef); 2]>,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    // --- animation --------------
    #[animator]
    animator: Animator,
    #[live(false)]
    pub animation_open: bool,
    // --- draw -------------------
    #[live]
    pub draw_view: DrawView,
}

impl LiveHook for LView {
    fn before_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            //self.draw_order.clear();
            self.live_update_order.clear();
            self.find_cache.get_mut().clear();
        }
    }

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
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
        if needs_draw_list(self.optimize) && self.draw_list.is_none() {
            self.draw_list = Some(DrawList2d::new(cx));
        }
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }

        if apply.from.is_new_from_doc() {
            self.render_after_apply(cx);
        }
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

impl WidgetNode for LView {
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.current_state());
        Walk {
            abs_pos: self.abs_pos.clone(),
            margin: prop.margin,
            width: prop.width,
            height: prop.height,
        }
    }

    fn area(&self) -> Area {
        self.area
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for (_, child) in &self.children {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx);
        for (_, child) in &mut self.children {
            child.redraw(cx);
        }
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        match cached {
            WidgetCache::Yes | WidgetCache::Clear => {
                if let WidgetCache::Clear = cached {
                    self.find_cache.borrow_mut().clear();
                    if path.len() == 0 {
                        return;
                    }
                }
                let mut hash = 0u64;
                for i in 0..path.len() {
                    hash ^= path[i].0
                }
                if let Some((_, widget_set)) =
                    self.find_cache.borrow().iter().find(|(h, _v)| h == &hash)
                {
                    results.extend_from_set(widget_set);
                    return;
                }
                let mut local_results = WidgetSet::empty();
                if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, &mut local_results);
                    } else {
                        local_results.push(child.clone());
                    }
                }
                for (_, child) in &self.children {
                    child.find_widgets(path, WidgetCache::No, &mut local_results);
                }
                if !local_results.is_empty() {
                    results.extend_from_set(&local_results);
                }
                self.find_cache.borrow_mut().push((hash, local_results));
            }
            WidgetCache::No => {
                if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                    if path.len() > 1 {
                        child.find_widgets(&path[1..], WidgetCache::No, results);
                    } else {
                        results.push(child.clone());
                    }
                }
                for (_, child) in &self.children {
                    child.find_widgets(path, WidgetCache::No, results);
                }
            }
        }
    }
}

impl Widget for LView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let prop = self.prop.get(self.current_state());
        // the beginning state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                self.draw_state.end();
                return DrawStep::done();
            }

            self.defer_walks.clear();

            match self.optimize {
                ViewOptimize::Texture => {
                    let walk = self.walk_from_previous_size(walk);
                    if !cx.will_redraw(self.draw_list.as_mut().unwrap(), walk) {
                        if let Some(texture_cache) = &self.texture_cache {
                            self.draw_view
                                .draw_vars
                                .set_texture(0, &texture_cache.color_texture);
                            let mut rect = cx.walk_turtle_with_area(&mut self.area, walk);
                            // NOTE(eddyb) see comment lower below for why this is
                            // disabled (it used to match `set_pass_scaled_area`).
                            if false {
                                rect.size *= 2.0 / self.dpi_factor.unwrap_or(1.0);
                            }
                            self.draw_view.draw_abs(cx, rect);
                            self.area = self.draw_view.area();
                            cx.set_pass_area(&texture_cache.pass, self.area);
                        }
                        return DrawStep::done();
                    }
                    // lets start a pass
                    if self.texture_cache.is_none() {
                        self.texture_cache = Some(ViewTextureCache {
                            pass: Pass::new(cx),
                            _depth_texture: Texture::new(cx),
                            color_texture: Texture::new(cx),
                        });
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        //cache.pass.set_depth_texture(cx, &cache.depth_texture, PassClearDepth::ClearWith(1.0));
                        texture_cache.color_texture = Texture::new_with_format(
                            cx,
                            TextureFormat::RenderBGRAu8 {
                                size: TextureSize::Auto,
                                initial: true,
                            },
                        );
                        texture_cache.pass.set_color_texture(
                            cx,
                            &texture_cache.color_texture,
                            PassClearColor::ClearWith(vec4(0.0, 0.0, 0.0, 0.0)),
                        );
                    }
                    let texture_cache = self.texture_cache.as_mut().unwrap();
                    cx.make_child_pass(&texture_cache.pass);
                    cx.begin_pass(&texture_cache.pass, self.dpi_factor);
                    self.draw_list.as_mut().unwrap().begin_always(cx)
                }
                ViewOptimize::DrawList => {
                    let walk = self.walk_from_previous_size(walk);
                    if self
                        .draw_list
                        .as_mut()
                        .unwrap()
                        .begin(cx, walk)
                        .is_not_redrawing()
                    {
                        cx.walk_turtle_with_area(&mut self.area, walk);
                        return DrawStep::done();
                    }
                }
                _ => (),
            }

            // ok so.. we have to keep calling draw till we return LiveId(0)
            let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                scroll_bars.begin_nav_area(cx);
                scroll_bars.get_scroll_pos()
            } else {
                self.scroll
            };

            let layout = Layout {
                scroll,
                clip_x: prop.clip_x,
                clip_y: prop.clip_y,
                padding: prop.padding,
                align: prop.align,
                flow: prop.flow,
                spacing: prop.spacing,
            };
            if prop.background_visible {
                self.draw_view.begin(cx, walk, layout);
            } else {
                cx.begin_turtle(walk, layout);
            }
        }

        while let Some(DrawState::Drawing(step, resume)) = self.draw_state.get() {
            if step < self.children.len() {
                if let Some((id, child)) = self.children.get_mut(step) {
                    if child.visible() {
                        let walk = child.walk(cx);
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
                    scope.with_id(*id, |scope| child.draw_walk(cx, scope, walk))?;
                }
                self.draw_state.set(DrawState::DeferWalk(step + 1));
            } else {
                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                };

                if prop.background_visible {
                    if is_texture(self.optimize) {
                        panic!("dont use show_bg and texture caching at the same time");
                    }
                    self.draw_view.end(cx);
                    self.area = self.draw_view.area();
                } else {
                    cx.end_turtle_with_area(&mut self.area);
                };

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.set_area(self.area);
                    scroll_bars.end_nav_area(cx);
                };

                if needs_draw_list(self.optimize) {
                    let rect = self.area.rect(cx);
                    self.view_size = Some(rect.size);
                    self.draw_list.as_mut().unwrap().end(cx);

                    if is_texture(self.optimize) {
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        cx.end_pass(&texture_cache.pass);
                        self.draw_view
                            .draw_vars
                            .set_texture(0, &texture_cache.color_texture);
                        self.draw_view.draw_abs(cx, rect);
                        let area = self.draw_view.area();
                        let texture_cache = self.texture_cache.as_mut().unwrap();

                        cx.set_pass_area(&texture_cache.pass, area);
                    }
                }
                self.draw_state.end();
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible && event.requires_visibility() {
            return;
        }

        let uid = self.widget_uid();
        if self.animation_open {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.redraw(cx);
            }
        }
        let prop = self.prop.get(self.current_state());
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
                        child.handle_event(cx, event, scope);
                    });
                }
            }
            EventOrder::Down => {
                for (id, child) in self.children.iter_mut() {
                    scope.with_id(*id, |scope| {
                        child.handle_event(cx, event, scope);
                    })
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some((_, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                        scope.with_id(*id, |scope| {
                            child.handle_event(cx, event, scope);
                        })
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

        if self.visible || self.animator.live_ptr.is_some() {
            match event.hits_with_capture_overload(cx, self.area(), self.capture_overload) {
                Hit::FingerDown(e) => {
                    if self.grab_key_focus {
                        cx.set_key_focus(self.area());
                    }
                    cx.widget_action(uid, &scope.path, ViewAction::FingerDown(e));
                    if self.animator.live_ptr.is_some() {
                        self.animator_play(cx, id!(hover.pressed));
                        self.switch_state_and_redraw(cx, ViewState::Pressed);
                    }
                }
                Hit::FingerMove(e) => cx.widget_action(uid, &scope.path, ViewAction::FingerMove(e)),
                Hit::FingerLongPress(e) => {
                    cx.widget_action(uid, &scope.path, ViewAction::FingerLongPress(e))
                }
                Hit::FingerUp(e) => {
                    cx.widget_action(uid, &scope.path, ViewAction::FingerUp(e));
                    if self.animator.live_ptr.is_some() {
                        self.animator_play(cx, id!(hover.off));
                        self.switch_state_and_redraw(cx, ViewState::None);
                    }
                }
                Hit::FingerHoverIn(e) => {
                    cx.widget_action(uid, &scope.path, ViewAction::FingerHoverIn(e));
                    cx.set_cursor(prop.cursor);
                    if self.animator.live_ptr.is_some() {
                        self.animator_play(cx, id!(hover.on));
                        self.switch_state_and_redraw(cx, ViewState::Hover);
                    }
                }
                Hit::FingerHoverOut(e) => {
                    cx.widget_action(uid, &scope.path, ViewAction::FingerHoverOut(e));
                    if self.animator.live_ptr.is_some() {
                        self.animator_play(cx, id!(hover.off));
                        self.switch_state_and_redraw(cx, ViewState::None);
                    }
                }
                Hit::KeyDown(e) => cx.widget_action(uid, &scope.path, ViewAction::KeyDown(e)),
                Hit::KeyUp(e) => cx.widget_action(uid, &scope.path, ViewAction::KeyUp(e)),
                _ => (),
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
}

impl Component for LView {
    type Error = Error;
    type State = ViewState;
    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.view;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.current_state();
        self.draw_view.background_color = self.prop.get(state).background_color;
        self.draw_view.border_color = self.prop.get(state).border_color;
        self.draw_view.border_width = self.prop.get(state).border_width;
        self.draw_view.border_radius = self.prop.get(state).border_radius.into();
        self.draw_view.shadow_color = self.prop.get(state).shadow_color;
        self.draw_view.spread_radius = self.prop.get(state).spread_radius;
        self.draw_view.blur_radius = self.prop.get(state).blur_radius;
        self.draw_view.shadow_offset = self.prop.get(state).shadow_offset;
        self.draw_view.background_visible = self.prop.get(state).background_visible.to_f32();
        self.draw_view.rotation = self.prop.get(state).rotation;
        self.draw_view.scale = self.prop.get(state).scale;

        Ok(())
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn current_state(&self) -> Self::State {
        self.draw_view.current_state()
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        self.draw_view.apply_over(
            cx,
            live! {
                hover: 0.0,
                pressed: 0.0
            },
        );
    }

    fn switch_state_and_redraw(&mut self, cx: &mut Cx, state: ViewState) -> () {
        if !self.animation_open {
            return;
        }

        match state {
            ViewState::None => {
                // switch to normal state
                if self.draw_view.hover != 0.0 || self.draw_view.pressed != 0.0 {
                    self.draw_view.hover = 0.0;
                    self.draw_view.pressed = 0.0;
                }
            }
            ViewState::Hover => {
                if self.draw_view.hover != 1.0 {
                    self.draw_view.hover = 1.0;
                    self.draw_view.pressed = 0.0;
                }
            }
            ViewState::Pressed => {
                if self.draw_view.pressed != 1.0 {
                    self.draw_view.hover = 0.0;
                    self.draw_view.pressed = 1.0;
                }
            }
        }
        let _ = self.render(cx);
        self.draw_view.redraw(cx);
    }

    play_animation!();
    set_scope_path!();
}

impl LView {
    pub fn walk_from_previous_size(&self, walk: Walk) -> Walk {
        let view_size = self.view_size.unwrap_or(DVec2::default());
        Walk {
            abs_pos: walk.abs_pos,
            width: if walk.width.is_fill() {
                walk.width
            } else {
                Size::Fixed(view_size.x)
            },
            height: if walk.height.is_fill() {
                walk.height
            } else {
                Size::Fixed(view_size.y)
            },
            margin: walk.margin,
        }
    }
}
