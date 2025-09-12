mod event;
mod prop;
mod rely;
use std::cell::RefCell;

pub use event::*;
use makepad_widgets::{event::FingerLongPressEvent, *};
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Style},
    },
    error::Error,
    event_option, event_option_ref, getter, getter_setter_ref, hit_finger_down, hit_finger_up,
    hit_hover_in, hit_hover_out, lifecycle, play_animation,
    prop::{
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
        traits::{ToColor, ToFloat},
        ApplyStateMap, Radius,
    },
    pure_after_apply, set_animation, set_index, set_scope_path, setter,
    shader::draw_view::DrawView,
    sync,
    themes::{conf::Conf, Theme},
    visible, ComponentAnInit,
};
pub use rely::*;

use super::traits::Component;

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GViewBase = {{GView}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_view: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_view: <AN_DRAW_VIEW> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_view: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_view: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GView {
    // --- prop -------------------
    #[live]
    pub style: ViewStyle,
    // --- other props ------------
    #[live(true)]
    pub visible: bool,
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
    #[live(false)]
    pub event_open: bool,
    #[live]
    pub disabled: bool,
    // --- texture and cache ------
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    find_cache: RefCell<SmallVec<[(u64, WidgetSet); 3]>>,
    #[rust]
    scroll_bars_obj: Option<Box<ScrollBars>>,
    #[rust]
    pub view_size: Option<DVec2>,
    #[rust]
    pub area: Area,
    #[rust]
    draw_list: Option<DrawList2d>,
    #[rust]
    texture_cache: Option<ViewTextureCache>,
    #[rust]
    defer_walks: SmallVec<[(LiveId, DeferWalk); 1]>,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    pub children: SmallVec<[(LiveId, WidgetRef); 2]>,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
    // --- animation --------------
    #[animator]
    animator: Animator,
    #[live(false)]
    pub animation_open: bool,
    #[live(true)]
    pub animation_spread: bool,
    // --- draw -------------------
    #[live]
    pub draw_view: DrawView,
    // --- lifecycle --------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub apply_state_map: ApplyStateMap<ViewState>,
    #[rust]
    pub state: ViewState,
}

impl LiveHook for GView {
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
        if needs_draw_list(self.optimize) && self.draw_list.is_none() {
            self.draw_list = Some(DrawList2d::new(cx));
        }
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }

        // if apply.from.is_new_from_doc() {
        //     self.render_after_apply(cx);
        // }

        self.set_apply_state_map(
            nodes,
            index,
            &ViewBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(pressed),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(ViewState::Basic, applys);
                }
                HOVER => {
                    component.apply_state_map.insert(ViewState::Hover, applys);
                }
                PRESSED => {
                    component.apply_state_map.insert(ViewState::Pressed, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(ViewState::Disabled, applys);
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

impl WidgetNode for GView {
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
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
        let _ = self.render(cx);
        self.area.redraw(cx);
        self.draw_view.redraw(cx);
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

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    visible!();
}

impl Widget for GView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let style = self.style.get(self.state);
        // the beginning state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            if !self.visible {
                self.draw_state.end();
                self.set_scope_path(&scope.path);
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
                        self.set_scope_path(&scope.path);
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
                        self.set_scope_path(&scope.path);
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

            let layout = style.layout().with_scroll(scroll);

            if style.background_visible {
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
                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.draw_scroll_bars(cx);
                };

                if style.background_visible {
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
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible && event.requires_visibility() {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().view = true;

        animation_open_then_redraw!(self, cx, event);

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

        if self.visible || self.animator.live_ptr.is_some() {
            let hit = event.hits_with_capture_overload(cx, sweep_area, self.capture_overload);
            if self.disabled {
                self.handle_when_disabled(cx, event, hit);
            } else {
                self.handle_widget_event(cx, event, hit, sweep_area);
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible && event.requires_visibility() {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().view = true;

        animation_open_then_redraw!(self, cx, event);

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
                    });
                }
            }
            EventOrder::List(list) => {
                for id in list {
                    if let Some((_, child)) = self.children.iter_mut().find(|(id2, _)| id2 == id) {
                        scope.with_id(*id, |scope| {
                            child.handle_event(cx, event, scope);
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

        if self.visible || self.animator.live_ptr.is_some() {
            let hit = event.hits_with_capture_overload(cx, self.area(), self.capture_overload);
            if self.disabled {
                self.handle_when_disabled(cx, event, hit);
            } else {
                self.handle_widget_event(cx, event, hit, self.area());
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
}

impl Component for GView {
    type Error = Error;
    type State = ViewState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.view;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.state;
        let style = self.style.get(state);
        self.draw_view.merge(style);
        if self.disabled {
            self.switch_state(ViewState::Disabled);
        }
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, _event: &Event, hit: Hit, area: Area) {
        match hit {
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, ViewState::Pressed);
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerMove(e) => {
                self.switch_state_with_animation(cx, ViewState::Pressed);
                self.active_move(cx, e);
            }
            Hit::FingerLongPress(e) => {
                self.switch_state_with_animation(cx, ViewState::Pressed);
                self.active_long_press(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, ViewState::Hover);
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.switch_state_with_animation(cx, ViewState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.switch_state_with_animation(cx, ViewState::Basic);
                    hit_finger_up!(self, cx, e);
                }
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.style.get(self.state).cursor);
                self.switch_state_with_animation(cx, ViewState::Hover);
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, ViewState::Basic);
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerHoverOver(e) => {
                cx.set_cursor(self.style.get(self.state).cursor);
                self.switch_state_with_animation(cx, ViewState::Hover);
                self.play_animation(cx, id!(hover.on));
                self.active_hover_over(cx, e);
            }
            Hit::KeyDown(e) => {
                self.switch_state_with_animation(cx, ViewState::Pressed);
                self.active_key_down(cx, e);
            }
            Hit::KeyUp(e) => {
                self.switch_state_with_animation(cx, ViewState::Basic);
                self.active_key_up(cx, e);
            }
            _ => (),
        }
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, ViewState::Disabled);
                cx.set_cursor(self.style.get(self.state).cursor);
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
        self.switch_state(state);
        self.set_animation(cx);
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
            let basic_prop = self.style.get(ViewState::Basic);
            let hover_prop = self.style.get(ViewState::Hover);
            let pressed_prop = self.style.get(ViewState::Pressed);
            let disabled_prop = self.style.get(ViewState::Disabled);
            let (mut basic_index, mut hover_index, mut pressed_index, mut disabled_index) =
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
                    live_id!(pressed).as_instance(),
                ],
            ) {
                pressed_index = Some(index);
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
                    hover_index => {
                        background_color => hover_prop.background_color,
                        border_color => hover_prop.border_color,
                        border_radius => hover_prop.border_radius,
                        border_width => (hover_prop.border_width as f64),
                        shadow_color => hover_prop.shadow_color,
                        spread_radius => (hover_prop.spread_radius as f64),
                        blur_radius => (hover_prop.blur_radius as f64),
                        shadow_offset => hover_prop.shadow_offset,
                        background_visible => hover_prop.background_visible.to_f64()
                    },
                    pressed_index => {
                        background_color => pressed_prop.background_color,
                        border_color => pressed_prop.border_color,
                        border_radius => pressed_prop.border_radius,
                        border_width => (pressed_prop.border_width as f64),
                        shadow_color => pressed_prop.shadow_color,
                        spread_radius => (pressed_prop.spread_radius as f64),
                        blur_radius => (pressed_prop.blur_radius as f64),
                        shadow_offset => pressed_prop.shadow_offset,
                        background_visible => pressed_prop.background_visible.to_f64()
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
                ViewState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                ViewState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                ViewState::Pressed => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(pressed).as_instance(),
                    ],
                ),
                ViewState::Disabled => nodes.child_by_path(
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
                        background_color => style.background_color,
                        border_color => style.border_color,
                        border_radius => style.border_radius,
                        border_width => (style.border_width as f64),
                        shadow_color => style.shadow_color,
                        spread_radius => (style.spread_radius as f64),
                        blur_radius => (style.blur_radius as f64),
                        shadow_offset => style.shadow_offset,
                        background_visible => style.background_visible.to_f64()
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

impl GView {
    active_event! {
        active_hover_in: ViewEvent::HoverIn |meta: FingerHoverEvent| => ViewHoverIn {meta},
        active_hover_over: ViewEvent::HoverOver |meta: FingerHoverEvent| => ViewHoverOver {meta},
        active_hover_out: ViewEvent::HoverOut |meta: FingerHoverEvent| => ViewHoverOut {meta},
        active_finger_down: ViewEvent::FingerDown |meta: FingerDownEvent| => ViewFingerDown {meta},
        active_finger_up: ViewEvent::FingerUp |meta: FingerUpEvent| => ViewFingerUp {meta},
        active_long_press: ViewEvent::LongPress |meta: FingerLongPressEvent| => ViewLongPress {meta},
        active_move: ViewEvent::Move |meta: FingerMoveEvent| => ViewMove {meta},
        active_key_down: ViewEvent::KeyDown |meta: KeyEvent| => ViewKeyDown {meta},
        active_key_up: ViewEvent::KeyUp |meta: KeyEvent| => ViewKeyUp {meta},
        active_clicked: ViewEvent::Clicked |meta: FingerUpEvent| => ViewClicked {meta}
    }
    event_option! {
        hover_in: ViewEvent::HoverIn => ViewHoverIn,
        hover_over: ViewEvent::HoverOver => ViewHoverOver,
        hover_out: ViewEvent::HoverOut => ViewHoverOut,
        finger_down: ViewEvent::FingerDown => ViewFingerDown,
        finger_up: ViewEvent::FingerUp => ViewFingerUp,
        long_press: ViewEvent::LongPress => ViewLongPress,
        finger_move: ViewEvent::Move => ViewMove,
        key_down: ViewEvent::KeyDown => ViewKeyDown,
        key_up: ViewEvent::KeyUp => ViewKeyUp,
        clicked: ViewEvent::Clicked => ViewClicked
    }

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

    getter! {
        GView {
            get_theme(Theme) {|c| {c.style.basic.get_theme()}},
            get_background_color(String) {|c| {c.style.basic.get_background_color().to_hex_string()}},
            get_border_color(String) {|c| {c.style.basic.get_border_color().to_hex_string()}},
            get_border_radius(Radius) {|c| {c.style.basic.get_border_radius()}},
            get_border_width(f32) {|c| {c.style.basic.get_border_width()}},
            get_shadow_color(String) {|c| {c.style.basic.get_shadow_color().to_hex_string()}},
            get_spread_radius(f32) {|c| {c.style.basic.get_spread_radius()}},
            get_blur_radius(f32) {|c| {c.style.basic.get_blur_radius()}},
            get_shadow_offset(Vec2) {|c| {c.style.basic.get_shadow_offset()}},
            get_background_visible(bool) {|c| {c.style.basic.get_background_visible()}},
            get_rotation(f32) {|c| {c.style.basic.get_rotation()}},
            get_scale(f32) {|c| {c.style.basic.get_scale()}},
            get_align(Align) {|c| {c.style.basic.get_align()}},
            get_flow(Flow) {|c| {c.style.basic.get_flow()}},
            get_spacing(f64) {|c| {c.style.basic.get_spacing()}},
            get_padding(Padding) {|c| {c.style.basic.get_padding()}},
            get_margin(Margin) {|c| {c.style.basic.get_margin()}},
            get_clip_x(bool) {|c| {c.style.basic.get_clip_x()}},
            get_clip_y(bool) {|c| {c.style.basic.get_clip_y()}},
            get_cursor(MouseCursor) {|c| {c.style.basic.get_cursor()}},
            get_height(Size) {|c| {c.style.basic.get_height()}},
            get_width(Size) {|c| {c.style.basic.get_width()}},
            get_visible(bool) {|c| {c.visible}},
            get_disabled(bool) {|c| {c.disabled}},
            get_dpi_factor(Option<f64>) {|c| {c.dpi_factor}},
            get_capture_overload(bool) {|c| {c.capture_overload}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_optimize(ViewOptimize) {|c| {c.optimize}},
            get_scroll(DVec2) {|c| {c.scroll}},
            get_abs_pos(Option<DVec2>) {|c| {c.style.basic.get_abs_pos()}}
        }
    }
    setter! {
        GView {
            set_theme(theme: Theme) {|c, _cx| {c.style.basic.set_theme(theme); c.style.basic.sync(ViewState::Basic); Ok(())}},
            set_background_color(color: String) {|c, _cx| {let color = Vec4::from_hex(&color)?; c.style.basic.set_background_color(color); Ok(())}},
            set_border_color(color: String) {|c, _cx| {let color = Vec4::from_hex(&color)?; c.style.basic.set_border_color(color); Ok(())}},
            set_border_radius(radius: Radius) {|c, _cx| {c.style.basic.set_border_radius(radius); Ok(())}},
            set_border_width(width: f32) {|c, _cx| {c.style.basic.set_border_width(width); Ok(())}},
            set_shadow_color(color: String) {|c, _cx| {let color = Vec4::from_hex(&color)?; c.style.basic.set_shadow_color(color); Ok(())}},
            set_spread_radius(radius: f32) {|c, _cx| {c.style.basic.set_spread_radius(radius); Ok(())}},
            set_blur_radius(radius: f32) {|c, _cx| {c.style.basic.set_blur_radius(radius); Ok(())}},
            set_shadow_offset(offset: Vec2) {|c, _cx| {c.style.basic.set_shadow_offset(offset); Ok(())}},
            set_background_visible(visible: bool) {|c, _cx| {c.style.basic.set_background_visible(visible); Ok(())}},
            set_rotation(rotation: f32) {|c, _cx| {c.style.basic.set_rotation(rotation); Ok(())}},
            set_scale(scale: f32) {|c, _cx| {c.style.basic.set_scale(scale); Ok(())}},
            set_align(align: Align) {|c, _cx| {c.style.basic.set_align(align); Ok(())}},
            set_flow(flow: Flow) {|c, _cx| {c.style.basic.set_flow(flow); Ok(())}},
            set_spacing(spacing: f64) {|c, _cx| {c.style.basic.set_spacing(spacing); Ok(())}},
            set_padding(padding: Padding) {|c, _cx| {c.style.basic.set_padding(padding); Ok(())}},
            set_margin(margin: Margin) {|c, _cx| {c.style.basic.set_margin(margin); Ok(())}},
            set_clip_x(clip_x: bool) {|c, _cx| {c.style.basic.set_clip_x(clip_x); Ok(())}},
            set_clip_y(clip_y: bool) {|c, _cx| {c.style.basic.set_clip_y(clip_y); Ok(())}},
            set_cursor(cursor: MouseCursor) {|c, _cx| {c.style.basic.set_cursor(cursor); Ok(())}},
            set_height(height: Size) {|c, _cx| {c.style.basic.set_height(height); Ok(())}},
            set_width(width: Size) {|c, _cx| {c.style.basic.set_width(width); Ok(())}},
            set_visible(visible: bool) {|c, _cx| {c.visible = visible; Ok(())}},
            set_disabled(disabled: bool) {|c, _cx| {c.disabled = disabled; c.clear_animation(_cx); Ok(())}},
            set_dpi_factor(dpi_factor: Option<f64>) {|c, _cx| {c.dpi_factor = dpi_factor;  Ok(())}},
            set_capture_overload(capture_overload: bool) {|c, _cx| { c.capture_overload = capture_overload; Ok(())}},
            set_grab_key_focus(grab_key_focus: bool) {|c, _cx| {c.grab_key_focus = grab_key_focus; Ok(())}},
            set_optimize(optimize: ViewOptimize) {|c, _cx| {c.optimize = optimize; Ok(())}},
            set_scroll(scroll: DVec2) {|c, _cx| {c.scroll = scroll; Ok(())}},
            set_abs_pos(abs_pos: Option<DVec2>) {|c, _cx| {c.style.basic.set_abs_pos(abs_pos); Ok(())}}
        }
    }
}

impl GViewRef {
    event_option_ref! {
        hover_in => ViewHoverIn,
        hover_over => ViewHoverOver,
        hover_out => ViewHoverOut,
        finger_down => ViewFingerDown,
        finger_up => ViewFingerUp,
        long_press => ViewLongPress,
        finger_move => ViewMove,
        key_down => ViewKeyDown,
        key_up => ViewKeyUp,
        clicked => ViewClicked
    }
    getter_setter_ref! {
        get_theme, set_theme -> Theme,
        get_background_color, set_background_color -> String,
        get_border_color, set_border_color -> String,
        get_border_radius, set_border_radius -> Radius,
        get_border_width, set_border_width -> f32,
        get_shadow_color, set_shadow_color -> String,
        get_spread_radius, set_spread_radius -> f32,
        get_blur_radius, set_blur_radius -> f32,
        get_shadow_offset, set_shadow_offset -> Vec2,
        get_background_visible, set_background_visible -> bool,
        get_rotation, set_rotation -> f32,
        get_scale, set_scale -> f32,
        get_align, set_align -> Align,
        get_flow, set_flow -> Flow,
        get_spacing, set_spacing -> f64,
        get_padding, set_padding -> Padding,
        get_margin, set_margin -> Margin,
        get_clip_x, set_clip_x -> bool,
        get_clip_y, set_clip_y -> bool,
        get_cursor, set_cursor -> MouseCursor,
        get_height, set_height -> Size,
        get_width, set_width -> Size,
        get_visible, set_visible -> bool,
        get_disabled, set_disabled -> bool,
        get_dpi_factor, set_dpi_factor -> Option<f64>,
        get_capture_overload, set_capture_overload -> bool,
        get_grab_key_focus, set_grab_key_focus -> bool,
        get_optimize, set_optimize -> ViewOptimize,
        get_scroll, set_scroll -> DVec2,
        get_abs_pos, set_abs_pos -> Option<DVec2>
    }
}
