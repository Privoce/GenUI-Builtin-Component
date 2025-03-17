mod event;
pub mod register;

pub use event::*;

use crate::utils::{set_cursor, BoolToF32, ThemeColor};
use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down, default_hit_finger_up, default_hit_hover_in, default_hit_hover_out, event_option, getter, play_animation, pure_after_apply, ref_area, ref_event_option, ref_getter_setter, ref_play_animation, ref_redraw, ref_render, render_after_apply, set_event, set_scope_path, setter, widget_area
};
use crate::{shader::draw_view::DrawGView, themes::Themes};
use makepad_widgets::*;

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::*;

    pub GButtonBase = {{GButton}}{
        height: Fit,
        width: Fit,
        theme: Primary,
        padding: <GLOBAL_PADDING>{}
        align: <ALIGN_CENTER_WALK>{},
        clip_x: false,
        clip_y: false,
        cursor: Hand,
        shadow_offset: vec2(0.0, 2.0),
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)},
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_button: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {hover: 0.0, focus: 1.0}
                    }
                }
            }
        }
    }
}

/// # GButton Component
///
/// The `GButton` component is a customizable button designed for interactive UI elements. It supports hover, focus, and click animations, with various configurable properties for appearance, including background color, shadow, and border styles.
///
/// ## Animation
/// This component supports animations, particularly for hover and focus states. The default hover and focus animations are defined using the `animator` field:
/// - **hover.off**:  
///   - `draw_button.hover`: changes to `0.0`  
///   - `draw_button.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.on**:  
///   - `draw_button.hover`: changes to `1.0`  
///   - `draw_button.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
/// - **hover.focus**:  
///   - `draw_button.hover`: changes to `0.0`  
///   - `draw_button.focus`: changes to `1.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
///
/// ## Event
///
/// The `GButton` component supports the following events:
///
/// - **HoverIn**: Triggered when the mouse hovers over the button.
/// - **HoverOut**: Triggered when the mouse leaves the button.
/// - **Clicked**: Triggered when the button is clicked.
/// - **Focus**: Triggered when the button receives focus (e.g., via a keyboard event).
/// - **FocusLost**: Triggered when the button loses focus.
///
/// These events can be customized to trigger additional behaviors based on user interactions.
///
/// ## Props
///
/// | macro  | prop               | description                               | type             | default   |
/// |--------|--------------------|-------------------------------------------|------------------|-----------|
/// | live   | theme              | Theme of the button                       | `Themes`         |           |
/// | live   | background_color   | Background color of the button            | `Option<Vec4>`   | `None`    |
/// | live   | background_visible | Visibility of the background              | `bool`           | `true`    |
/// | live   | hover_color        | Color of the button when hovered          | `Option<Vec4>`   | `None`    |
/// | live   | focus_color        | Color of the button when focused          | `Option<Vec4>`   | `None`    |
/// | live   | shadow_color       | Color of the shadow                       | `Option<Vec4>`   | `None`    |
/// | live   | spread_radius      | Spread radius of the shadow               | `f32`            | `0.0`     |
/// | live   | blur_radius        | Blur radius of the shadow                 | `f32`            | `4.8`     |
/// | live   | shadow_offset      | Offset of the shadow                      | `Vec2`           |           |
/// | live   | border_color       | Color of the border                       | `Option<Vec4>`   | `None`    |
/// | live   | border_width       | Width of the border                       | `f32`            | `0.0`     |
/// | live   | border_radius      | Radius of the border's corners            | `f32`            | `2.0`     |
/// | live   | cursor             | Mouse cursor when hovering over the button| `Option<MouseCursor>`|        |
/// | live   | visible            | Whether the button is visible             | `bool`           | `true`    |
/// | live   | grab_key_focus     | Whether the button grabs keyboard focus   | `bool`           | `true`    |
/// | animator | animator         | Animation controller for the button       | `Animator`       |           |
/// | walk   | `abs_pos`           | Absolute position for layout             | `Option<DVec2>`    | `None`   |
/// | walk   | `margin`            | Margin size around the view              | `Margin`           | `Margin::default()` |
/// | walk   | `width`             | Width of the view                        | `Size`             | `Size::default()` |
/// | walk   | `height`            | Height of the view                       | `Size`             | `Size::default()` |
/// | layout | `scroll`            | Scroll position for layout               | `DVec2`            | `(0.0, 0.0)` |
/// | layout | `clip_x`            | Clip content horizontally                | `bool`             | `true`   |
/// | layout | `clip_y`            | Clip content vertically                  | `bool`             | `true`   |
/// | layout | `padding`           | Padding within the view                  | `Padding`          | `Padding::default()` |
/// | layout | `align`             | Alignment for content                    | `Align`            | `Align::default()` |
/// | layout | `flow`              | Flow direction of the content            | `Flow`             | `Flow::default()` |
/// | layout | `spacing`           | Spacing between elements                 | `f64`              | `0.0`    |
#[derive(Live, Widget)]
pub struct GButton {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // ---------------------------
    #[find]
    #[redraw]
    #[live]
    pub slot: WidgetRef,
    #[live(false)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    pub draw_button: DrawGView,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GButton {
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
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.draw_button.begin(cx, walk, self.layout);

        if self.slot.is_visible() {
            let slot_walk = self.slot.walk(cx);
            let _ = self.slot.draw_walk(cx, scope, slot_walk);
        }

        self.draw_button.end(cx);

        self.set_scope_path(&scope.path);

        DrawStep::done()
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

// impl LiveHook for GButton {
//     fn after_apply_from_doc(&mut self, cx: &mut Cx) {
//         if !self.visible {
//             return;
//         }
//         if let Err(e) = self.render(cx) {
//             error!("GButton render error: {:?}", e);
//         }
//     }
// }

impl LiveHook for GButton {
    pure_after_apply!();
}

impl GButton {
    render_after_apply!("GButton");
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_button,
        area_slot, slot
    }
    event_option! {
        hover_in: GButtonEvent::HoverIn => GButtonHoverParam,
        hover_out: GButtonEvent::HoverOut => GButtonHoverParam,
        focus: GButtonEvent::Focus => GButtonFocusParam,
        focus_lost: GButtonEvent::FocusLost => GButtonFocusLostParam,
        clicked: GButtonEvent::Clicked => GButtonClickedParam
    }
    active_event! {
        active_hover_in: GButtonEvent::HoverIn |e: FingerHoverEvent| => GButtonHoverParam {e},
        active_hover_out: GButtonEvent::HoverOut |e: FingerHoverEvent| => GButtonHoverParam {e},
        active_focus: GButtonEvent::Focus |e: FingerDownEvent| => GButtonFocusParam {e},
        active_focus_lost: GButtonEvent::FocusLost |e: FingerUpEvent| => GButtonFocusLostParam {e},
        active_clicked: GButtonEvent::Clicked |e: FingerUpEvent| => GButtonClickedParam {e}
    }
    pub fn render(&mut self, _cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 600);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let background_visible = self.background_visible.to_f32();
        // apply over props to draw_button ----------------------------------------------
        // self.draw_button.apply_over(
        //     cx,
        //     live! {
        //         background_color: (bg_color),
        //         background_visible: (background_visible),
        //         border_color: (border_color),
        //         border_width: (self.border_width),
        //         border_radius: (self.border_radius),
        //         focus_color: (focus_color),
        //         hover_color: (hover_color),
        //         shadow_color: (shadow_color),
        //         shadow_offset: (self.shadow_offset),
        //         spread_radius: (self.spread_radius),
        //         blur_radius: (self.blur_radius)
        //     },
        // );
        self.draw_button.background_color = bg_color;
        self.draw_button.background_visible = background_visible;
        self.draw_button.border_color = border_color;
        self.draw_button.border_width = self.border_width;
        self.draw_button.border_radius = self.border_radius;
        self.draw_button.focus_color = focus_color;
        self.draw_button.hover_color = hover_color;
        self.draw_button.shadow_color = shadow_color;
        self.draw_button.shadow_offset = self.shadow_offset;
        self.draw_button.spread_radius = self.spread_radius;
        self.draw_button.blur_radius = self.blur_radius;

        Ok(())
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, focus_area: Area) {
        default_handle_animation!(self, cx, event);
        match hit {
            Hit::FingerDown(e) => {
                default_hit_finger_down!(self, cx, focus_area, e);
            }
            Hit::FingerHoverIn(e) => {
                default_hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                default_hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                default_hit_finger_up!(self, cx, e);
            }
            _ => (),
        }
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_button.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_button.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_button.redraw(cx);
        if self.slot.is_visible() {
            self.slot.redraw(cx);
        }
    }
    setter! {
        GButton{
            set_theme(theme: Themes){|c, cx| {c.theme = theme; c.render(cx)}},
            set_background_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.background_color.replace(color); c.draw_button.background_color = color; Ok(())}},
            set_background_visible(visible: bool){|c, _cx| {c.background_visible = visible; c.draw_button.background_visible = visible.to_f32(); Ok(())}},
            set_hover_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.hover_color.replace(color); c.draw_button.hover_color = color; Ok(())}},
            set_focus_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.focus_color.replace(color); c.draw_button.focus_color = color; Ok(())}},
            set_shadow_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.shadow_color.replace(color); c.draw_button.shadow_color = color; Ok(())}},
            set_spread_radius(radius: f32){|c, _cx| {c.spread_radius = radius; c.draw_button.spread_radius = radius; Ok(())}},
            set_blur_radius(radius: f32){|c, _cx| {c.blur_radius = radius; c.draw_button.blur_radius = radius; Ok(())}},
            set_shadow_offset(offset: Vec2){|c, _cx| {c.shadow_offset = offset; c.draw_button.shadow_offset = offset; Ok(())}},
            set_border_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.border_color.replace(color); c.draw_button.border_color = color; Ok(())}},
            set_border_width(width: f32){|c, _cx| {c.border_width = width; c.draw_button.border_width = width; Ok(())}},
            set_border_radius(radius: f32){|c, _cx| {c.border_radius = radius; c.draw_button.border_radius = radius; Ok(())}},
            set_cursor(cursor: MouseCursor){|c, _cx| {c.cursor.replace(cursor); Ok(())}},
            set_visible(visible: bool){|c, _cx| {c.visible = visible; Ok(())}},
            set_grab_key_focus(grab: bool){|c, _cx| {c.grab_key_focus = grab; Ok(())}},
            set_animation_key(key: bool){|c, _cx| {c.animation_key = key; Ok(())}},
            set_abs_pos(pos: DVec2){|c, _cx| {c.walk.abs_pos.replace(pos); Ok(())}},
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
            set_event_key(key: bool){|c, _cx| {c.event_key = key; Ok(())}}
        }
    }
    getter! {
        GButton{
            get_theme(Themes) {|c| {c.theme}},
            get_background_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_button.background_color)}},
            get_background_visible(bool) {|c| {c.background_visible}},
            get_hover_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_button.hover_color)}},
            get_focus_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_button.focus_color)}},
            get_shadow_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_button.shadow_color)}},
            get_spread_radius(f32) {|c| {c.draw_button.spread_radius}},
            get_blur_radius(f32) {|c| {c.draw_button.blur_radius}},
            get_shadow_offset(Vec2) {|c| {c.draw_button.shadow_offset}},
            get_border_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_button.border_color)}},
            get_border_width(f32) {|c| {c.draw_button.border_width}},
            get_border_radius(f32) {|c| {c.draw_button.border_radius}},
            get_cursor(MouseCursor){|c| {c.cursor.unwrap_or_default()}},
            get_visible(bool) {|c| {c.visible}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_animation_key(bool) {|c| {c.animation_key}},
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
            get_event_key(bool) {|c| {c.event_key}}
        }
    }
}

impl GButtonRef {
    ref_event_option! {
        hover_in => GButtonHoverParam,
        hover_out => GButtonHoverParam,
        focus => GButtonFocusParam,
        focus_lost => GButtonFocusLostParam,
        clicked => GButtonClickedParam
    }
    ref_area!();
    ref_render!();
    ref_redraw!();
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    ref_play_animation! {
        play_hover_on: id!(hover.on),
        play_hover_off: id!(hover.off),
        play_focus_on: id!(hover.focus),
        play_focus_off: id!(hover.off)
    }
    ref_getter_setter!{
        get_theme, set_theme -> Themes,
        get_background_color, set_background_color -> String,
        get_background_visible, set_background_visible -> bool,
        get_hover_color, set_hover_color -> String,
        get_focus_color, set_focus_color -> String,
        get_shadow_color, set_shadow_color -> String,
        get_spread_radius, set_spread_radius -> f32,
        get_blur_radius, set_blur_radius -> f32,
        get_shadow_offset, set_shadow_offset -> Vec2,
        get_border_color, set_border_color -> String,
        get_border_width, set_border_width -> f32,
        get_border_radius, set_border_radius -> f32,
        get_cursor, set_cursor -> MouseCursor,
        get_visible, set_visible -> bool,
        get_grab_key_focus, set_grab_key_focus -> bool,
        get_animation_key, set_animation_key -> bool,
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
        get_event_key, set_event_key -> bool
    }
}

impl GButtonSet {
    set_event! {
        hover_in => GButtonHoverParam,
        hover_out => GButtonHoverParam,
        focus => GButtonFocusParam,
        focus_lost => GButtonFocusLostParam,
        clicked => GButtonClickedParam
    }
}
