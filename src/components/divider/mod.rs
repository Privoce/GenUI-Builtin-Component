pub mod register;

use makepad_widgets::*;

use crate::{
    animatie_fn, getter, pure_after_apply, ref_area, ref_event_option, ref_getter_setter, ref_redraw_mut, ref_render, render_after_apply, setter, shader::manual::Direction, themes::Themes, utils::{BoolToF32, ToBool}
};

use super::view::{
    event::{GViewClickedParam, GViewFocusLostParam, GViewFocusParam, GViewHoverParam},
    GView,
};

live_design! {
    link gen_base;
    use link::shaders::*;

    pub GDividerBase = {{GDivider}}{
        height: 2.0,
        width: Fill,
        align: {x: 0.5, y: 0.5},
        draw_view: {
            // direction is 1.0 for horizontal and 0.0 for vertical
            instance direction: 1.0,
            instance stroke_width: 1.4,
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                if self.direction == 1.0 {
                    sdf.box(
                        self.pos.x,
                        self.pos.y + self.rect_size.y / 2.0 - self.stroke_width / 2.0,
                        self.rect_size.x,
                        self.stroke_width,
                        max(1.0, self.border_radius)
                    );
                } else {
                    sdf.box(
                        self.pos.x + self.rect_size.x / 2.0 - self.stroke_width / 2.0,
                        self.pos.y,
                        self.stroke_width,
                        self.rect_size.y,
                        max(1.0, self.border_radius)
                    );
                }

                if self.background_visible != 0.0 {
                    sdf.fill(self.get_background_color());
                }
                return sdf.result;
            }
        }
    }
}

/// # GDivider component
/// The `GDivider` component is used to create a simple dividing line between other UI elements. It can be horizontal or vertical based on the `direction` property.
///
/// ## Animation
/// The `GDivider` inherits animation properties from `GView`, but typically, animations are not the primary focus for dividers. Instead, animations should be handled within inner components.
///
/// ## Event
/// The `GDivider` inherits event handling from `GView`. However, since it functions mainly as a visual separator, its event handling is generally minimal.
///
/// ## Props
/// |macro |prop           |description                          |type         |default|
/// |------|---------------|--------------------------------------|-------------|-------|
/// |live  |direction       |Divider direction: horizontal (1.0) or vertical (0.0)|f32 |1.0|
/// |live  |stroke_width    |The width of the divider's stroke    |f32          |1.4    |
///
/// > Other Props see: [GView]
#[derive(Live, Widget)]
pub struct GDivider {
    #[deref]
    pub deref_widget: GView,
    #[live(1.4)]
    pub stroke_width: f32,
    #[live(Direction::Horizontal)]
    pub direction: Direction,
}

impl Widget for GDivider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        self.deref_widget.handle_event(cx, event, scope)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GDivider {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    // fn after_apply_from_doc(&mut self, cx: &mut Cx) {
    //     if !self.visible {
    //         return;
    //     }

    //     if let Err(e) = self.render(cx) {
    //         error!("GDivider render error: {:?}", e);
    //     }
    // }
    pure_after_apply!();
}

impl GDivider {
    render_after_apply!("GDivider");
    pub fn animate_hover_on(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_hover_on(cx);
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_hover_off(cx);
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_focus_on(cx);
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) {
        self.deref_widget.animate_focus_off(cx);
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.deref_widget.clear_animation(cx);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        self.deref_widget.render(cx)
    }
    pub fn area(&self) -> Area {
        self.deref_widget.area()
    }
    pub fn hover_in(&self, actions: &Actions) -> Option<GViewHoverParam> {
        self.deref_widget.hover_in(actions)
    }
    pub fn hover_out(&self, actions: &Actions) -> Option<GViewHoverParam> {
        self.deref_widget.hover_out(actions)
    }
    pub fn focus(&self, actions: &Actions) -> Option<GViewFocusParam> {
        self.deref_widget.focus(actions)
    }
    pub fn focus_lost(&self, actions: &Actions) -> Option<GViewFocusLostParam> {
        self.deref_widget.focus_lost(actions)
    }
    pub fn clicked(&self, actions: &Actions) -> Option<GViewClickedParam> {
        self.deref_widget.clicked(actions)
    }
    setter! {
        GDivider{
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
            set_abs_pos(pos: Option<DVec2>) {|c, _cx| {c.walk.abs_pos = pos; Ok(())}},
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
            set_event_key(event_key: bool) {|c, _cx| {c.event_key = event_key; Ok(())}},
            set_stroke_width(width: f32) {|c, _cx| {c.stroke_width = width; Ok(())}},
            set_direction(direction: Direction) {|c, _cx| {c.direction = direction; Ok(())}}
        }
    }
    getter! {
        GDivider{
            get_theme(Themes) {|c| {c.theme}},
            get_background_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.background_color)}},
            get_shadow_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.shadow_color)}},
            get_hover_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.hover_color)}},
            get_focus_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.focus_color)}},
            get_border_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.border_color)}},
            get_border_width(f32) {|c| {c.border_width}},
            get_border_radius(f32) {|c| {c.border_radius}},
            get_shadow_offset(Vec2) {|c| {c.shadow_offset}},
            get_spread_radius(f32) {|c| {c.spread_radius}},
            get_blur_radius(f32) {|c| {c.blur_radius}},
            get_background_visible(bool) {|c| {c.draw_view.background_visible.to_bool()}},
            get_visible(bool) {|c| {c.visible}},
            get_cursor(MouseCursor) {|c| {c.cursor.unwrap_or_default()}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_block_signal_event(bool) {|c| {c.block_signal_event}},
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
            get_dpi_factor(f64) {|c| {c.dpi_factor.unwrap_or_default()}},
            get_optimize(ViewOptimize) {|c| {c.optimize}},
            get_capture_overload(bool) {|c| {c.capture_overload}},
            get_event_key(bool) {|c| {c.event_key}},
            get_stroke_width(f32) {|c| {c.stroke_width}},
            get_direction(Direction) {|c| {c.direction.clone()}}
        }
    }
}

impl GDividerRef {
    ref_getter_setter! {
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
        get_dpi_factor, set_dpi_factor -> f64,
        get_optimize, set_optimize -> ViewOptimize,
        get_capture_overload, set_capture_overload -> bool,
        get_event_key, set_event_key -> bool,
        get_stroke_width, set_stroke_width -> f32,
        get_direction, set_direction -> Direction
    }
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    ref_event_option! {
        hover_in => GViewHoverParam,
        hover_out => GViewHoverParam,
        focus => GViewFocusParam,
        focus_lost => GViewFocusLostParam,
        clicked => GViewClickedParam
    }
}
