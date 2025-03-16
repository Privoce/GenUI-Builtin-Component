pub mod event;
pub mod register;

use event::{GToggleClickedParam, GToggleEvent, GToggleHoverParam};
use makepad_widgets::*;


use crate::{
    animatie_fn, default_handle_animation, default_hit_hover_in, default_hit_hover_out, event_option, play_animation, ref_getter, prop_setter, ref_area, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path, shader::draw_toggle::{DrawGToggle, GToggleType}, themes::Themes, utils::{set_cursor, BoolToF32, ThemeColor, ToBool}, widget_area
};

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::GLOBAL_DURATION;
    
    pub GToggleBase = {{GToggle}}{
        width: 36.0,
        height: 19.0,
        align: { x: 0.0, y: 0.0 }
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_toggle: {selected: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GToggle {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub selected_color: Option<Vec4>,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_selected_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(1.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live(0.64)]
    pub scale: f32,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    #[live(false)]
    pub selected: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live]
    pub toggle_type: GToggleType,
    // deref -------------------
    #[redraw]
    #[live]
    draw_toggle: DrawGToggle,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    animator: Animator,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GToggle {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        self.draw_toggle.draw_walk(cx, walk);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.is_visible() {
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, scope, hit, focus_area)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GToggle {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("GToggle render error: {:?}", e);
        }
    }
}

impl GToggle {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_toggle
    }
    event_option! {
        clicked: GToggleEvent::Clicked => GToggleClickedParam,
        hover_in: GToggleEvent::HoverIn => GToggleHoverParam,
        hover_out: GToggleEvent::HoverOut => GToggleHoverParam
    }
    fn check_event_scope(&self) -> Option<&HeapLiveIdPath> {
        self.event_key.then(|| self.scope_path.as_ref()).flatten()
    }
    pub fn active_hover_in(&mut self, cx: &mut Cx, e: FingerHoverEvent){
        self.check_event_scope().map(|path|{
            cx.widget_action(self.widget_uid(), path, GToggleEvent::HoverIn(GToggleHoverParam{
                selected: self.selected,
                e,
            }));
        });
    }
    pub fn active_hover_out(&mut self, cx: &mut Cx, e: FingerHoverEvent){
        self.check_event_scope().map(|path|{
            cx.widget_action(self.widget_uid(), path, GToggleEvent::HoverOut(GToggleHoverParam{
                selected: self.selected,
                e,
            }));
        });
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, e: FingerUpEvent){
        self.check_event_scope().map(|path|{
            cx.widget_action(self.widget_uid(), path, GToggleEvent::Clicked(GToggleClickedParam{
                selected: self.selected,
                e,
            }));
        });
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_toggle.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn redraw(&self, cx: &mut Cx) {
        self.draw_toggle.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ selected color ---------------------------------------------
        let selected_color = self.selected_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        // ------------------ stroke color ---------------------------------------------
        let stroke_color = self.stroke_color.get(self.theme, 50);
        // ------------------ stroke hover color ---------------------------------------
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 50);
        let stroke_selected_color = self.stroke_selected_color.get(self.theme, 50);
        // ------------------ apply to draw_toggle ----------------------------------------
        let selected = self.selected.to_f32();
        let background_visible = self.background_visible.to_f32();
        self.draw_toggle.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (background_visible)
                hover_color: (hover_color),
                selected_color: (selected_color),
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
                stroke_selected_color: (stroke_selected_color)
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                scale: (self.scale),
                selected: (selected)
            },
        );
        self.draw_toggle.apply_type(self.toggle_type.clone());
        Ok(())
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_toggle.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_toggle.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_selected_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.selected = true;
        self.draw_toggle.apply_over(
            cx,
            live! {
                selected: 1.0,
            },
        );
    }
    pub fn animate_selected_off(&mut self, cx: &mut Cx) -> () {
        self.selected = false;
        self.draw_toggle.apply_over(
            cx,
            live! {
                selected: 0.0,
            },
        );
    }
    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        default_handle_animation!(self, cx, event);

        match hit {
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, e);
            }
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerUp(e) => {
                // you need to add this line to ensure animation currently open if selected is true
                if self.selected {
                    self.animator_play(cx, id!(selected.on));
                }
                let state = if self.animator_in_state(cx, id!(selected.on)) {
                    self.selected = false;
                    id!(selected.off)
                } else {
                    self.selected = true;
                    id!(selected.on)
                };
                self.play_animation(cx, state);
                self.active_clicked(cx, e);
            }
            _ => (),
        }
    }
}

impl GToggleRef {
    prop_setter!{
        GToggle{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme; Ok(())}},
            set_background_color(color: String) {|c_ref| {c_ref.background_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_background_visible(visible: bool) {|c_ref| {c_ref.background_visible = visible; Ok(())}},
            set_hover_color(color: String) {|c_ref| {c_ref.hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_selected_color(color: String) {|c_ref| {c_ref.selected_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_stroke_color(color: String) {|c_ref| {c_ref.stroke_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_stroke_hover_color(color: String) {|c_ref| {c_ref.stroke_hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_stroke_selected_color(color: String) {|c_ref| {c_ref.stroke_selected_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_color(color: String) {|c_ref| {c_ref.border_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_width(width: f32) {|c_ref| {c_ref.border_width = width; Ok(())}},
            set_border_radius(radius: f32) {|c_ref| {c_ref.border_radius = radius; Ok(())}},
            set_scale(scale: f32) {|c_ref| {c_ref.scale = scale; Ok(())}},
            set_cursor(cursor: MouseCursor) {|c_ref| {c_ref.cursor = Some(cursor); Ok(())}},
            set_selected(selected: bool) {|c_ref| {c_ref.selected = selected; Ok(())}},
            set_grab_key_focus(grab_key_focus: bool) {|c_ref| {c_ref.grab_key_focus = grab_key_focus; Ok(())}},
            set_toggle_type(toggle_type: GToggleType) {|c_ref| {c_ref.toggle_type = toggle_type; Ok(())}},
            set_visible(visible: bool) {|c_ref| {c_ref.visible = visible; Ok(())}},
            set_animation_key(animation_key: bool) {|c_ref| {c_ref.animation_key = animation_key; Ok(())}},
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key; Ok(())}}
        }
    }
    ref_getter!{
        GToggle{
            get_theme(Themes) {|| Default::default()}, {|c_ref| {c_ref.theme}},
            get_background_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_toggle.background_color)}},
            get_background_visible(bool) {|| Default::default()}, {|c_ref| {c_ref.draw_toggle.background_visible.to_bool()}},
            get_hover_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_toggle.hover_color)}},
            get_selected_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_toggle.selected_color)}},
            get_stroke_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_toggle.stroke_color)}},
            get_stroke_hover_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_toggle.stroke_hover_color)}},
            get_stroke_selected_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_toggle.stroke_selected_color)}},
            get_border_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_toggle.border_color)}},
            get_border_width(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_toggle.border_width}},
            get_border_radius(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_toggle.border_radius}},
            get_scale(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_toggle.scale}},
            get_cursor(MouseCursor) {|| Default::default()}, {|c_ref| {c_ref.cursor.unwrap_or(MouseCursor::Hand)}},
            get_selected(bool) {|| Default::default()}, {|c_ref| {c_ref.selected}},
            get_grab_key_focus(bool) {|| Default::default()}, {|c_ref| {c_ref.grab_key_focus}},
            get_toggle_type(GToggleType) {|| Default::default()}, {|c_ref| {c_ref.toggle_type.clone()}},
            get_visible(bool) {|| true}, {|c_ref| {c_ref.visible}},
            get_animation_key(bool) {|| true}, {|c_ref| {c_ref.animation_key}},
            get_event_key(bool) {|| true}, {|c_ref| {c_ref.event_key}}
        }
    }
    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_event_option! {
        clicked => GToggleClickedParam,
        hover_in => GToggleHoverParam,
        hover_out => GToggleHoverParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_selected_on,
        animate_selected_off
    }
}

impl GToggleSet {
    set_event! {
        clicked => GToggleClickedParam,
        hover_in => GToggleHoverParam,
        hover_out => GToggleHoverParam
    }
}