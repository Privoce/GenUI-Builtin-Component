mod event;
pub mod register;

pub use event::*;

use crate::shader::draw_icon_pixel::DrawGIconPixel;
use crate::shader::draw_svg::DrawGSvg;
use crate::shader::draw_text::DrawGText;
use crate::utils::{
    get_font_family, hex_to_vec4, set_cursor, vec4_to_hex, BoolToF32, RectExp, ThemeColor, ToBool,
};
use crate::{
    active_event, animatie_fn, check_event_scope, default_handle_animation,
    default_hit_finger_down, default_hit_hover_in, default_hit_hover_out, event_option, getter,
    play_animation, pure_after_apply, ref_area, ref_area_ext, ref_event_option, ref_getter_setter,
    ref_redraw, ref_render, render_after_apply, set_event, set_scope_path, 
    setter, widget_area,
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
            HitOptions::new().with_sweep_area(sweep_area),
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
    fn visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GTag {
    // fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
    //     if !self.visible {
    //         return;
    //     }
    //     if let Err(e) = self.render(cx) {
    //         error!("GTag render error: {:?}", e);
    //     }
    // }
    pure_after_apply!();
}

impl GTag {
    render_after_apply!("GTag");
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
    // pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
    //     self.visible = visible;
    //     if visible {
    //         self.clear_animation(cx);
    //         self.redraw(cx);
    //     }
    // }
    setter! {
        GTag{
            set_theme(theme: Themes) {|c, cx| {c.theme = theme; c.render(cx)}},
            set_background_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.background_color.replace(color); c.draw_tag.background_color = color; Ok(())}},
            set_hover_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.hover_color.replace(color); c.draw_tag.hover_color = color; Ok(())}},
            set_focus_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.focus_color.replace(color); c.draw_tag.focus_color = color; Ok(())}},
            set_border_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.border_color.replace(color); c.draw_tag.border_color = color; Ok(())}},
            set_shadow_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.shadow_color.replace(color); c.draw_tag.shadow_color = color; Ok(())}},
            set_text_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.color.replace(color); c.draw_text.color = color; Ok(())}},
            set_icon_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.icon_color.replace(color); c.draw_icon.color = color; c.draw_close.color = color; Ok(())}},
            set_stroke_hover_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.stroke_hover_color.replace(color); c.draw_text.stroke_hover_color = color; c.draw_icon.stroke_hover_color = color; Ok(())}},
            set_stroke_focus_color(color: String) {|c, _cx| {let color = hex_to_vec4(&color)?; c.stroke_focus_color.replace(color); c.draw_text.stroke_focus_color = color; c.draw_icon.stroke_focus_color = color; Ok(())}},
            set_background_visible(visible: bool) {|c, _cx| {c.background_visible = visible; c.draw_tag.background_visible = visible.to_f32(); Ok(())}},
            set_border_width(width: f32) {|c, _cx| {c.border_width = width; c.draw_tag.border_width = width; Ok(())}},
            set_border_radius(radius: f32) {|c, _cx| {c.border_radius = radius; c.draw_tag.border_radius = radius; Ok(())}},
            set_spread_radius(radius: f32) {|c, _cx| {c.spread_radius = radius; c.draw_tag.spread_radius = radius; Ok(())}},
            set_blur_radius(radius: f32) {|c, _cx| {c.blur_radius = radius; c.draw_tag.blur_radius = radius; Ok(())}},
            set_shadow_offset(offset: Vec2) {|c, _cx| {c.shadow_offset = offset; c.draw_tag.shadow_offset = offset; Ok(())}},
            set_font_size(size: f64) {|c, _cx| {c.font_size = size; c.draw_text.text_style.font_size = size; Ok(())}},
            set_height_factor(factor: f64) {|c, _cx| {c.height_factor = factor; c.draw_text.text_style.height_factor = factor; Ok(())}},
            set_line_scale(scale: f64) {|c, _cx| {c.line_scale = scale; c.draw_text.text_style.line_scale = scale; Ok(())}},
            set_icon_brightness(brightness: f32) {|c, _cx| {c.icon_brightness = brightness; c.draw_icon.brightness = brightness; c.draw_close.brightness = brightness; Ok(())}},
            set_icon_curve(curve: f32) {|c, _cx| {c.icon_curve = curve; c.draw_icon.curve = curve; c.draw_close.curve = curve; Ok(())}},
            set_icon_linearize(linearize: f32) {|c, _cx| {c.icon_linearize = linearize; c.draw_icon.linearize = linearize; c.draw_close.linearize = linearize; Ok(())}},
            set_icon_scale(scale: f64) {|c, _cx| {c.icon_scale = scale; c.draw_icon.scale = scale; Ok(())}},
            set_icon_draw_depth(depth: f32) {|c, _cx| {c.icon_draw_depth = depth; c.draw_icon.draw_depth = depth; c.draw_close.draw_depth = depth; Ok(())}},
            set_cursor(cursor: MouseCursor) {|c, _cx| {c.cursor.replace(cursor); Ok(())}},
            set_closeable(closeable: bool) {|c, _cx| {c.closeable = closeable; Ok(())}},
            set_text_height(height: Size) {|c, _cx| {c.text_walk.height = height; Ok(())}},
            set_text_width(width: Size) {|c, _cx| {c.text_walk.width = width; Ok(())}},
            set_text_margin(margin: Margin) {|c, _cx| {c.text_walk.margin = margin; Ok(())}},
            set_text_abs_pos(pos: Option<DVec2>) {|c, _cx| {c.text_walk.abs_pos = pos; Ok(())}},
            set_grab_key_focus(focus: bool) {|c, _cx| {c.grab_key_focus = focus; Ok(())}},
            set_icon_height(height: Size) {|c, _cx| {c.icon_walk.height = height; Ok(())}},
            set_icon_width(width: Size) {|c, _cx| {c.icon_walk.width = width; Ok(())}},
            set_icon_margin(margin: Margin) {|c, _cx| {c.icon_walk.margin = margin; Ok(())}},
            set_icon_abs_pos(pos: Option<DVec2>) {|c, _cx| {c.icon_walk.abs_pos = pos; Ok(())}},
            set_icon_padding(padding: Padding) {|c, _cx| {c.icon_layout.padding = padding; Ok(())}},
            set_icon_align(align: Align) {|c, _cx| {c.icon_layout.align = align; Ok(())}},
            set_icon_clip_x(clip: bool) {|c, _cx| {c.icon_layout.clip_x = clip; Ok(())}},
            set_icon_clip_y(clip: bool) {|c, _cx| {c.icon_layout.clip_y = clip; Ok(())}},
            set_icon_scroll(scroll: DVec2) {|c, _cx| {c.icon_layout.scroll = scroll; Ok(())}},
            set_icon_flow(flow: Flow) {|c, _cx| {c.icon_layout.flow = flow; Ok(())}},
            set_icon_spacing(spacing: f64) {|c, _cx| {c.icon_layout.spacing = spacing; Ok(())}},
            set_height(height: Size) {|c, _cx| {c.walk.height = height; Ok(())}},
            set_width(width: Size) {|c, _cx| {c.walk.width = width; Ok(())}},
            set_margin(margin: Margin) {|c, _cx| {c.walk.margin = margin; Ok(())}},
            set_abs_pos(pos: Option<DVec2>) {|c, _cx| {c.walk.abs_pos = pos; Ok(())}},
            set_align(align: Align) {|c, _cx| {c.layout.align = align; Ok(())}},
            set_clip_x(clip: bool) {|c, _cx| {c.layout.clip_x = clip; Ok(())}},
            set_clip_y(clip: bool) {|c, _cx| {c.layout.clip_y = clip; Ok(())}},
            set_scroll(scroll: DVec2) {|c, _cx| {c.layout.scroll = scroll; Ok(())}},
            set_flow(flow: Flow) {|c, _cx| {c.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64) {|c, _cx| {c.layout.spacing = spacing; Ok(())}},
            set_animation_key(key: bool) {|c, _cx| {c.animation_key = key; Ok(())}},
            set_event_key(key: bool) {|c, _cx| {c.event_key = key; Ok(())}},
            set_text(text: String){|c, _cx| {c.text.as_mut_empty().push_str(&text); Ok(())}},
            set_visible(visible: bool) {|c, _cx| {c.visible = visible;Ok(())}}
        }
    }
    getter! {
        GTag{
            get_theme(Themes) {|c| {c.theme}},
            get_background_color(String) {|c| {vec4_to_hex(&c.draw_tag.background_color)}},
            get_hover_color(String) {|c| {vec4_to_hex(&c.draw_tag.hover_color)}},
            get_focus_color(String) {|c| {vec4_to_hex(&c.draw_tag.focus_color)}},
            get_border_color(String) {|c| {vec4_to_hex(&c.draw_tag.border_color)}},
            get_shadow_color(String) {|c| {vec4_to_hex(&c.draw_tag.shadow_color)}},
            get_text_color(String) {|c| {vec4_to_hex(&c.draw_text.color)}},
            get_icon_color(String) {|c| {vec4_to_hex(&c.draw_icon.color)}},
            get_stroke_hover_color(String) {|c| {vec4_to_hex(&c.draw_text.stroke_hover_color)}},
            get_stroke_focus_color(String) {|c| {vec4_to_hex(&c.draw_text.stroke_focus_color)}},
            get_background_visible(bool) {|c| {c.draw_tag.background_visible.to_bool()}},
            get_border_width(f32) {|c| {c.draw_tag.border_width}},
            get_border_radius(f32) {|c| {c.draw_tag.border_radius}},
            get_spread_radius(f32) {|c| {c.draw_tag.spread_radius}},
            get_blur_radius(f32) {|c| {c.draw_tag.blur_radius}},
            get_shadow_offset(Vec2) {|c| {c.draw_tag.shadow_offset}},
            get_font_size(f64) {|c| {c.font_size}},
            get_height_factor(f64) {|c| {c.height_factor}},
            get_line_scale(f64) {|c| {c.line_scale}},
            get_icon_brightness(f32) {|c| {c.draw_icon.brightness}},
            get_icon_curve(f32) {|c| {c.draw_icon.curve}},
            get_icon_linearize(f32) {|c| {c.draw_icon.linearize}},
            get_icon_scale(f64) {|c| {c.icon_scale}},
            get_icon_draw_depth(f32) {|c| {c.draw_icon.draw_depth}},
            get_cursor(MouseCursor) {|c| {c.cursor.unwrap_or_default()}},
            get_closeable(bool) {|c| {c.closeable}},
            get_text_height(Size) {|c| {c.text_walk.height}},
            get_text_width(Size) {|c| {c.text_walk.width}},
            get_text_margin(Margin) {|c| {c.text_walk.margin}},
            get_text_abs_pos(Option<DVec2>) {|c| {c.text_walk.abs_pos.clone()}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_icon_height(Size) {|c| {c.icon_walk.height}},
            get_icon_width(Size) {|c| {c.icon_walk.width}},
            get_icon_margin(Margin) {|c| {c.icon_walk.margin}},
            get_icon_abs_pos(Option<DVec2>) {|c| {c.icon_walk.abs_pos.clone()}},
            get_icon_padding(Padding) {|c| {c.icon_layout.padding}},
            get_icon_align(Align) {|c| {c.icon_layout.align}},
            get_icon_clip_x(bool) {|c| {c.icon_layout.clip_x}},
            get_icon_clip_y(bool) {|c| {c.icon_layout.clip_y}},
            get_icon_scroll(DVec2) {|c| {c.icon_layout.scroll}},
            get_icon_flow(Flow) {|c| {c.icon_layout.flow}},
            get_icon_spacing(f64) {|c| {c.icon_layout.spacing}},
            get_height(Size) {|c| {c.walk.height}},
            get_width(Size) {|c| {c.walk.width}},
            get_margin(Margin) {|c| {c.walk.margin}},
            get_abs_pos(Option<DVec2>) {|c| {c.walk.abs_pos.clone()}},
            get_align(Align) {|c| {c.layout.align}},
            get_clip_x(bool) {|c| {c.layout.clip_x}},
            get_clip_y(bool) {|c| {c.layout.clip_y}},
            get_scroll(DVec2) {|c| {c.layout.scroll}},
            get_flow(Flow) {|c| {c.layout.flow}},
            get_spacing(f64) {|c| {c.layout.spacing}},
            get_animation_key(bool) {|c| {c.animation_key}},
            get_event_key(bool) {|c| {c.event_key}},
            get_text(String) {|c| {c.text.as_ref().to_string()}},
            get_visible(bool) {|c| {c.visible}}
        }
    }
}

impl GTagRef {
    ref_getter_setter! {
        get_theme, set_theme -> Themes,
        get_background_color, set_background_color -> String,
        get_hover_color, set_hover_color -> String,
        get_focus_color, set_focus_color -> String,
        get_border_color, set_border_color -> String,
        get_shadow_color, set_shadow_color -> String,
        get_text_color, set_text_color -> String,
        get_icon_color, set_icon_color -> String,
        get_stroke_hover_color, set_stroke_hover_color -> String,
        get_stroke_focus_color, set_stroke_focus_color -> String,
        get_background_visible, set_background_visible -> bool,
        get_border_width, set_border_width -> f32,
        get_border_radius, set_border_radius -> f32,
        get_spread_radius, set_spread_radius -> f32,
        get_blur_radius, set_blur_radius -> f32,
        get_shadow_offset, set_shadow_offset -> Vec2,
        get_font_size, set_font_size -> f64,
        get_height_factor, set_height_factor -> f64,
        get_line_scale, set_line_scale -> f64,
        get_icon_brightness, set_icon_brightness -> f32,
        get_icon_curve, set_icon_curve -> f32,
        get_icon_linearize, set_icon_linearize -> f32,
        get_icon_scale, set_icon_scale -> f64,
        get_icon_draw_depth, set_icon_draw_depth -> f32,
        get_cursor, set_cursor -> MouseCursor,
        get_closeable, set_closeable -> bool,
        get_text_height, set_text_height -> Size,
        get_text_width, set_text_width -> Size,
        get_text_margin, set_text_margin -> Margin,
        get_text_abs_pos, set_text_abs_pos -> Option<DVec2>,
        get_grab_key_focus, set_grab_key_focus -> bool,
        get_icon_height, set_icon_height -> Size,
        get_icon_width, set_icon_width -> Size,
        get_icon_margin, set_icon_margin -> Margin,
        get_icon_abs_pos, set_icon_abs_pos -> Option<DVec2>,
        get_icon_padding, set_icon_padding -> Padding,
        get_icon_align, set_icon_align -> Align,
        get_icon_clip_x, set_icon_clip_x -> bool,
        get_icon_clip_y, set_icon_clip_y -> bool,
        get_icon_scroll, set_icon_scroll -> DVec2,
        get_icon_flow, set_icon_flow -> Flow,
        get_icon_spacing, set_icon_spacing -> f64,
        get_height, set_height -> Size,
        get_width, set_width -> Size,
        get_margin, set_margin -> Margin,
        get_abs_pos, set_abs_pos -> Option<DVec2>,
        get_align, set_align -> Align,
        get_clip_x, set_clip_x -> bool,
        get_clip_y, set_clip_y -> bool,
        get_scroll, set_scroll -> DVec2,
        get_flow, set_flow -> Flow,
        get_spacing, set_spacing -> f64,
        get_animation_key, set_animation_key -> bool,
        get_event_key, set_event_key -> bool,
        get_text, set_text -> String,
        get_visible, set_visible -> bool
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
