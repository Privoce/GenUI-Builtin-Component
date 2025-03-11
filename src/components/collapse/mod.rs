pub mod event;
pub mod register;
mod types;

use event::*;

use types::*;

use makepad_widgets::*;

use crate::{
    animatie_fn, event_option, prop_getter, prop_setter, ref_event_option, ref_render, set_event, shader::{draw_view::DrawGView, manual::Position4}, utils::{set_cursor, BoolToF32}, widget_area
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
    pub fn render(&mut self, _cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>>{
        Ok(())
    }
}

impl GCollapseRef {
    prop_setter!{
        GCollapse{
            set_abs_pos(pos: DVec2){|c_ref| {c_ref.walk.abs_pos.replace(pos); Ok(())}},
            set_margin(margin: Margin){|c_ref| {c_ref.walk.margin = margin; Ok(())}},
            set_height(height: Size){|c_ref| {c_ref.walk.height = height; Ok(())}},
            set_width(width: Size){|c_ref| {c_ref.walk.width = width; Ok(())}},
            set_scroll(scroll: DVec2){|c_ref| {c_ref.layout.scroll = scroll; Ok(())}},
            set_clip_x(clip: bool){|c_ref| {c_ref.layout.clip_x = clip; Ok(())}},
            set_clip_y(clip: bool){|c_ref| {c_ref.layout.clip_y = clip; Ok(())}},
            set_padding(padding: Padding){|c_ref| {c_ref.layout.padding = padding; Ok(())}},
            set_align(align: Align){|c_ref| {c_ref.layout.align = align; Ok(())}},
            set_flow(flow: Flow){|c_ref| {c_ref.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64){|c_ref| {c_ref.layout.spacing = spacing; Ok(())}},
            set_fold(fold: f64){|c_ref| {c_ref.fold = fold; Ok(())}},
            set_cursor(cursor: MouseCursor){|c_ref| {c_ref.cursor.replace(cursor); Ok(())}},
            set_grab_key_focus(grab_key_focus: bool){|c_ref| {c_ref.grab_key_focus = grab_key_focus; Ok(())}},
            set_visible(visible: bool){|c_ref| {c_ref.visible = visible; Ok(())}},
            set_animation_key(animation_key: bool){|c_ref| {c_ref.animation_key = animation_key; Ok(())}},
            set_position(position: Position4){|c_ref| {c_ref.position = position; Ok(())}},
            set_event_key(event_key: bool){|c_ref| {c_ref.event_key = event_key; Ok(())}}
        }
    }
    prop_getter!{
        GCollapse{
            get_abs_pos(Option<DVec2>) {||None}, {|c_ref| {c_ref.walk.abs_pos}},
            get_margin(Margin) {||Margin::default()}, {|c_ref| {c_ref.walk.margin}},
            get_height(Size) {||Size::default()}, {|c_ref| {c_ref.walk.height}},
            get_width(Size) {||Size::default()}, {|c_ref| {c_ref.walk.width}},
            get_scroll(DVec2) {||DVec2::default()}, {|c_ref| {c_ref.layout.scroll}},
            get_clip_x(bool) {||true}, {|c_ref| {c_ref.layout.clip_x}},
            get_clip_y(bool) {||true}, {|c_ref| {c_ref.layout.clip_y}},
            get_padding(Padding) {||Padding::default()}, {|c_ref| {c_ref.layout.padding}},
            get_align(Align) {||Align::default()}, {|c_ref| {c_ref.layout.align}},
            get_flow(Flow) {||Flow::default()}, {|c_ref| {c_ref.layout.flow}},
            get_spacing(f64) {||0.0}, {|c_ref| {c_ref.layout.spacing}},
            get_fold(f64) {||0.0}, {|c_ref| {c_ref.fold}},
            get_cursor(MouseCursor) {|| Default::default()}, {|c_ref| {c_ref.cursor.unwrap_or_default()}},
            get_grab_key_focus(bool) {||true}, {|c_ref| {c_ref.grab_key_focus}},
            get_visible(bool) {||true}, {|c_ref| {c_ref.visible}},
            get_animation_key(bool) {||false}, {|c_ref| {c_ref.animation_key}},
            get_position(Position4) {||Position4::default()}, {|c_ref| {c_ref.position}},
            get_event_key(bool) {||true}, {|c_ref| {c_ref.event_key}}
        }
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
}

impl GCollapseSet {
    set_event! {
        opened => FingerUpEvent,
        closed => FingerUpEvent,
        hover => FingerHoverEvent
    }
}
