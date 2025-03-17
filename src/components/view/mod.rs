pub mod event;
pub mod register;

use event::*;

use std::{cell::RefCell, collections::HashMap};

use makepad_widgets::*;

use crate::{
    active_event, animatie_fn, event_option, getter, play_animation, ref_area, ref_event_option, ref_getter_setter, ref_redraw_mut, ref_render, set_event, set_scope_path, setter, shader::draw_view::DrawGView, themes::Themes, utils::{set_cursor, BoolToF32, ThemeColor, ToBool}
};

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::GLOBAL_DURATION;

    pub GViewBase = {{GView}}{
        height: Fill,
        width: Fill,
        spread_radius: 0.0,
        clip_x: false,
        clip_y: false,
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_view: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_view: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_view: {hover: 0.0, focus: 1.0}
                    }
                }
            }
        }
    }
}

/// # GView Component
///
/// The `GView` component is designed for creating a custom graphical view with advanced layout, drawing, and event handling capabilities. It supports various graphical properties like background color, borders, shadows, and animations, providing flexibility in appearance and interaction.
///
/// ## Animation
///
/// This component supports animations, particularly for hover and focus states. The default hover and focus animations are defined using the `animator` field:
/// - **hover.off**:  
///   - `draw_view.hover`: changes to `0.0`  
///   - `draw_view.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.on**:  
///   - `draw_view.hover`: changes to `1.0`  
///   - `draw_view.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.focus**:  
///   - `draw_view.hover`: changes to `0.0`  
///   - `draw_view.focus`: changes to `1.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
///
/// Animations can be customized to control transitions between different states, ensuring smooth visual feedback for user interactions.
///
/// ## Event
///
/// The `GView` component supports a variety of events for user interaction. It includes:
/// - **HoverIn**: Triggered when the mouse hovers into the component area.
/// - **HoverOut**: Triggered when the mouse leaves the component area.
/// - **Click**: Triggered when the component is clicked.
/// - **Drag**: Triggered when the component is dragged.
/// - **Key Events**: Handles `KeyDown` and `KeyUp` events for keyboard interactions.
///
/// Each event is processed through methods like `handle_event`, where interactions are managed and animations are triggered in response to user actions.
///
/// ## Props
///
/// | Macro  | Prop               | Description                                      | Type               | Default  |
/// |--------|--------------------|--------------------------------------------------|--------------------|----------|
/// | live   | `theme`             | The visual theme (Dark or Light)                 | `Themes`           | `Themes::Dark` |
/// | live   | `background_color`  | The background color of the view                 | `Option<Vec4>`     | `None`   |
/// | live   | `hover_color`       | The color of the view on hover                   | `Option<Vec4>`     | `None`   |
/// | live   | `focus_color`       | The color of the view on focus                   | `Option<Vec4>`     | `None`   |
/// | live   | `border_color`      | The color of the view’s border                   | `Option<Vec4>`     | `None`   |
/// | live   | `border_width`      | The width of the border                          | `f32`              | `0.0`    |
/// | live   | `border_radius`     | The radius for rounded corners                   | `f32`              | `2.0`    |
/// | live   | `visible`           | Controls the visibility of the component         | `bool`             | `true`   |
/// | live   | `background_visible`| Controls the visibility of the background        | `bool`             | `true`   |
/// | live   | `shadow_color`      | The color of the shadow                          | `Option<Vec4>`     | `None`   |
/// | live   | `spread_radius`     | The radius of the shadow spread                  | `f32`              | `4.8`    |
/// | live   | `blur_radius`       | The radius of the shadow blur                    | `f32`              | `4.8`    |
/// | live   | `shadow_offset`     | The offset of the shadow                         | `Vec2`             | `(0.0, 0.0)` |
/// | live   | `cursor`            | The cursor to display when hovering over the view| `Option<MouseCursor>` | `None` |
/// | live   | `animation_key`     | Boolean to enable animations                     | `bool`             | `false`  |
/// | walk   | `abs_pos`           | Absolute position for layout                     | `Option<DVec2>`    | `None`   |
/// | walk   | `margin`            | Margin size around the view                      | `Margin`           | `Margin::default()` |
/// | walk   | `width`             | Width of the view                               | `Size`             | `Size::default()` |
/// | walk   | `height`            | Height of the view                              | `Size`             | `Size::default()` |
/// | layout | `scroll`            | Scroll position for layout                      | `DVec2`            | `(0.0, 0.0)` |
/// | layout | `clip_x`            | Clip content horizontally                       | `bool`             | `true`   |
/// | layout | `clip_y`            | Clip content vertically                         | `bool`             | `true`   |
/// | layout | `padding`           | Padding within the view                         | `Padding`          | `Padding::default()` |
/// | layout | `align`             | Alignment for content                           | `Align`            | `Align::default()` |
/// | layout | `flow`              | Flow direction of the content                   | `Flow`             | `Flow::default()` |
/// | layout | `spacing`           | Spacing between elements                        | `f64`              | `0.0`    |
#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GView {
    #[live(Themes::Dark)]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(4.8)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(false)]
    pub animation_key: bool,
    // scroll ---------------------
    #[live]
    pub scroll_bars: Option<LivePtr>,
    #[rust]
    pub scroll_bars_obj: Option<Box<ScrollBars>>,
    // control ---------------------
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(false)]
    pub block_signal_event: bool,
    // deref ---------------------
    #[live]
    pub draw_view: DrawGView,
    #[live]
    pub min_width: Option<f32>,
    #[live]
    pub min_height: Option<f32>,
    #[live]
    pub max_width: Option<f32>,
    #[live]
    pub max_height: Option<f32>,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub draw_state: DrawStateWrap<DrawState>,
    // #[rust]
    // pub children: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    pub children: Vec<(LiveId, WidgetRef)>,
    // #[rust]
    // pub draw_order: Vec<LiveId>,
    #[live]
    pub event_order: EventOrder,
    // #[rust]
    // pub defer_walks: Vec<(LiveId, DeferWalk)>,
    #[rust]
    defer_walks: SmallVec<[(LiveId, DeferWalk); 1]>,
    #[animator]
    pub animator: Animator,
    #[rust]
    pub find_cache: RefCell<HashMap<u64, WidgetSet>>,
    // optimize ---------------------
    #[live]
    pub dpi_factor: Option<f64>,
    #[live]
    pub optimize: ViewOptimize,
    #[rust]
    pub draw_list: Option<DrawList2d>,
    #[rust]
    pub view_size: Option<DVec2>,
    #[rust]
    pub texture_cache: Option<ViewTextureCache>,
    #[rust]
    pub area: Area,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[live(false)]
    pub capture_overload: bool,
    #[live(true)]
    pub event_key: bool,
    /// box walk is used to store the walk of the view, it helps to fix_walk fn
    #[rust]
    pub restore_walk: Option<Walk>,
    /// do fix then redraw at the first time
    #[rust(true)]
    pub fix_flag: bool,
    #[live(false)]
    pub block_child_events: bool,
    #[rust]
    live_update_order: SmallVec<[LiveId; 1]>,
}

pub struct ViewTextureCache {
    pass: Pass,
    _depth_texture: Texture,
    color_texture: Texture,
}

pub trait OptimizeFor {
    fn is_texture(&self) -> bool;
    fn is_draw_list(&self) -> bool;
    fn needs_draw_list(&self) -> bool;
}

impl OptimizeFor for ViewOptimize {
    fn is_texture(&self) -> bool {
        if let Self::Texture = self {
            true
        } else {
            false
        }
    }
    fn is_draw_list(&self) -> bool {
        if let Self::DrawList = self {
            true
        } else {
            false
        }
    }
    fn needs_draw_list(&self) -> bool {
        return self.is_texture() || self.is_draw_list();
    }
}

#[derive(Clone)]
pub enum DrawState {
    Drawing(usize, bool),
    DeferWalk(usize),
}

impl LiveHook for GView {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            // self.draw_order.clear();
            self.live_update_order.clear();
            self.find_cache.get_mut().clear();
        }
    }
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // if !self.visible {
        //     return;
        // }
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
        if self.optimize.needs_draw_list() && self.draw_list.is_none() {
            self.draw_list = Some(DrawList2d::new(cx));
        }
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }
        if let Err(e) = self.render(cx) {
            error!("GView render error: {:?}", e);
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
                // if !self.visible {
                //     nodes.skip_node(index);
                // }

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

impl Widget for GView {
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

        if self.animation_key {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.redraw(cx);
            }
        }

        if self.block_signal_event {
            if let Event::Signal = event {
                return;
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            let mut actions = Vec::new();
            scroll_bars.handle_main_event(cx, event, scope, &mut actions);
            if actions.len().gt(&0) {
                cx.redraw_area_and_children(self.area());
            }
        }

        if !self.block_child_events {
            match &self.event_order {
                EventOrder::Down => {
                    for (id, child) in self.children.iter_mut() {
                        scope.with_id(*id, |scope| {
                            child.handle_event_with(cx, event, scope, sweep_area);
                        });
                    }
                }
                EventOrder::Up => {
                    // the default event order is Up
                    for (id, child) in self.children.iter_mut().rev() {
                        scope.with_id(*id, |scope| {
                            child.handle_event_with(cx, event, scope, sweep_area);
                        });
                    }
                }
                EventOrder::List(list) => {
                    for id in list {
                        if let Some((_, child)) =
                            self.children.iter_mut().find(|(id2, _)| id2 == id)
                        {
                            scope.with_id(*id, |scope| {
                                child.handle_event_with(cx, event, scope, sweep_area);
                            });
                        }
                    }
                }
            }
        }

        // handle event and set cursor to control
        match event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        ) {
            Hit::KeyDown(e) => {
                if self.grab_key_focus {
                    self.active_key_down(cx, e);
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    self.active_key_up(cx, e);
                }
            }
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                self.play_animation(cx, id!(hover.focus));
                self.active_focus(cx, e);
            }
            Hit::FingerMove(e) => {
                self.active_drag(cx, e);
            }
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.play_animation(cx, id!(hover.on));
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOver(e) => {
                self.active_hover_over(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.play_animation(cx, id!(hover.off));
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.play_animation(cx, id!(hover.off));
                    self.active_focus_lost(cx, e);
                }
            }
            _ => (),
        }
        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        // if self.restore_walk.is_none() {
        //     self.restore_walk.replace(self.walk.clone());
        // }

        // do fix walk
        // let walk = self.fix_walk(cx, walk);

        // begin the draw state
        if self.draw_state.begin(cx, DrawState::Drawing(0, false)) {
            self.set_scope_path(&scope.path);
            if !self.visible {
                // visible is false, so we are done
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
                        texture_cache.pass.add_color_texture(
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

            // get scroll position
            let scroll = if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                scroll_bars.begin_nav_area(cx);
                scroll_bars.get_scroll_pos()
            } else {
                self.layout.scroll
            };

            // begin draw the view
            if self.visible {
                self.draw_view
                    .begin(cx, walk, self.layout.with_scroll(scroll)); //.with_scale(2.0 / self.dpi_factor.unwrap_or(2.0)));
            } else {
                cx.begin_turtle(walk, self.layout.with_scroll(scroll)); //.with_scale(2.0 / self.dpi_factor.unwrap_or(2.0)));
            }
        }

        // loop handle the inner children
        while let Some(DrawState::Drawing(step, resume)) = self.draw_state.get() {
            if step < self.children.len() {
                //let id = self.draw_order[step];
                if let Some((id, child)) = self.children.get_mut(step) {
                    if child.is_visible() {
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

        // loop handle the defer walk
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
                }
                if self.visible {
                    if self.optimize.is_texture() {
                        panic!("dont use background_visible and texture caching at the same time");
                    }
                    self.draw_view.end(cx);
                    self.area = self.draw_view.area();
                } else {
                    cx.end_turtle_with_area(&mut self.area);
                };

                // // draw background
                // self.draw_view.end(cx);

                if let Some(scroll_bars) = &mut self.scroll_bars_obj {
                    scroll_bars.set_area(self.area);
                    scroll_bars.end_nav_area(cx);
                }

                if self.optimize.needs_draw_list() {
                    let rect = self.area.rect(cx);
                    self.view_size = Some(rect.size);
                    self.draw_list.as_mut().unwrap().end(cx);

                    if self.optimize.is_texture() {
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        cx.end_pass(&texture_cache.pass);
                        /*if cache.pass.id_equals(4){
                            self.draw_bg.draw_vars.set_uniform(cx, id!(marked),&[1.0]);
                        }
                        else{
                            self.draw_bg.draw_vars.set_uniform(cx, id!(marked),&[0.0]);
                        }*/
                        self.draw_view
                            .draw_vars
                            .set_texture(0, &texture_cache.color_texture);
                        self.draw_view.draw_abs(cx, rect);
                        let area = self.draw_view.area();
                        let texture_cache = self.texture_cache.as_mut().unwrap();
                        /* if false {
                            // FIXME(eddyb) this was the previous logic,
                            // but the only tested apps that use `CachedView`
                            // are sized correctly (regardless of `dpi_factor`)
                            // *without* extra scaling here.
                            cx.set_pass_scaled_area(
                                &texture_cache.pass,
                                area,
                                2.0 / self.dpi_factor.unwrap_or(1.0),
                            );
                        } else {*/
                        cx.set_pass_area(&texture_cache.pass, area);
                        //}
                    }
                }
                self.draw_state.end();
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        if self.animation_key {
            if self.animator_handle_event(cx, event).must_redraw() {
                self.redraw(cx);
            }
        }

        if self.block_signal_event {
            if let Event::Signal = event {
                return;
            }
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            let mut actions = Vec::new();
            scroll_bars.handle_main_event(cx, event, scope, &mut actions);
            if actions.len().gt(&0) {
                cx.redraw_area_and_children(self.area());
            }
        }

        if !self.block_child_events {
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
                        if let Some((_, child)) =
                            self.children.iter_mut().find(|(id2, _)| id2 == id)
                        {
                            scope.with_id(*id, |scope| {
                                child.handle_event(cx, event, scope);
                            });
                        }
                    }
                }
            }
        }

        // 构建统一事件: Hover的冒泡处理, 需要判断传过来的param中的鼠标位置是否在当前的区域内
        // if let Event::Actions(actions) = event{
        //     for action in actions {
        //         if let Some(actions) = action.as_widget_action(){
        //             if let UnifiedEvent::HoverIn(_) = actions.cast(){
        //                 self.animator_play(cx, id!(hover.on));
        //             }
        //         }
        //     }
        // }

        // handle event and set cursor to control
        match event.hits_with_capture_overload(cx, self.area(), self.capture_overload) {
            Hit::KeyDown(e) => {
                if self.grab_key_focus {
                    self.active_key_down(cx, e);
                }
            }
            Hit::KeyUp(e) => {
                if self.grab_key_focus {
                    self.active_key_up(cx, e);
                }
            }
            Hit::FingerDown(e) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
                self.play_animation(cx, id!(hover.focus));
                self.active_focus(cx, e);
            }
            Hit::FingerMove(e) => {
                self.active_drag(cx, e);
            }
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.play_animation(cx, id!(hover.on));
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOver(e) => {
                self.active_hover_over(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.play_animation(cx, id!(hover.off));
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.play_animation(cx, id!(hover.off));
                    self.active_focus_lost(cx, e);
                }
            }
            _ => (),
        }

        if let Some(scroll_bars) = &mut self.scroll_bars_obj {
            scroll_bars.handle_scroll_event(cx, event, scope, &mut Vec::new());
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl WidgetNode for GView {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for (_, child) in &self.children {
            let x = child.uid_to_widget(uid);
            if !x.is_empty() {
                return x;
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        match cached {
            WidgetCache::Yes | WidgetCache::Clear => {
                if let WidgetCache::Clear = cached {
                    self.find_cache.borrow_mut().clear();
                }
                let mut hash = 0u64;
                for i in 0..path.len() {
                    hash ^= path[i].0
                }
                if let Some(widget_set) = self.find_cache.borrow().get(&hash) {
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
                self.find_cache.borrow_mut().insert(hash, local_results);
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

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx);
        self.draw_view.redraw(cx);
        for (_, child) in &self.children {
            child.redraw(cx);
        }
    }
    fn area(&self) -> Area {
        self.area
    }
}

impl GView {
    set_scope_path!();
    play_animation!();
    event_option! {
        hover_in: GViewEvent::HoverIn => GViewHoverParam,
        hover_over: GViewEvent::HoverOver => GViewHoverParam,
        hover_out: GViewEvent::HoverOut => GViewHoverParam,
        focus: GViewEvent::Focus => GViewFocusParam,
        focus_lost: GViewEvent::FocusLost => GViewFocusLostParam,
        clicked: GViewEvent::Clicked => GViewClickedParam,
        drag: GViewEvent::Drag => GViewDragParam,
        key_down: GViewEvent::KeyDown => GViewKeyEventParam,
        key_up: GViewEvent::KeyUp => GViewKeyEventParam
    }
    active_event! {
        active_hover_in: GViewEvent::HoverIn |e: FingerHoverEvent| => GViewHoverParam{e},
        active_hover_over: GViewEvent::HoverOver |e: FingerHoverEvent| => GViewHoverParam{e},
        active_hover_out: GViewEvent::HoverOut |e: FingerHoverEvent| => GViewHoverParam{e},
        active_focus: GViewEvent::Focus |e: FingerDownEvent| => GViewFocusParam{e},
        active_focus_lost: GViewEvent::FocusLost |e: FingerUpEvent| => GViewFocusLostParam{e},
        active_clicked: GViewEvent::Clicked |e: FingerUpEvent| => GViewClickedParam{e},
        active_drag: GViewEvent::Drag |e: FingerMoveEvent| => GViewDragParam{e},
        active_key_down: GViewEvent::KeyDown |e: KeyEvent| => GViewKeyEventParam{e},
        active_key_up: GViewEvent::KeyUp |e: KeyEvent| => GViewKeyEventParam{e}
    }
    /// fix walk by min_width, min_height, max_width, max_height
    pub fn fix_walk(&mut self, cx: &mut Cx2d, mut walk: Walk) -> Walk {
        // drawable_size is the biggest draw size of the view
        let drawable_size = cx.turtle().size();

        if let Some(max_width) = self.max_width {
            if drawable_size.x > max_width as f64 {
                walk.width = Size::Fixed(max_width as f64);
            }
        }

        if let Some(min_width) = self.min_width {
            if drawable_size.x < min_width as f64 {
                walk.width = Size::Fixed(min_width as f64);
            }
        }

        if let Some(max_height) = self.max_height {
            if drawable_size.y > max_height as f64 {
                walk.height = Size::Fixed(max_height as f64);
            }
        }

        if let Some(min_height) = self.min_height {
            if drawable_size.y < min_height as f64 {
                walk.height = Size::Fixed(min_height as f64);
            }
        }
        // cx.turtle_mut().update_height_max(27.0, drawable_size.y - self.max_height.unwrap_or(0.0) as f64);

        walk
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

    pub fn child_count(&self) -> usize {
        self.children.len()
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_view.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_view.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_view.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_view.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_view.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ check scroll bar -------------------------------------------
        if self.scroll_bars.is_some() {
            if self.scroll_bars_obj.is_none() {
                self.scroll_bars_obj =
                    Some(Box::new(ScrollBars::new_from_ptr(cx, self.scroll_bars)));
            }
        }
        // ------------------ apply draw_view --------------------------------------------
        self.draw_view.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        Ok(())
        // self.draw_view.redraw(cx);
    }
    /// ## set the absolute position of the view
    /// x, y range: `[0.0, 100.0]`, 0.0 means the left or top, 100.0 means the right or bottom
    /// - if x, y is None, do nothing and return None
    /// - if x, y is Some, set the absolute position of the view and return `Some(bool)`
    ///     - if x, y is the same as the current position, return `Some(false)`
    pub fn set_scroll_pos(&mut self, cx: &mut Cx, x: Option<f64>, y: Option<f64>) -> Option<bool> {
        if x.is_none() && y.is_none() {
            return None;
        }

        // first get the current scroll pos
        if let Some(sc) = self.scroll_bars_obj.as_mut() {
            let current = sc.get_scroll_pos();
            // check x, y is some
            let x = x.unwrap_or(current.x);
            let y = y.unwrap_or(current.y);
            // set the scroll pos
            Some(sc.set_scroll_pos(cx, dvec2(x, y)))
        } else {
            None
        }
    }
    setter! {
        GView{
            set_theme(theme: Themes) {|c, cx| {c.theme = theme; c.render(cx)}},
            set_background_color(color: String) {|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.background_color.replace(color); c.draw_view.background_color = color; Ok(())}},
            set_shadow_color(color: String) {|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.shadow_color.replace(color); c.draw_view.shadow_color = color; Ok(())}},
            set_hover_color(color: String) {|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.hover_color.replace(color); c.draw_view.hover_color = color; Ok(())}},
            set_focus_color(color: String) {|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.focus_color.replace(color); c.draw_view.focus_color = color; Ok(())}},
            set_border_color(color: String) {|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.border_color.replace(color); c.draw_view.border_color = color; Ok(())}},
            set_border_width(width: f32) {|c, _cx| {c.border_width = width; c.draw_view.border_width = width; Ok(())}},
            set_border_radius(radius: f32) {|c, _cx| {c.border_radius = radius; c.draw_view.border_radius = radius; Ok(())}},
            set_shadow_offset(offset: Vec2) {|c, _cx| {c.shadow_offset = offset; c.draw_view.shadow_offset = offset; Ok(())}},
            set_spread_radius(radius: f32) {|c, _cx| {c.spread_radius = radius; c.draw_view.spread_radius = radius; Ok(())}},
            set_blur_radius(radius: f32) {|c, _cx| {c.blur_radius = radius; c.draw_view.blur_radius = radius; Ok(())}},
            set_background_visible(visible: bool) {|c, _cx| {c.background_visible = visible; c.draw_view.background_visible = visible.to_f32(); Ok(())}},
            set_visible(visible: bool) {|c, _cx| {c.visible = visible; Ok(())}},
            set_cursor(cursor: MouseCursor) {|c, _cx| {c.cursor = Some(cursor); Ok(())}},
            set_grab_key_focus(grab: bool) {|c, _cx| {c.grab_key_focus = grab; Ok(())}},
            set_block_signal_event(block: bool) {|c, _cx| {c.block_signal_event = block; Ok(())}},
            set_abs_pos(pos: DVec2) {|c, _cx| {c.walk.abs_pos.replace(pos); Ok(())}},
            set_margin(margin: Margin) {|c, _cx| {c.walk.margin = margin; Ok(())}},
            set_height(height: Size) {|c, _cx| {c.walk.height = height; Ok(())}},
            set_width(width: Size) {|c, _cx| {c.walk.width = width; Ok(())}},
            set_scroll(scroll: DVec2) {|c, _cx| {c.layout.scroll = scroll; Ok(())}},
            set_clip_x(clip: bool) {|c, _cx| {c.layout.clip_x = clip; Ok(())}},
            set_clip_y(clip: bool) {|c, _cx| {c.layout.clip_y = clip; Ok(())}},
            set_padding(padding: Padding) {|c, _cx| {c.layout.padding = padding; Ok(())}},
            set_align(align: Align) {|c, _cx| {c.layout.align = align; Ok(())}},
            set_flow(flow: Flow) {|c, _cx| {c.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64) {|c, _cx| {c.layout.spacing = spacing; Ok(())}},
            set_dpi_factor(factor: f64) {|c, _cx| {c.dpi_factor.replace(factor); Ok(())}},
            set_optimize(optimize: ViewOptimize) {|c, _cx| {c.optimize = optimize; Ok(())}},
            set_capture_overload(overload: bool) {|c, _cx| {c.capture_overload = overload; Ok(())}},
            set_event_key(event_key: bool) {|c, _cx| {c.event_key = event_key; Ok(())}}
        }
    }
    getter! {
        GView{
            get_theme(Themes) {|c| {c.theme}},
            get_background_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.background_color)}},
            get_shadow_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.shadow_color)}},
            get_hover_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.hover_color)}},
            get_focus_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.focus_color)}},
            get_border_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.border_color)}},
            get_border_width(f32) {|c| {c.draw_view.border_width}},
            get_border_radius(f32) {|c| {c.draw_view.border_radius}},
            get_shadow_offset(Vec2) {|c| {c.draw_view.shadow_offset}},
            get_spread_radius(f32) {|c| {c.draw_view.spread_radius}},
            get_blur_radius(f32) {|c| {c.draw_view.blur_radius}},
            get_background_visible(bool) {|c| {c.draw_view.background_visible.to_bool()}},
            get_visible(bool) {|c| {c.visible}},
            get_cursor(MouseCursor) {|c| {c.cursor.unwrap_or_default()}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_block_signal_event(bool) {|c| {c.block_signal_event}},
            get_abs_pos(DVec2) {|c| {c.walk.abs_pos.unwrap_or_default()}},
            get_margin(Margin) {|c| {c.walk.margin}},
            get_height(Size) {|c| {c.walk.height}},
            get_width(Size) {|c| {c.walk.width}},
            get_scroll(DVec2) {|c| {c.layout.scroll}},
            get_clip_x(bool) {|c| {c.layout.clip_x}},
            get_clip_y(bool) {|c| {c.layout.clip_y}},
            get_padding(Padding) {|c| {c.layout.padding}},
            get_align(Align) {|c| {c.layout.align}},
            get_flow(Flow) {|c| {c.layout.flow}},
            get_spacing(f64) {|c| {c.layout.spacing}},
            get_dpi_factor(f64) {|c| {c.dpi_factor.unwrap_or_default()}},
            get_optimize(ViewOptimize) {|c| {c.optimize}},
            get_capture_overload(bool) {|c| {c.capture_overload}},
            get_event_key(bool) {|c| {c.event_key}}
        }
    }
}

impl GViewRef {
    ref_getter_setter!{
        get_theme, set_theme -> Themes,
        get_background_color, set_background_color -> String,
        get_shadow_color, set_shadow_color -> String,
        get_hover_color, set_hover_color -> String,
        get_focus_color, set_focus_color -> String,
        get_border_color, set_border_color -> String,
        get_border_width, set_border_width -> f32,
        get_border_radius, set_border_radius -> f32,
        get_shadow_offset, set_shadow_offset -> Vec2,
        get_spread_radius, set_spread_radius -> f32,
        get_blur_radius, set_blur_radius -> f32,
        get_background_visible, set_background_visible -> bool,
        get_visible, set_visible -> bool,
        get_cursor, set_cursor -> MouseCursor,
        get_grab_key_focus, set_grab_key_focus -> bool,
        get_block_signal_event, set_block_signal_event -> bool,
        get_abs_pos, set_abs_pos -> DVec2,
        get_margin, set_margin -> Margin,
        get_height, set_height -> Size,
        get_width, set_width -> Size,
        get_scroll, set_scroll -> DVec2,
        get_clip_x, set_clip_x -> bool,
        get_clip_y, set_clip_y -> bool,
        get_padding, set_padding -> Padding,
        get_align, set_align -> Align,
        get_flow, set_flow -> Flow,
        get_spacing, set_spacing -> f64,
        get_dpi_factor, set_dpi_factor -> f64,
        get_optimize, set_optimize -> ViewOptimize,
        get_capture_overload, set_capture_overload -> bool,
        get_event_key, set_event_key -> bool

    }
    ref_event_option! {
        hover_in => GViewHoverParam,
        hover_over => GViewHoverParam,
        hover_out => GViewHoverParam,
        focus => GViewFocusParam,
        focus_lost => GViewFocusLostParam,
        clicked => GViewClickedParam,
        drag => GViewDragParam,
        key_down => GViewKeyEventParam,
        key_up => GViewKeyEventParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    // widget_origin_fn!(GView);
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    pub fn animator_cut(&self, cx: &mut Cx, state: &[LiveId; 2]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_cut(cx, state);
        }
    }

    pub fn animator_play(&self, cx: &mut Cx, state: &[LiveId; 2]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, state);
        }
    }
    pub fn toggle_state(
        &self,
        cx: &mut Cx,
        is_state_1: bool,
        animate: Animate,
        state1: &[LiveId; 2],
        state2: &[LiveId; 2],
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_toggle(cx, is_state_1, animate, state1, state2);
        }
    }
    pub fn set_texture(&self, slot: usize, texture: &Texture) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.draw_view.set_texture(slot, texture);
        }
    }

    pub fn set_uniform(&self, cx: &Cx, uniform: &[LiveId], value: &[f32]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.draw_view.set_uniform(cx, uniform, value);
        }
    }

    pub fn set_scroll_pos(&self, cx: &mut Cx, x: Option<f64>, y: Option<f64>) -> Option<bool> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_scroll_pos(cx, x, y)
        } else {
            None
        }
    }

    pub fn child_count(&self) -> usize {
        if let Some(inner) = self.borrow_mut() {
            inner.children.len()
        } else {
            0
        }
    }
}

impl GViewSet {
    pub fn animator_cut(&mut self, cx: &mut Cx, state: &[LiveId; 2]) {
        for item in self.iter() {
            item.animator_cut(cx, state)
        }
    }

    pub fn animator_play(&mut self, cx: &mut Cx, state: &[LiveId; 2]) {
        for item in self.iter() {
            item.animator_play(cx, state);
        }
    }
    pub fn toggle_state(
        &mut self,
        cx: &mut Cx,
        is_state_1: bool,
        animate: Animate,
        state1: &[LiveId; 2],
        state2: &[LiveId; 2],
    ) {
        for item in self.iter() {
            item.toggle_state(cx, is_state_1, animate, state1, state2);
        }
    }

    pub fn set_texture(&self, slot: usize, texture: &Texture) {
        for item in self.iter() {
            item.set_texture(slot, texture)
        }
    }

    pub fn set_uniform(&self, cx: &Cx, uniform: &[LiveId], value: &[f32]) {
        for item in self.iter() {
            item.set_uniform(cx, uniform, value)
        }
    }

    set_event! {
        hover_in => GViewHoverParam,
        hover_over => GViewHoverParam,
        hover_out => GViewHoverParam,
        focus => GViewFocusParam,
        focus_lost => GViewFocusLostParam,
        clicked => GViewClickedParam,
        drag => GViewDragParam,
        key_down => GViewKeyEventParam,
        key_up => GViewKeyEventParam
    }
}
