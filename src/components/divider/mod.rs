pub mod register;

use makepad_widgets::*;

use crate::{
    animatie_fn, prop_getter, prop_setter, ref_area, ref_event_option, ref_redraw_mut, ref_render,
    shader::manual::Direction, themes::Themes, utils::ToBool,
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
    fn after_apply_from_doc(&mut self, cx:&mut Cx) {
        if !self.visible {
            return;
        }

        self.render(cx);
    }
}

impl GDivider {
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
    pub fn render(&mut self, cx: &mut Cx) {
        self.deref_widget.render(cx);
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
}

impl GDividerRef {
    prop_setter! {
        GDivider{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme;}},
            set_background_color(color: Vec4) {|c_ref| {c_ref.background_color.replace(color);}},
            set_shadow_color(color: Vec4) {|c_ref| {c_ref.shadow_color.replace(color);}},
            set_hover_color(color: Vec4) {|c_ref| {c_ref.hover_color.replace(color);}},
            set_focus_color(color: Vec4) {|c_ref| {c_ref.focus_color.replace(color);}},
            set_border_color(color: Vec4) {|c_ref| {c_ref.border_color.replace(color);}},
            set_border_width(width: f64) {|c_ref| {c_ref.border_width = width as f32;}},
            set_border_radius(radius: f64) {|c_ref| {c_ref.border_radius = radius as f32;}},
            set_shadow_offset(offset: Vec2) {|c_ref| {c_ref.shadow_offset = offset;}},
            set_spread_radius(radius: f64) {|c_ref| {c_ref.spread_radius = radius as f32;}},
            set_blur_radius(radius: f64) {|c_ref| {c_ref.blur_radius = radius as f32;}},
            set_background_visible(visible: bool) {|c_ref| {c_ref.background_visible = visible;}},
            set_visible(visible: bool) {|c_ref| {c_ref.visible = visible;}},
            set_cursor(cursor: MouseCursor) {|c_ref| {c_ref.cursor = Some(cursor);}},
            set_grab_key_focus(grab: bool) {|c_ref| {c_ref.grab_key_focus = grab;}},
            set_block_signal_event(block: bool) {|c_ref| {c_ref.block_signal_event = block;}},
            set_abs_pos(pos: DVec2) {|c_ref| {c_ref.walk.abs_pos.replace(pos);}},
            set_margin(margin: Margin) {|c_ref| {c_ref.walk.margin = margin;}},
            set_height(height: Size) {|c_ref| {c_ref.walk.height = height;}},
            set_width(width: Size) {|c_ref| {c_ref.walk.width = width;}},
            set_scroll(scroll: DVec2) {|c_ref| {c_ref.layout.scroll = scroll;}},
            set_clip_x(clip: bool) {|c_ref| {c_ref.layout.clip_x = clip;}},
            set_clip_y(clip: bool) {|c_ref| {c_ref.layout.clip_y = clip;}},
            set_padding(padding: Padding) {|c_ref| {c_ref.layout.padding = padding;}},
            set_align(align: Align) {|c_ref| {c_ref.layout.align = align;}},
            set_flow(flow: Flow) {|c_ref| {c_ref.layout.flow = flow;}},
            set_spacing(spacing: f64) {|c_ref| {c_ref.layout.spacing = spacing;}},
            set_dpi_factor(factor: f64) {|c_ref| {c_ref.dpi_factor.replace(factor);}},
            set_optimize(optimize: ViewOptimize) {|c_ref| {c_ref.optimize = optimize;}},
            set_capture_overload(overload: bool) {|c_ref| {c_ref.capture_overload = overload;}},
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key;}},
            set_stroke_width(width: f32) {|c_ref| {c_ref.stroke_width = width;}},
            set_direction(direction: Direction) {|c_ref| {c_ref.direction = direction;}}
        }
    }
    prop_getter! {
        GDivider{
            get_theme(Themes) {|| Themes::default()}, {|c_ref| {c_ref.theme}},
            get_background_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.background_color}},
            get_shadow_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.shadow_color}},
            get_hover_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.hover_color}},
            get_focus_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.focus_color}},
            get_border_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.border_color}},
            get_border_width(f64) {|| 0.0}, {|c_ref| {c_ref.border_width as f64}},
            get_border_radius(f64) {|| 0.0}, {|c_ref| {c_ref.border_radius as f64}},
            get_shadow_offset(Vec2) {|| Vec2::default()}, {|c_ref| {c_ref.shadow_offset}},
            get_spread_radius(f64) {|| 0.0}, {|c_ref| {c_ref.spread_radius as f64}},
            get_blur_radius(f64) {|| 0.0}, {|c_ref| {c_ref.blur_radius as f64}},
            get_background_visible(bool) {|| true}, {|c_ref| {c_ref.draw_view.background_visible.to_bool()}},
            get_visible(bool) {|| true}, {|c_ref| {c_ref.visible}},
            get_cursor(MouseCursor) {|| MouseCursor::Default}, {|c_ref| {c_ref.cursor.unwrap_or_default()}},
            get_grab_key_focus(bool) {|| false}, {|c_ref| {c_ref.grab_key_focus}},
            get_block_signal_event(bool) {|| false}, {|c_ref| {c_ref.block_signal_event}},
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
            get_dpi_factor(Option<f64>) {||None}, {|c_ref| {c_ref.dpi_factor}},
            get_optimize(ViewOptimize) {||ViewOptimize::None}, {|c_ref| {c_ref.optimize}},
            get_capture_overload(bool) {||false}, {|c_ref| {c_ref.capture_overload}},
            get_event_key(bool) {||true}, {|c_ref| {c_ref.event_key}},
            get_stroke_width(f32) {||1.4}, {|c_ref| {c_ref.stroke_width}},
            get_direction(Direction) {||Direction::Horizontal}, {|c_ref| {c_ref.direction.clone()}}
        }
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
