pub mod event;
pub mod register;

use event::*;
use makepad_widgets::*;

use crate::{
    event_option, prop_getter, prop_setter, ref_area, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path, shader::draw_loading::{DrawGLoading, GLoadingType}, themes::Themes, utils::{BoolToF32, ThemeColor}, widget_area
};

live_design! {
    link gen_base;

    pub GLoadingBase = {{GLoading}}{
        height: 48.0,
        width: 48.0,
    }
}

#[derive(Live, Widget)]
pub struct GLoading {
    #[live]
    pub theme: Themes,
    #[live]
    pub stroke_color: Option<Vec4>,
    // deref -------------------
    #[live]
    #[redraw]
    pub draw_loading: DrawGLoading,
    #[live]
    pub loading_type: GLoadingType,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // frame -------------------
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub animation_key: bool,
    #[live]
    pub time: f32,
    #[rust]
    next_frame: NextFrame,
    // store previous state(animation_key)
    #[rust]
    pub pre_state: bool,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GLoading {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        self.draw_loading.draw_walk(cx, walk);
        // redraw is important when changing visible or animation open state
        // self.redraw(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.animation_key || !self.visible {
            return;
        }
        if let Some(ne) = self.next_frame.is_event(event) {
            // update time to use for animation
            self.time = (ne.time * 0.001).fract() as f32;
            // force updates, so that we can animate in the absence of user-generated events
            self.redraw(cx);
            self.next_frame = cx.new_next_frame();
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GLoading {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.pre_state = self.animation_key;
        if !self.visible {
            return;
        }

        if let Err(e) = self.render(cx) {
            error!("GLoading render error: {:?}", e);
        }
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // starts the animation cycle on startup
        if self.animation_key {
            self.next_frame = cx.new_next_frame();
        }
    }

    fn after_update_from_doc(&mut self, cx: &mut Cx) {
        if self.pre_state != self.animation_key {
            let uid = self.widget_uid();
            if self.pre_state {
                cx.widget_action(
                    uid,
                    &Scope::empty().path,
                    GLoadingEvent::Closed(GLoadingEventParam),
                );
            } else {
                cx.widget_action(
                    uid,
                    &Scope::empty().path,
                    GLoadingEvent::Opened(GLoadingEventParam),
                );
            }
        }
    }
}

impl GLoading {
    set_scope_path!();
    widget_area! {
        area, draw_loading
    }
    event_option! {
        opened: GLoadingEvent::Opened => GLoadingEventParam,
        closed: GLoadingEvent::Closed => GLoadingEventParam
    }
    pub fn active_opened(&mut self, cx: &mut Cx) -> () {
        if self.event_key {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GLoadingEvent::Opened(GLoadingEventParam),
                );
            }
        }
    }
    pub fn active_closed(&mut self, cx: &mut Cx) -> () {
        if self.event_key {
            if let Some(path) = self.scope_path.as_ref() {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GLoadingEvent::Closed(GLoadingEventParam),
                );
            }
        }
    }
    pub fn open(&mut self, cx: &mut Cx) -> () {
        self.animation_key = true;
        self.draw_loading.opened = 1.0;
        self.redraw(cx);
    }
    pub fn close(&mut self, cx: &mut Cx) -> () {
        self.animation_key = false;
        self.draw_loading.opened = 0.0;
        self.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // ------------------ hover color -----------------------------------------------
        let loading_color = self.stroke_color.get(self.theme, 600);
        // ------------------ apply to draw_loading_wrap ----------------------------------------
        self.draw_loading.apply_over(
            cx,
            live! {
                stroke_color: (loading_color),
                opened: (self.animation_key.to_f32()),
            },
        );
        self.draw_loading.apply_type(self.loading_type.clone());
        Ok(())
    }
    pub fn redraw(&self, cx: &mut Cx) {
        self.draw_loading.redraw(cx);
    }
}

impl GLoadingRef {
    prop_setter! {
        GLoading{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme; Ok(())}},
            set_stroke_color(color: String) {|c_ref| {c_ref.stroke_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_loading_type(ty: GLoadingType) {|c_ref| {c_ref.loading_type = ty; Ok(())}},
            set_visible(visible: bool) {|c_ref| {c_ref.visible = visible; Ok(())}},
            set_animation_key(animation_key: bool) {|c_ref| {c_ref.animation_key = animation_key; Ok(())}},
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key; Ok(())}}
        }
    }
    prop_getter! {
        GLoading{
            get_theme(Themes) {|| Themes::default()}, {|c_ref| {c_ref.theme}},
            get_stroke_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_loading.stroke_color)}},
            get_loading_type(GLoadingType) {|| GLoadingType::default()}, {|c_ref| {c_ref.loading_type}},
            get_visible(bool) {|| Default::default()}, {|c_ref| {c_ref.visible}},
            get_animation_key(bool) {|| true}, {|c_ref| {c_ref.animation_key}},
            get_event_key(bool) {|| true}, {|c_ref| {c_ref.event_key}}
        }
    }
    ref_redraw!();
    ref_render!();
    ref_area!();
    ref_event_option! {
        opened => GLoadingEventParam,
        closed => GLoadingEventParam
    }
    pub fn open(&mut self, cx: &mut Cx) -> () {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.open(cx);
        }
    }
    pub fn close(&mut self, cx: &mut Cx) -> () {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.close(cx);
        }
    }
    /// ⚠️ This fn should be called when you need to encapsulate the new component
    pub fn active_opened(&mut self, cx: &mut Cx) -> () {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.active_opened(cx);
        }
    }
    /// ⚠️ This fn should be called when you need to encapsulate the new component
    pub fn active_closed(&mut self, cx: &mut Cx) -> () {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.active_closed(cx);
        }
    }
}

impl GLoadingSet {
    set_event! {
        opened => GLoadingEventParam,
        closed => GLoadingEventParam
    }
}
