pub mod event;
pub mod register;
mod types;

use event::*;

use types::*;

use makepad_widgets::*;

use crate::{
    animatie_fn, event_option, getter, ref_event_option, ref_getter_setter, ref_redraw, ref_render, set_event, setter, shader::{draw_view::DrawGView, manual::Position4}, utils::{set_cursor, BoolToF32}, widget_area
};

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::GLOBAL_DURATION;

    pub GCollapseBase = {{GCollapse}}{
        height: Fit,
        width: Fill,
        flow: Down,
        opened: false,
        animator: {
            open = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    redraw: true
                    apply: {
                        fold: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    redraw: true
                    apply: {
                        fold: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GCollapse {
    #[live]
    #[redraw]
    #[find]
    pub header: WidgetRef,
    #[live]
    #[redraw]
    #[find]
    pub body: WidgetRef,
    #[redraw]
    #[live]
    pub draw_collapse: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[rust]
    pub rect_size: f64,
    #[rust]
    pub area: Area,
    #[live(false)]
    pub opened: bool,
    #[live]
    fold: f64,
    #[rust]
    pub draw_state: DrawStateWrap<DrawCollapseState>,
    #[live(Some(MouseCursor::Hand))]
    pub cursor: Option<MouseCursor>,
    #[live(true)]
    pub grab_key_focus: bool,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(false)]
    pub animation_key: bool,
    #[animator]
    animator: Animator,
    // use animation counter to prevent multiple animations
    #[rust(true)]
    animation_counter: bool,
    #[live]
    pub position: Position4,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GCollapse {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.fold = self.opened.to_f32() as f64;
        let body_walk = self.body.walk(cx);
        let header_walk = self.header.walk(cx);
        let (flow, steps) = match self.position {
            Position4::Left => (
                Flow::Right,
                [DrawCollapseState::DrawBody, DrawCollapseState::DrawHeader],
            ),
            Position4::Right => (
                Flow::Right,
                [DrawCollapseState::DrawHeader, DrawCollapseState::DrawBody],
            ),
            Position4::Top => (
                Flow::Down,
                [DrawCollapseState::DrawBody, DrawCollapseState::DrawHeader],
            ),
            Position4::Bottom => (
                Flow::Down,
                [DrawCollapseState::DrawHeader, DrawCollapseState::DrawBody],
            ),
        };

        self.layout.flow = flow;
        if self.draw_state.begin(cx, steps[0]) {
            if !self.opened {
                match self.position {
                    Position4::Left | Position4::Right => {
                        if !walk.width.is_fixed() {
                            walk.width = header_walk.width;
                        }
                    }
                    Position4::Top | Position4::Bottom => {
                        if !walk.height.is_fixed() {
                            walk.height = header_walk.height;
                        }
                    }
                }

                cx.begin_turtle(walk, self.layout);
            } else {
                match self.position {
                    Position4::Left | Position4::Right => {
                        if !walk.width.is_fixed() {
                            walk.width = Size::Fill;
                        }
                    }
                    Position4::Top | Position4::Bottom => {
                        if !walk.height.is_fixed() {
                            walk.height = Size::Fill;
                        }
                    }
                }

                // // if is opened, walk should be Fill
                // let walk = if walk.height.is_fixed() {
                //     walk
                // } else {
                //     Walk::fill()
                // };
                cx.begin_turtle(walk, self.layout);
            }
        }

        for (index, _) in steps.iter().enumerate() {
            let _ = self.draw_state.get().map(|state| match state {
                DrawCollapseState::DrawHeader => {
                    let _ = self.header.draw_walk(cx, scope, header_walk);
                    // check is the first step
                    if index == 0 {
                        cx.begin_turtle(
                            body_walk,
                            Layout::flow_down()
                                .with_scroll(dvec2(0.0, self.rect_size * (1.0 - self.fold))),
                        );
                        self.draw_state.set(steps[1]);
                    } else {
                        match self.position {
                            Position4::Left | Position4::Right => {
                                self.rect_size = cx.turtle().used().x;
                            }
                            Position4::Top | Position4::Bottom => {
                                self.rect_size = cx.turtle().used().y;
                            }
                        }
                        cx.end_turtle();
                        cx.end_turtle_with_area(&mut self.area);
                        self.draw_state.end();
                    }
                }
                DrawCollapseState::DrawBody => {
                    if self.fold == 1.0 {
                        self.animator_play(cx, id!(open.on));
                        let _ = self.body.draw_walk(cx, scope, body_walk);
                    }
                    // check is the last step
                    if index == 1 {
                        match self.position {
                            Position4::Left | Position4::Right => {
                                self.rect_size = cx.turtle().used().x;
                            }
                            Position4::Top | Position4::Bottom => {
                                self.rect_size = cx.turtle().used().y;
                            }
                        }
                        cx.end_turtle();
                        cx.end_turtle_with_area(&mut self.area);
                        self.draw_state.end();
                    } else {
                        cx.begin_turtle(header_walk, Layout::flow_down());
                        self.draw_state.set(steps[1]);
                    }
                }
            });
        }
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        let uid = self.widget_uid();
        if !self.animation_key && self.animation_counter {
            if self.animator_handle_event(cx, event).must_redraw() {
                if self.animator.is_track_animating(cx, id!(open)) {
                    self.area.redraw(cx);
                    self.animation_counter = !self.animation_counter;
                }
            }
        }

        match event.hits(cx, self.area_header()) {
            Hit::FingerDown(_, _) => {
                if self.grab_key_focus {
                    cx.set_key_focus(sweep_area);
                }
            }
            Hit::FingerHoverIn(f_in, _) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(uid, &scope.path, GCollapseEvent::Hover(f_in.clone()));
            }
            Hit::FingerHoverOut(_) => {
                let _ = set_cursor(cx, Some(&MouseCursor::Arrow));
            }
            Hit::FingerUp(f_up) => {
                self.opened = !self.opened;
                self.fold = self.opened.to_f32() as f64;

                if self.opened {
                    self.animator_play(cx, id!(open.on));
                    cx.widget_action(uid, &scope.path, GCollapseEvent::Opened(f_up.clone()));
                } else {
                    self.animator_play(cx, id!(open.off));
                    cx.widget_action(uid, &scope.path, GCollapseEvent::Closed(f_up.clone()));
                }
                self.animation_counter = !self.animation_counter;
            }
            _ => {}
        }

        if self.opened {
            self.body.handle_event(cx, event, scope);
        }
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if !self.animation_key && self.animation_counter {
            if self.animator_handle_event(cx, event).must_redraw() {
                if self.animator.is_track_animating(cx, id!(open)) {
                    self.area.redraw(cx);
                    self.animation_counter = !self.animation_counter;
                }
            }
        }

        match event.hits(cx, self.area_header()) {
            Hit::FingerDown(_, _) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.area());
                }
            }
            Hit::FingerHoverIn(f_in, _) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                cx.widget_action(uid, &scope.path, GCollapseEvent::Hover(f_in.clone()));
            }
            Hit::FingerHoverOut(_) => {
                let _ = set_cursor(cx, Some(&MouseCursor::Arrow));
            }
            Hit::FingerUp(f_up) => {
                self.opened = !self.opened;
                self.fold = self.opened.to_f32() as f64;

                if self.opened {
                    self.animator_play(cx, id!(open.on));
                    cx.widget_action(uid, &scope.path, GCollapseEvent::Opened(f_up.clone()));
                } else {
                    self.animator_play(cx, id!(open.off));
                    cx.widget_action(uid, &scope.path, GCollapseEvent::Closed(f_up.clone()));
                }
                self.animation_counter = true;
            }
            _ => {}
        }

        if self.opened {
            self.body.handle_event(cx, event, scope);
        }

        // self.header.handle_event(cx, event, scope);
        // if let Event::Actions(actions) = event {
        //     match actions
        //         .find_widget_action(self.header.widget(id!(fold_button)).widget_uid())
        //         .cast()
        //     {
        //         FoldButtonAction::Opening => self.animator_play(cx, id!(open.on)),
        //         FoldButtonAction::Closing => self.animator_play(cx, id!(open.off)),
        //         _ => (),
        //     }
        // }
    }
}

impl LiveHook for GCollapse {}

impl GCollapse {
    widget_area! {
        area, area,
        area_header, header,
        area_body, body
    }
    event_option! {
        opened: GCollapseEvent::Opened => FingerUpEvent,
        closed: GCollapseEvent::Closed => FingerUpEvent,
        hover: GCollapseEvent::Hover => FingerHoverEvent
    }
    pub fn animate_open_on(&mut self, cx: &mut Cx) -> () {
        self.opened = true;
        self.fold = 1.0;
        self.animator_play(cx, id!(open.on));
        self.animation_counter = true;
    }
    pub fn animate_open_off(&mut self, cx: &mut Cx) -> () {
        self.opened = false;
        self.fold = 0.0;
        self.animator_play(cx, id!(open.off));
        self.animation_counter = true;
    }
    pub fn render(&mut self, _cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.header.redraw(cx);
        self.body.redraw(cx);
        self.draw_collapse.redraw(cx);
    }
    setter! {
        GCollapse{
            set_abs_pos(pos: Option<DVec2>){|c, _cx| {c.walk.abs_pos = pos; Ok(())}},
            set_margin(margin: Margin){|c, _cx| {c.walk.margin = margin; Ok(())}},
            set_height(height: Size){|c, _cx| {c.walk.height = height; Ok(())}},
            set_width(width: Size){|c, _cx| {c.walk.width = width; Ok(())}},
            set_scroll(scroll: DVec2){|c, _cx| {c.layout.scroll = scroll; Ok(())}},
            set_clip_x(clip: bool){|c, _cx| {c.layout.clip_x = clip; Ok(())}},
            set_clip_y(clip: bool){|c, _cx| {c.layout.clip_y = clip; Ok(())}},
            set_padding(padding: Padding){|c, _cx| {c.layout.padding = padding; Ok(())}},
            set_align(align: Align){|c, _cx| {c.layout.align = align; Ok(())}},
            set_flow(flow: Flow){|c, _cx| {c.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64){|c, _cx| {c.layout.spacing = spacing; Ok(())}},
            set_fold(fold: f64){|c, _cx| {c.fold = fold; Ok(())}},
            set_cursor(cursor: MouseCursor){|c, _cx| {c.cursor.replace(cursor); Ok(())}},
            set_grab_key_focus(grab_key_focus: bool){|c, _cx| {c.grab_key_focus = grab_key_focus; Ok(())}},
            set_visible(visible: bool){|c, _cx| {c.visible = visible; Ok(())}},
            set_animation_key(animation_key: bool){|c, _cx| {c.animation_key = animation_key; Ok(())}},
            set_position(position: Position4){|c, _cx| {c.position = position; Ok(())}},
            set_event_key(event_key: bool){|c, _cx| {c.event_key = event_key; Ok(())}}
        }
    }
    getter! {
        GCollapse{
            get_abs_pos(Option<DVec2>) {|c| {c.walk.abs_pos}},
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
            get_fold(f64) {|c| {c.fold}},
            get_cursor(MouseCursor) {|c| {c.cursor.unwrap_or_default()}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_visible(bool) {|c| {c.visible}},
            get_animation_key(bool) {|c| {c.animation_key}},
            get_position(Position4) {|c| {c.position}},
            get_event_key(bool) {|c| {c.event_key}}
        }
    }
}

impl GCollapseRef {
    ref_getter_setter! {
        get_abs_pos, set_abs_pos -> Option<DVec2>,
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
        get_fold, set_fold -> f64,
        get_cursor, set_cursor -> MouseCursor,
        get_grab_key_focus, set_grab_key_focus -> bool,
        get_visible, set_visible -> bool,
        get_animation_key, set_animation_key -> bool,
        get_position, set_position -> Position4,
        get_event_key, set_event_key -> bool
    }
    ref_event_option! {
        opened => FingerUpEvent,
        closed => FingerUpEvent,
        hover => FingerHoverEvent
    }
    animatie_fn! {
        animate_open_on,
        animate_open_off
    }
    ref_render!();
    // ref_redraw_mut!();
    ref_redraw!();
}

impl GCollapseSet {
    set_event! {
        opened => FingerUpEvent,
        closed => FingerUpEvent,
        hover => FingerHoverEvent
    }
}
