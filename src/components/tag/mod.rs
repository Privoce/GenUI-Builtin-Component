mod event;
pub mod register;

pub use event::*;

use crate::shader::draw_icon_pixel::DrawGIconPixel;
use crate::shader::draw_svg::DrawGSvg;
use crate::shader::draw_text::DrawGText;
use crate::utils::{get_font_family, set_cursor, BoolToF32, RectExp, ThemeColor, ToBool};
use crate::{
    active_event, animatie_fn, check_event_scope, default_handle_animation,
    default_hit_finger_down, default_hit_hover_in, default_hit_hover_out, event_option,
    play_animation, prop_getter, prop_setter, ref_area, ref_area_ext, ref_event_option, ref_redraw,
    ref_render, set_event, set_scope_path, set_text_and_visible_fn, widget_area,
};
use crate::{shader::draw_view::DrawGView, themes::Themes};
use makepad_widgets::*;

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::*;

    pub GTagBase = {{GTag}}{
        spacing: 4.6,
        theme: Primary,
        text: "",
        font_family: (FONT_FAMILY),
        padding: <GLOBAL_PADDING_SMALL>{},
        font_size: (FONT_SIZE_SMALL),
        align: <ALIGN_CENTER_WALK>{},
        clip_x: false,
        clip_y: false,
        shadow_offset: vec2(0.0, 2.0),
        height: Fit,
        width: Fit,
        text_walk: {
            height: Fit,
            width: Fit,
        }
        cursor: Default,
        icon_walk: {
            margin: 0,
        },
        icon_layout: {
            padding: 0,
        },
        draw_close: {
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.move_to(self.pos.x + 0.5, self.pos.y + 0.5);
                sdf.line_to(self.rect_size.x - 0.5, self.rect_size.y - 0.5);
                sdf.move_to(self.rect_size.x - 0.5, self.pos.y - 0.5);
                sdf.line_to(self.pos.x + 0.5, self.rect_size.y - 0.5);
                sdf.stroke(self.color, 1.2);
                return sdf.result;
            }
        },
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_tag: {hover: 0.0, focus: 0.0},
                        draw_icon: {hover: 0.0, focus: 0.0},
                        draw_text: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_tag: {hover: 1.0, focus: 0.0},
                        draw_icon: {hover: 1.0, focus: 0.0},
                        draw_text: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_tag: {hover: 0.0, focus: 1.0},
                        draw_icon: {hover: 0.0, focus: 1.0},
                        draw_text: {hover: 0.0, focus: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GTag {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    // text ----------------------------
    #[live]
    pub text: ArcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    // #[live(1.1)]
    // pub top_drop: f64,
    #[live(1.3)]
    pub height_factor: f64,
    #[live(0.88)]
    pub line_scale: f64,
    // icon ----------------------------
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live]
    pub closeable: bool,
    #[live]
    pub src: LiveDependency,
    #[live(1.0)]
    pub icon_brightness: f32,
    #[live(0.6)]
    pub icon_curve: f32,
    #[live(0.5)]
    pub icon_linearize: f32,
    #[live(1.0)]
    pub icon_scale: f64,
    #[live]
    pub icon_color: Option<Vec4>,
    #[live(1.0)]
    pub icon_draw_depth: f32,
    // visible -------------------------
    #[live(true)]
    pub visible: bool,
    // define area ---------------------
    #[live]
    draw_text: DrawGText,
    #[live]
    text_walk: Walk,
    #[live(true)]
    grab_key_focus: bool,
    #[live]
    draw_icon: DrawGSvg,
    #[live]
    draw_close: DrawGIconPixel,
    #[live]
    icon_walk: Walk,
    #[live]
    icon_layout: Layout,
    // deref -----------------
    #[redraw]
    #[live]
    draw_tag: DrawGView,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
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

impl Widget for GTag {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.set_scope_path(&scope.path);
        let _ = get_font_family(&self.font_family, cx, &mut self.draw_text.text_style.font);
        self.icon_walk.height = Size::Fixed(self.font_size);
        self.icon_walk.width = Size::Fixed(self.font_size);
        // self.text_walk.margin.top = self.font_size / 4.0;
        let _ = self.draw_tag.begin(cx, walk, self.layout);
        let _ = self.draw_icon.draw_walk(cx, self.icon_walk);

        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());
        if self.closeable {
            let _ = self.draw_close.draw_walk(cx, self.icon_walk);
        }
        self.draw_tag.end(cx);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::default().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, event, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, event, hit, focus_area)
    }
    set_text_and_visible_fn!();
}

impl LiveHook for GTag {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("GTag render error: {:?}", e);
        }
    }
}

impl GTag {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_tag,
        area_icon, draw_icon,
        area_text, draw_text
    }
    pub fn area_close(&self) -> Area {
        if self.closeable {
            return self.draw_close.area;
        }
        return Area::Empty;
    }
    check_event_scope!();
    active_event! {
        active_hover_in: GTagEvent::HoverIn |e: Option<FingerHoverEvent>| => GTagHoverParam {e},
        active_hover_out: GTagEvent::HoverOut |e: Option<FingerHoverEvent>| => GTagHoverParam {e},
        active_focus: GTagEvent::Focus |e: Option<FingerDownEvent>| => GTagFocusParam {e},
        active_focus_lost: GTagEvent::FocusLost |e: Option<FingerUpEvent>| => GTagFocusLostParam {e}
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GTagEvent::Clicked(GTagClickedParam {
                    text: self.text.as_ref().to_string(),
                    e,
                }),
            );
        });
    }
    pub fn active_closed(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        self.check_event_scope().map(|path| {
            cx.widget_action(
                self.widget_uid(),
                path,
                GTagEvent::Closed(GTagClosedParam {
                    text: self.text.as_ref().to_string(),
                    e,
                }),
            );
        });
    }
    event_option! {
        hover_in: GTagEvent::HoverIn => GTagHoverParam,
        hover_out: GTagEvent::HoverOut => GTagHoverParam,
        focus: GTagEvent::Focus => GTagFocusParam,
        focus_lost: GTagEvent::FocusLost => GTagFocusLostParam,
        clicked: GTagEvent::Clicked => GTagClickedParam,
        closed: GTagEvent::Closed => GTagClosedParam
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_tag.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        // if self.closeable{
        //     self.draw_close.apply_over(cx, live!{
        //         hover: 0.0,
        //         focus: 0.0
        //     });
        // }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_tag.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_tag.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_tag.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_tag.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        self.draw_icon.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 0.0,
            },
        );
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
        self.draw_icon.redraw(cx);
        self.draw_close.redraw(cx);
        self.draw_tag.redraw(cx);
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
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ text ------------------------------------------------------
        let text_color = self.color.get(self.theme, 50);
        let text_hover_color = self.text_hover_color.get(self.theme, 25);
        let text_focus_color = self.text_focus_color.get(self.theme, 100);
        // ------------------icon color -----------------------------------------------
        let icon_color = self.icon_color.get(self.theme, 50);
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 25);
        let stroke_focus_color = self.stroke_focus_color.get(self.theme, 100);
        let background_visible = self.background_visible.to_f32();
        self.draw_tag.apply_over(
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
        self.draw_icon.apply_over(
            cx,
            live! {
                stroke_hover_color: (stroke_hover_color),
                stroke_focus_color: (stroke_focus_color),
                color: (icon_color),
                brightness: (self.icon_brightness),
                curve: (self.icon_curve),
                linearize: (self.icon_linearize),
                scale: (self.icon_scale),
                draw_depth: (self.icon_draw_depth),
            },
        );

        self.draw_icon.set_src(self.src.clone());
        self.draw_text.apply_over(
            cx,
            live! {
                color: (text_color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
                text_style: {
                    font_size: (self.font_size),
                    // brightness: (default_text_style.brightness),
                    // curve: (default_text_style.curve),
                    // line_spacing: (self.line_spacing),
                    line_scale: (self.line_scale)
                    // top_drop: (self.top_drop),
                    height_factor: (self.height_factor),
                },
            },
        );

        if self.closeable {
            self.draw_close.apply_over(
                cx,
                live! {
                    brightness: (self.icon_brightness),
                    color: (icon_color),
                    curve: (self.icon_curve),
                    draw_depth: (self.icon_draw_depth),
                    linearize: (self.icon_linearize),
                },
            );
        }
        Ok(())
    }
    pub fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, focus_area: Area) {
        default_handle_animation!(self, cx, event);
        match hit {
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, Some(e));
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, Some(e));
            }
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, focus_area, Some(e));
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }
                    // use is in to judge
                    if self.closeable {
                        if self.area_close().rect(cx).is_in_pos(&e.abs) {
                            self.active_closed(cx, Some(e.clone()));
                            return;
                        }
                    }

                    self.active_clicked(cx, Some(e));
                } else {
                    self.play_animation(cx, id!(hover.off));
                    self.active_focus_lost(cx, Some(e));
                }
            }
            _ => (),
        }
    }
    pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.visible = visible;
        if visible {
            self.clear_animation(cx);
            self.redraw(cx);
        }
    }
}

impl GTagRef {
    pub fn set_text(&self, cx: &mut Cx, text: String) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.set_text(cx, &text);
        }
    }
    prop_setter! {
        GTag{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme; Ok(())}},
            set_background_color(color: String) {|c_ref| {c_ref.background_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_hover_color(color: String) {|c_ref| {c_ref.hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_focus_color(color: String) {|c_ref| {c_ref.focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_color(color: String) {|c_ref| {c_ref.border_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_shadow_color(color: String) {|c_ref| {c_ref.shadow_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_text_color(color: String) {|c_ref| {c_ref.color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_icon_color(color: String) {|c_ref| {c_ref.icon_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_background_visible(visible: bool) {|c_ref| {c_ref.background_visible = visible; Ok(())}},
            set_border_width(width: f32) {|c_ref| {c_ref.border_width = width; Ok(())}},
            set_border_radius(radius: f32) {|c_ref| {c_ref.border_radius = radius; Ok(())}},
            set_spread_radius(radius: f32) {|c_ref| {c_ref.spread_radius = radius; Ok(())}},
            set_blur_radius(radius: f32) {|c_ref| {c_ref.blur_radius = radius; Ok(())}},
            set_shadow_offset(offset: Vec2) {|c_ref| {c_ref.shadow_offset = offset; Ok(())}},
            set_font_size(size: f64) {|c_ref| {c_ref.font_size = size; Ok(())}},
            set_height_factor(factor: f64) {|c_ref| {c_ref.height_factor = factor; Ok(())}},
            set_line_scale(scale: f64) {|c_ref| {c_ref.line_scale = scale; Ok(())}},
            set_icon_brightness(brightness: f32) {|c_ref| {c_ref.icon_brightness = brightness; Ok(())}},
            set_icon_curve(curve: f32) {|c_ref| {c_ref.icon_curve = curve; Ok(())}},
            set_icon_linearize(linearize: f32) {|c_ref| {c_ref.icon_linearize = linearize; Ok(())}},
            set_icon_scale(scale: f64) {|c_ref| {c_ref.icon_scale = scale; Ok(())}},
            set_icon_draw_depth(depth: f32) {|c_ref| {c_ref.icon_draw_depth = depth; Ok(())}},
            set_cursor(cursor: MouseCursor) {|c_ref| {c_ref.cursor.replace(cursor); Ok(())}},
            set_closeable(closeable: bool) {|c_ref| {c_ref.closeable = closeable; Ok(())}},
            set_text_height(height: Size) {|c_ref| {c_ref.text_walk.height = height; Ok(())}},
            set_text_width(width: Size) {|c_ref| {c_ref.text_walk.width = width; Ok(())}},
            set_text_margin(margin: Margin) {|c_ref| {c_ref.text_walk.margin = margin; Ok(())}},
            set_text_abs_pos(pos: DVec2) {|c_ref| {c_ref.text_walk.abs_pos.replace(pos); Ok(())}},
            set_grab_key_focus(focus: bool) {|c_ref| {c_ref.grab_key_focus = focus; Ok(())}},
            set_icon_height(height: Size) {|c_ref| {c_ref.icon_walk.height = height; Ok(())}},
            set_icon_width(width: Size) {|c_ref| {c_ref.icon_walk.width = width; Ok(())}},
            set_icon_margin(margin: Margin) {|c_ref| {c_ref.icon_walk.margin = margin; Ok(())}},
            set_icon_abs_pos(pos: DVec2) {|c_ref| {c_ref.icon_walk.abs_pos.replace(pos); Ok(())}},
            set_icon_padding(padding: Padding) {|c_ref| {c_ref.icon_layout.padding = padding; Ok(())}},
            set_icon_align(align: Align) {|c_ref| {c_ref.icon_layout.align = align; Ok(())}},
            set_icon_clip_x(clip: bool) {|c_ref| {c_ref.icon_layout.clip_x = clip; Ok(())}},
            set_icon_clip_y(clip: bool) {|c_ref| {c_ref.icon_layout.clip_y = clip; Ok(())}},
            set_icon_scroll(scroll: DVec2) {|c_ref| {c_ref.icon_layout.scroll = scroll; Ok(())}},
            set_icon_flow(flow: Flow) {|c_ref| {c_ref.icon_layout.flow = flow; Ok(())}},
            set_icon_spacing(spacing: f64) {|c_ref| {c_ref.icon_layout.spacing = spacing; Ok(())}},
            set_height(height: Size) {|c_ref| {c_ref.walk.height = height; Ok(())}},
            set_width(width: Size) {|c_ref| {c_ref.walk.width = width; Ok(())}},
            set_margin(margin: Margin) {|c_ref| {c_ref.walk.margin = margin; Ok(())}},
            set_abs_pos(pos: DVec2) {|c_ref| {c_ref.walk.abs_pos.replace(pos); Ok(())}},
            set_align(align: Align) {|c_ref| {c_ref.layout.align = align; Ok(())}},
            set_clip_x(clip: bool) {|c_ref| {c_ref.layout.clip_x = clip; Ok(())}},
            set_clip_y(clip: bool) {|c_ref| {c_ref.layout.clip_y = clip; Ok(())}},
            set_scroll(scroll: DVec2) {|c_ref| {c_ref.layout.scroll = scroll; Ok(())}},
            set_flow(flow: Flow) {|c_ref| {c_ref.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64) {|c_ref| {c_ref.layout.spacing = spacing; Ok(())}},
            set_animation_key(key: bool) {|c_ref| {c_ref.animation_key = key; Ok(())}},
            set_event_key(key: bool) {|c_ref| {c_ref.event_key = key; Ok(())}}
        }
    }

    prop_getter! {
        GTag{
            get_theme(Themes) {|| Themes::default()}, {|c_ref| {c_ref.theme}},
            get_background_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_tag.background_color)}},
            get_hover_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_tag.hover_color)}},
            get_focus_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_tag.focus_color)}},
            get_border_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_tag.border_color)}},
            get_shadow_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_tag.shadow_color)}},
            get_text_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_text.color)}},
            get_icon_color(String) {|| Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_icon.color)}},
            get_background_visible(bool) {|| Default::default()}, {|c_ref| {c_ref.draw_tag.background_visible.to_bool()}},
            get_border_width(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_tag.border_width}},
            get_border_radius(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_tag.border_radius}},
            get_spread_radius(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_tag.spread_radius}},
            get_blur_radius(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_tag.blur_radius}},
            get_shadow_offset(Vec2) {|| Default::default()}, {|c_ref| {c_ref.draw_tag.shadow_offset}},
            get_font_size(f64) {|| Default::default()}, {|c_ref| {c_ref.font_size}},
            get_height_factor(f64) {|| Default::default()}, {|c_ref| {c_ref.height_factor}},
            get_line_scale(f64) {|| Default::default()}, {|c_ref| {c_ref.line_scale}},
            get_icon_brightness(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_icon.brightness}},
            get_icon_curve(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_icon.curve}},
            get_icon_linearize(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_icon.linearize}},
            get_icon_scale(f64) {|| Default::default()}, {|c_ref| {c_ref.icon_scale}},
            get_icon_draw_depth(f32) {|| Default::default()}, {|c_ref| {c_ref.draw_icon.draw_depth}},
            get_cursor(MouseCursor) {|| Default::default()}, {|c_ref| {c_ref.cursor.unwrap_or_default()}},
            get_closeable(bool) {|| Default::default()}, {|c_ref| {c_ref.closeable}},
            get_text_height(Size) {|| Default::default()}, {|c_ref| {c_ref.text_walk.height}},
            get_text_width(Size) {|| Default::default()}, {|c_ref| {c_ref.text_walk.width}},
            get_text_margin(Margin) {|| Default::default()}, {|c_ref| {c_ref.text_walk.margin}},
            get_text_abs_pos(DVec2) {|| Default::default()}, {|c_ref| {c_ref.text_walk.abs_pos.unwrap_or_default()}},
            get_grab_key_focus(bool) {|| Default::default()}, {|c_ref| {c_ref.grab_key_focus}},
            get_icon_height(Size) {|| Default::default()}, {|c_ref| {c_ref.icon_walk.height}},
            get_icon_width(Size) {|| Default::default()}, {|c_ref| {c_ref.icon_walk.width}},
            get_icon_margin(Margin) {|| Default::default()}, {|c_ref| {c_ref.icon_walk.margin}},
            get_icon_abs_pos(DVec2) {|| Default::default()}, {|c_ref| {c_ref.icon_walk.abs_pos.unwrap_or_default()}},
            get_icon_padding(Padding) {|| Default::default()}, {|c_ref| {c_ref.icon_layout.padding}},
            get_icon_align(Align) {|| Default::default()}, {|c_ref| {c_ref.icon_layout.align}},
            get_icon_clip_x(bool) {|| Default::default()}, {|c_ref| {c_ref.icon_layout.clip_x}},
            get_icon_clip_y(bool) {|| Default::default()}, {|c_ref| {c_ref.icon_layout.clip_y}},
            get_icon_scroll(DVec2) {|| Default::default()}, {|c_ref| {c_ref.icon_layout.scroll}},
            get_icon_flow(Flow) {|| Default::default()}, {|c_ref| {c_ref.icon_layout.flow}},
            get_icon_spacing(f64) {|| Default::default()}, {|c_ref| {c_ref.icon_layout.spacing}},
            get_height(Size) {|| Default::default()}, {|c_ref| {c_ref.walk.height}},
            get_width(Size) {|| Default::default()}, {|c_ref| {c_ref.walk.width}},
            get_margin(Margin) {|| Default::default()}, {|c_ref| {c_ref.walk.margin}},
            get_abs_pos(DVec2) {|| Default::default()}, {|c_ref| {c_ref.walk.abs_pos.unwrap_or_default()}},
            get_align(Align) {|| Default::default()}, {|c_ref| {c_ref.layout.align}},
            get_clip_x(bool) {|| Default::default()}, {|c_ref| {c_ref.layout.clip_x}},
            get_clip_y(bool) {|| Default::default()}, {|c_ref| {c_ref.layout.clip_y}},
            get_scroll(DVec2) {|| Default::default()}, {|c_ref| {c_ref.layout.scroll}},
            get_flow(Flow) {|| Default::default()}, {|c_ref| {c_ref.layout.flow}},
            get_spacing(f64) {|| Default::default()}, {|c_ref| {c_ref.layout.spacing}},
            get_animation_key(bool) {|| Default::default()}, {|c_ref| {c_ref.animation_key}},
            get_event_key(bool) {|| Default::default()}, {|c_ref| {c_ref.event_key}},
            get_text(String) {|| Default::default()}, {|c_ref| {c_ref.text.as_ref().to_string()}}
        }
    }

    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_area_ext! {
        area_close,
        area_icon,
        area_text
    }
    ref_event_option! {
        clicked => GTagClickedParam,
        hover_in => GTagHoverParam,
        hover_out => GTagHoverParam,
        focus => GTagFocusParam,
        focus_lost => GTagFocusLostParam,
        closed => GTagClosedParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.set_visible(cx, visible);
        }
    }
}

impl GTagSet {
    set_event! {
        clicked => GTagClickedParam,
        hover_in => GTagHoverParam,
        hover_out => GTagHoverParam,
        focus => GTagFocusParam,
        focus_lost => GTagFocusLostParam,
        closed => GTagClosedParam
    }
}
