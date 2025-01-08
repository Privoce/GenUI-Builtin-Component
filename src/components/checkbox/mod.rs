pub mod event;
mod group;
pub mod register;

use event::*;
pub use group::*;
use makepad_widgets::*;
use shader::draw_text::TextWrap;

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_hover_in,
    default_hit_hover_out, event_option, play_animation, prop_getter, prop_setter, ref_area,
    ref_area_ext, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path,
    set_text_and_visible_fn,
    shader::{
        draw_check_box::DrawGCheckbox, draw_radio::GChooseType, draw_text::DrawGText,
        draw_view::DrawGView,
    },
    themes::Themes,
    utils::{get_font_family, set_cursor, BoolToF32, ThemeColor, ToBool},
    widget_area,
};

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::GLOBAL_DURATION;

    pub GCheckboxBase = {{GCheckbox}}{
        height: Fit,
        width: Fit,
        font_size: 10.0,
        spacing: 8.0,
        align: {
            x: 0.0,
            y: 0.5
        },
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {hover: 0.0},
                        draw_checkbox_wrap: {hover: 0.0},
                        draw_text: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {hover: 1.0},
                        draw_checkbox_wrap: {hover: 1.0},
                        draw_text: {hover: 1.0}
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {selected: 0.0},
                        draw_checkbox_wrap: {focus: 0.0},
                        draw_text: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_checkbox: {selected: 1.0},
                        draw_checkbox_wrap: {focus: 1.0},
                        draw_text: {focus: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Widget, Live)]
pub struct GCheckbox {
    #[live]
    pub theme: Themes,
    // text ----------------------
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live(12.0)]
    pub font_size: f64,
    #[live(0.0)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live(true)]
    pub text_visible: bool,
    // checkbox ---------------------
    #[live(8.0)]
    pub size: f32,
    #[live]
    pub checkbox_background_color: Option<Vec4>,
    #[live(true)]
    pub checkbox_background_visible: bool,
    #[live]
    pub checkbox_hover_color: Option<Vec4>,
    #[live]
    pub checkbox_selected_color: Option<Vec4>,
    #[live]
    pub stroke_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_selected_color: Option<Vec4>,
    #[live]
    pub checkbox_border_color: Option<Vec4>,
    #[live(1.0)]
    pub checkbox_border_width: f32,
    #[live(0.48)]
    pub scale: f32,
    // checkbox_wrap ----------------
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live]
    pub background_visible: bool,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live(MouseCursor::Hand)]
    pub cursor: Option<MouseCursor>,
    // value -----------------------
    #[live(None)]
    pub value: Option<String>,
    // selected ------------------
    #[live(false)]
    pub selected: bool,
    #[live]
    pub text: ArcStringMut,
    // ---- type
    #[live]
    pub checkbox_type: GChooseType,
    // deref -------------------
    #[redraw]
    #[live]
    pub draw_checkbox: DrawGCheckbox,
    #[redraw]
    #[live]
    pub draw_text: DrawGText,
    #[redraw]
    #[live]
    pub draw_checkbox_wrap: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GCheckbox {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);

        self.draw_checkbox_wrap.begin(cx, walk, self.layout);
        let size = self.size + self.checkbox_border_width;
        let checkbox_walk = Walk {
            width: Size::Fixed((size * 2.0) as f64),
            height: Size::Fixed((size * 2.0) as f64),
            ..Default::default()
        };
        self.draw_checkbox.draw_walk(cx, checkbox_walk);

        if self.text_visible {
            let font = get_font_family(&self.font_family, cx);
            self.draw_text.text_style.font = font;
            let text_walk = Walk {
                width: Size::Fit,
                height: Size::Fit,
                ..Default::default()
            };
            self.draw_text
                .draw_walk(cx, text_walk, Align { x: 0.0, y: 0.0 }, self.text.as_ref());
        }

        self.draw_checkbox_wrap.end(cx);
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
    set_text_and_visible_fn!();
}

impl LiveHook for GCheckbox {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        self.render(cx);
    }
}

impl GCheckbox {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_checkbox_wrap,
        area_checkbox, draw_checkbox,
        area_text, draw_text
    }
    event_option! {
        clicked: GCheckboxEvent::Clicked => GCheckboxClickedParam,
        hover_in: GCheckboxEvent::HoverIn => GCheckboxHoverParam,
        hover_out: GCheckboxEvent::HoverOut => GCheckboxHoverParam
    }
    active_event! {
        active_hover_in: GCheckboxEvent::HoverIn |e: Option<FingerHoverEvent>| => GCheckboxHoverParam {e},
        active_hover_out: GCheckboxEvent::HoverOut |e: Option<FingerHoverEvent>| => GCheckboxHoverParam {e}
    }
    fn check_event_scope(&self) -> Option<&HeapLiveIdPath> {
        self.event_key.then(|| self.scope_path.as_ref()).flatten()
    }
    fn active_clicked(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GCheckboxEvent::Clicked(GCheckboxClickedParam {
                    value: self.value.clone(),
                    selected: self.selected,
                    e,
                }),
            );
        });
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn render(&mut self, cx: &mut Cx) -> () {
        // ----------------- checkbox -----------------------------------------------------
        let checkbox_background_color = self.checkbox_background_color.get(self.theme, 50);
        let checkbox_hover_color = self.checkbox_hover_color.get(self.theme, 100);
        let checkbox_selected_color = self.checkbox_selected_color.get(self.theme, 500);
        let checkbox_border_color = self.checkbox_border_color.get(self.theme, 600);
        let stroke_color = if self.checkbox_background_visible {
            self.stroke_color.get(self.theme, 50)
        } else {
            vec4(0.0, 0.0, 0.0, 0.0)
        };
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 50);
        let stroke_selected_color = self.stroke_selected_color.get(self.theme, 50);
        let checkbox_background_visible = self.checkbox_background_visible.to_f32();
        // ----------------- checkbox_wrap ------------------------------------------------
        let background_color = self.background_color.get(self.theme, 500);
        let hover_color = self.hover_color.get(self.theme, 400);
        let focus_color = self.focus_color.get(self.theme, 600);
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        // ----------------- text ------------------------------------------------------
        let color = self.color.get(self.theme, 50);
        let text_hover_color = self.text_hover_color.get(self.theme, 25);
        let text_focus_color = self.text_focus_color.get(self.theme, 100);
        // selected --------------------------------------------------------------------
        let selected = self.selected.to_f32();
        // ------------------ apply to draw_checkbox ---------------------------------------
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                background_color: (background_color),
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
        self.draw_checkbox.apply_over(
            cx,
            live! {
                background_color: (checkbox_background_color),
                background_visible: (checkbox_background_visible),
                hover_color: (checkbox_hover_color),
                selected_color: (checkbox_selected_color),
                stroke_color: (stroke_color),
                stroke_hover_color: (stroke_hover_color),
                stroke_selected_color: (stroke_selected_color),
                border_color: (checkbox_border_color),
                border_width: (self.checkbox_border_width),
                scale: (self.scale),
                size: (self.size),
                scale: (self.scale),
                selected: (selected)
            },
        );
        self.draw_checkbox.apply_type(self.checkbox_type.clone());
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    color: (color),
                    stroke_hover_color: (text_hover_color),
                    stroke_focus_color: (text_focus_color),
                    text_style: {
                        // top_drop: (self.top_drop),
                        font_size: (self.font_size),
                        height_factor: (self.height_factor),
                    }
                },
            );
            self.draw_text.wrap = self.wrap.clone();
        }
    }
    pub fn toggle(&mut self, cx: &mut Cx, selected: bool) -> () {
        self.selected = selected;
        self.draw_checkbox.selected = selected.to_f32();
        if selected {
            self.play_animation(cx, id!(selected.on));
        } else {
            self.play_animation(cx, id!(selected.off));
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    hover: 1.0,
                },
            );
        }
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_checkbox.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    hover: 0.0,
                },
            );
        }
    }
    pub fn animate_selected_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.selected = true;
        self.draw_checkbox.apply_over(
            cx,
            live! {
                selected: 1.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    focus: 1.0,
                },
            );
        }
    }
    pub fn animate_selected_off(&mut self, cx: &mut Cx) -> () {
        self.selected = false;
        self.draw_checkbox.apply_over(
            cx,
            live! {
                selected: 0.0,
            },
        );
        self.draw_checkbox_wrap.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        if self.text_visible {
            self.draw_text.apply_over(
                cx,
                live! {
                    focus: 0.0,
                },
            );
        }
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
        self.draw_checkbox.redraw(cx);
        self.draw_checkbox_wrap.redraw(cx);
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
                default_hit_hover_in!(self, cx, Some(e));
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, Some(e));
            }
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerUp(e) => {
                if self.selected {
                    self.play_animation(cx, id!(selected.on));
                }
                let state = if self.animator_in_state(cx, id!(selected.on)) {
                    self.selected = false;
                    id!(selected.off)
                } else {
                    self.selected = true;
                    id!(selected.on)
                };

                self.play_animation(cx, state);
                self.active_clicked(cx, Some(e));
            }
            _ => (),
        }
    }
    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }
    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

impl GCheckboxRef {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: bool) -> () {
        self.borrow_mut().map(|mut c_ref| c_ref.toggle(cx, selected));
    }
    prop_setter! {
        GCheckbox{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme;}},
            set_color(color: Vec4) {|c_ref| {c_ref.color.replace(color);}},
            set_text_hover_color(color: Vec4) {|c_ref| {c_ref.text_hover_color.replace(color);}},
            set_text_focus_color(color: Vec4) {|c_ref| {c_ref.text_focus_color.replace(color);}},
            set_font_size(font_size: f64) {|c_ref| {c_ref.font_size = font_size;}},
            set_height_factor(height_factor: f64) {|c_ref| {c_ref.height_factor = height_factor;}},
            set_wrap(wrap: TextWrap) {|c_ref| {c_ref.wrap = wrap;}},
            // set_font_family(font_family: LiveDependency) {|c_ref| {c_ref.font_family = font_family;}},
            set_text_visible(text_visible: bool) {|c_ref| {c_ref.text_visible = text_visible;}},
            set_size(size: f32) {|c_ref| {c_ref.size = size;}},
            set_checkbox_background_color(color: Vec4) {|c_ref| {c_ref.checkbox_background_color.replace(color);}},
            set_checkbox_background_visible(checkbox_background_visible: bool) {|c_ref| {c_ref.checkbox_background_visible = checkbox_background_visible;}},
            set_checkbox_hover_color(color: Vec4) {|c_ref| {c_ref.checkbox_hover_color.replace(color);}},
            set_checkbox_selected_color(color: Vec4) {|c_ref| {c_ref.checkbox_selected_color.replace(color);}},
            set_stroke_color(color: Vec4) {|c_ref| {c_ref.stroke_color.replace(color);}},
            set_stroke_hover_color(color: Vec4) {|c_ref| {c_ref.stroke_hover_color.replace(color);}},
            set_stroke_selected_color(color: Vec4) {|c_ref| {c_ref.stroke_selected_color.replace(color);}},
            set_checkbox_border_color(color: Vec4) {|c_ref| {c_ref.checkbox_border_color.replace(color);}},
            set_checkbox_border_width(border_width: f32) {|c_ref| {c_ref.checkbox_border_width = border_width;}},
            set_scale(scale: f32) {|c_ref| {c_ref.scale = scale;}},
            set_background_color(color: Vec4) {|c_ref| {c_ref.background_color.replace(color);}},
            set_hover_color(color: Vec4) {|c_ref| {c_ref.hover_color.replace(color);}},
            set_focus_color(color: Vec4) {|c_ref| {c_ref.focus_color.replace(color);}},
            set_shadow_color(color: Vec4) {|c_ref| {c_ref.shadow_color.replace(color);}},
            set_border_color(color: Vec4) {|c_ref| {c_ref.border_color.replace(color);}},
            set_background_visible(background_visible: bool) {|c_ref| {c_ref.background_visible = background_visible;}},
            set_border_width(border_width: f32) {|c_ref| {c_ref.border_width = border_width;}},
            set_border_radius(border_radius: f32) {|c_ref| {c_ref.border_radius = border_radius;}},
            set_spread_radius(spread_radius: f32) {|c_ref| {c_ref.spread_radius = spread_radius;}},
            set_blur_radius(blur_radius: f32) {|c_ref| {c_ref.blur_radius = blur_radius;}},
            set_shadow_offset(shadow_offset: Vec2) {|c_ref| {c_ref.shadow_offset = shadow_offset;}},
            set_cursor(cursor: MouseCursor) {|c_ref| {c_ref.cursor = Some(cursor);}},
            set_value(value: Option<String>) {|c_ref| {c_ref.value = value;}},
            set_checkbox_type(checkbox_type: GChooseType) {|c_ref| {c_ref.checkbox_type = checkbox_type;}},
            set_abs_pos(abs_pos: Option<DVec2>) {|c_ref| {c_ref.walk.abs_pos = abs_pos;}},
            set_margin(margin: Margin) {|c_ref| {c_ref.walk.margin = margin;}},
            set_height(height: Size) {|c_ref| {c_ref.walk.height = height;}},
            set_width(width: Size) {|c_ref| {c_ref.walk.width = width;}},
            set_scroll(scroll: DVec2) {|c_ref| {c_ref.layout.scroll = scroll;}},
            set_clip_x(clip_x: bool) {|c_ref| {c_ref.layout.clip_x = clip_x;}},
            set_clip_y(clip_y: bool) {|c_ref| {c_ref.layout.clip_y = clip_y;}},
            set_padding(padding: Padding) {|c_ref| {c_ref.layout.padding = padding;}},
            set_align(align: Align) {|c_ref| {c_ref.layout.align = align;}},
            set_flow(flow: Flow) {|c_ref| {c_ref.layout.flow = flow;}},
            set_spacing(spacing: f64) {|c_ref| {c_ref.layout.spacing = spacing;}},
            set_visible(visible: bool) {|c_ref| {c_ref.visible = visible;}},
            set_animation_key(animation_key: bool) {|c_ref| {c_ref.animation_key = animation_key;}},
            set_grab_key_focus(grab_key_focus: bool) {|c_ref| {c_ref.grab_key_focus = grab_key_focus;}},
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key;}}
        }
    }
    prop_getter! {
        GCheckbox{
            get_theme(Themes) {|| Themes::default()}, {|c_ref| {c_ref.theme}},
            get_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_text.color}},
            get_text_hover_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_text.stroke_hover_color}},
            get_text_focus_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_text.stroke_focus_color}},
            get_font_size(f64) {|| 12.0}, {|c_ref| {c_ref.draw_text.text_style.font_size}},
            get_height_factor(f64) {|| 0.0}, {|c_ref| {c_ref.draw_text.text_style.height_factor}},
            get_wrap(TextWrap) {|| TextWrap::Word}, {|c_ref| {c_ref.draw_text.wrap.clone()}},
            // get_font_family(LiveDependency) {|| LiveDependency::default()}, {|c_ref| {c_ref.draw_text.text_style.font}},
            get_text_visible(bool) {|| true}, {|c_ref| {c_ref.text_visible}},
            get_size(f32) {|| 8.0}, {|c_ref| {c_ref.draw_checkbox.size}},
            get_checkbox_background_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox.background_color}},
            get_checkbox_background_visible(bool) {|| true}, {|c_ref| {c_ref.draw_checkbox.background_visible.to_bool()}},
            get_checkbox_hover_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox.hover_color}},
            get_checkbox_selected_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox.selected_color}},
            get_stroke_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox.stroke_color}},
            get_stroke_hover_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox.stroke_hover_color}},
            get_stroke_selected_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox.stroke_selected_color}},
            get_checkbox_border_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox.border_color}},
            get_checkbox_border_width(f32) {|| 1.0}, {|c_ref| {c_ref.draw_checkbox.border_width}},
            get_scale(f32) {|| 0.48}, {|c_ref| {c_ref.draw_checkbox.scale}},
            get_background_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox_wrap.background_color}},
            get_hover_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox_wrap.hover_color}},
            get_focus_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox_wrap.focus_color}},
            get_shadow_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox_wrap.shadow_color}},
            get_border_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_checkbox_wrap.border_color}},
            get_background_visible(bool) {|| true}, {|c_ref| {c_ref.draw_checkbox_wrap.background_visible.to_bool()}},
            get_border_width(f32) {|| 0.0}, {|c_ref| {c_ref.draw_checkbox_wrap.border_width}},
            get_border_radius(f32) {|| 0.0}, {|c_ref| {c_ref.draw_checkbox_wrap.border_radius}},
            get_spread_radius(f32) {|| 0.0}, {|c_ref| {c_ref.draw_checkbox_wrap.spread_radius}},
            get_blur_radius(f32) {|| 4.8}, {|c_ref| {c_ref.draw_checkbox_wrap.blur_radius}},
            get_shadow_offset(Vec2) {|| Vec2::default()}, {|c_ref| {c_ref.draw_checkbox_wrap.shadow_offset}},
            get_cursor(MouseCursor) {|| MouseCursor::Hand}, {|c_ref| {c_ref.cursor.unwrap_or_default()}},
            get_value(Option<String>) {|| None}, {|c_ref| {c_ref.value.clone()}},
            get_checkbox_type(GChooseType) {|| GChooseType::default()}, {|c_ref| {c_ref.draw_checkbox.check_type.clone()}},
            get_selected(bool) {|| false}, {|c_ref| {c_ref.selected}},
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
            get_visible(bool) {||true}, {|c_ref| {c_ref.visible}},
            get_animation_key(bool) {||true}, {|c_ref| {c_ref.animation_key}},
            get_grab_key_focus(bool) {||true}, {|c_ref| {c_ref.grab_key_focus}},
            get_event_key(bool) {||true}, {|c_ref| {c_ref.event_key}}
        }
    }
    ref_area!();
    ref_area_ext! {
        area_checkbox,
        area_text
    }
    ref_redraw!();
    ref_render!();
    ref_event_option! {
        clicked => GCheckboxClickedParam,
        hover_in => GCheckboxHoverParam,
        hover_out => GCheckboxHoverParam
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_selected_on,
        animate_selected_off
    }
    /// ## Get the value of the checkbox.
    /// If the checkbox has a value, it will return the Some(value).
    /// Otherwise, it will return None.(include can not find the checkbox)
    pub fn value(&self) -> Option<String> {
        if let Some(c_ref) = self.borrow() {
            c_ref.value()
        } else {
            None
        }
    }
    /// ## Get the selected state of the checkbox.
    /// If the checkbox is selected, it will return true.
    /// Otherwise, it will return false.(include can not find the checkbox)
    #[deprecated(since = "0.1.0", note = "Please use get_selected() instead")]
    pub fn is_selected(&self) -> bool {
        if let Some(c_ref) = self.borrow() {
            c_ref.is_selected()
        } else {
            false
        }
    }
}

impl GCheckboxSet {
    set_event! {
        clicked => GCheckboxClickedParam,
        hover_in => GCheckboxHoverParam,
        hover_out => GCheckboxHoverParam
    }
}
