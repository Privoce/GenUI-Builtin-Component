mod event;
pub mod register;

pub use event::*;

use makepad_widgets::*;

use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down, default_hit_finger_up, default_hit_hover_in, default_hit_hover_out, event_option, getter, play_animation, pure_after_apply, ref_area, ref_event_option, ref_getter_setter, ref_play_animation, ref_redraw, ref_render, render_after_apply, set_event, set_scope_path, setter, shader::draw_svg::DrawGSvg, themes::Themes, utils::{set_cursor, ThemeColor}, widget_area
};

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::GLOBAL_DURATION;

    pub GSvgBase = {{GSvg}}{
        width: 24.0,
        height: 24.0,
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_svg: {hover: 0.0, focus: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_svg: {hover: 1.0, focus: 0.0}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_svg: {hover: 0.0, focus: 1.0}
                    }
                }
            }
        }
    }
}

/// # GSvg Component
/// The `GSvg` component is designed to handle scalable vector graphics (SVG) with hover and focus animations, supporting various visual properties such as stroke colors, scale, and cursor interactions.
///
/// ## Animation
/// The `GSvg` component utilizes an animator to handle hover and focus transitions, allowing smooth changes in visual states. Below are the animations defined for different states:
///
/// - **hover.off**:  
///   - `draw_svg.hover`: changes to `0.0`  
///   - `draw_svg.focus`: changes to `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
///
/// - **hover.on**:  
///   - `draw_svg.hover`: changes to `1.0`  
///   - `draw_svg.focus`: remains `0.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s` for both `hover` and `focus` states
///
/// - **hover.focus**:  
///   - `draw_svg.hover`: changes to `0.0`  
///   - `draw_svg.focus`: changes to `1.0`  
///   - Animation transition: uses `Forward` with a duration of `0.25s`
///
/// ## Event
/// The `GSvg` component can handle various events, allowing interaction with the user. It triggers events like `Clicked`, `HoverIn`, `HoverOut`, `Focus`, and `FocusLost` when corresponding actions occur.
///
/// ## Props
/// |macro  |prop                 |description                   |type            |default|
/// |-------|---------------------|-------------------------------|----------------|-------|
/// |live   |theme                |Theme settings                 |Themes          |       |
/// |live   |brightness           |Brightness level               |f32             |1.0    |
/// |live   |curve                |Curve intensity                |f32             |0.6    |
/// |live   |linearize            |Linearize factor               |f32             |0.5    |
/// |live   |src                  |SVG source dependency          |LiveDependency  |       |
/// |live   |scale                |Scaling factor                 |f64             |1.0    |
/// |live   |color                |SVG fill color                 |Option<Vec4>    |None   |
/// |live   |draw_depth           |Drawing depth                  |f32             |1.0    |
/// |live   |stroke_hover_color   |Color on hover                 |Option<Vec4>    |None   |
/// |live   |stroke_focus_color   |Color on focus                 |Option<Vec4>    |None   |
/// |live   |cursor               |Mouse cursor when hovered      |Option<MouseCursor>|None |
/// |live   |grab_key_focus       |Enable key focus grabbing      |bool            |true   |
/// |live   |visible              |Visibility of the component    |bool            |true   |
/// |live   |animation_key        |Triggers animation when true   |bool            |false  |
/// |animator|animator            |Handles animations             |Animator        |       |
/// |walk   |abs_pos              |Absolute position              |Option<DVec2>   |None   |
/// |walk   |margin               |Margin space                   |Margin          |       |
/// |walk   |width                |Component width                |Size            |       |
/// |walk   |height               |Component height               |Size            |       |
/// |layout |scroll               |Scroll position                |DVec2           |       |
/// |layout |clip_x               |Enable horizontal clipping     |bool            |true   |
/// |layout |clip_y               |Enable vertical clipping       |bool            |true   |
/// |layout |padding              |Padding around content         |Padding         |       |
/// |layout |align                |Alignment of content           |Align           |       |
/// |layout |flow                 |Flow direction of content      |Flow            |       |
/// |layout |spacing              |Spacing between elements       |f64             |       |
#[derive(Live, Widget)]
pub struct GSvg {
    #[live]
    pub theme: Themes,
    #[live(1.0)]
    pub brightness: f32,
    #[live(0.6)]
    pub curve: f32,
    #[live(0.5)]
    pub linearize: f32,
    #[live]
    pub src: LiveDependency,
    /// svg path command (todo!)
    // #[live]
    // pub command: Option<String>,
    #[live(1.0)]
    pub scale: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live(1.0)]
    pub draw_depth: f32,
    #[live]
    pub stroke_hover_color: Option<Vec4>,
    #[live]
    pub stroke_focus_color: Option<Vec4>,
    #[live]
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
    // deref -----------------
    #[redraw]
    #[live]
    pub draw_svg: DrawGSvg,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GSvg {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.draw_svg.draw_walk(cx, walk);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
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

        self.handle_widget_event(cx, event, scope, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
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

// impl LiveHook for GSvg {
//     fn after_apply_from_doc(&mut self, cx: &mut Cx) {
//         if !self.visible {
//             return;
//         }
//         if let Err(e) = self.render(cx) {
//             error!("GSvg render error: {:?}", e);
//         }
//     }
// }

impl LiveHook for GSvg{
    pure_after_apply!();
}

impl GSvg {
    render_after_apply!("GSvg");
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_svg
    }
    event_option! {
        clicked: GSvgEvent::Clicked => GSvgClickedParam,
        hover_in: GSvgEvent::HoverIn => GSvgHoverParam,
        hover_out: GSvgEvent::HoverOut => GSvgHoverParam,
        focus: GSvgEvent::Focus => GSvgFocusParam,
        focus_lost: GSvgEvent::FocusLost => GSvgFocusLostParam
    }
    active_event! {
        active_hover_in: GSvgEvent::HoverIn |e: FingerHoverEvent| => GSvgHoverParam{ e },
        active_hover_out: GSvgEvent::HoverOut |e: FingerHoverEvent| => GSvgHoverParam{ e },
        active_focus: GSvgEvent::Focus |e: FingerDownEvent| => GSvgFocusParam{ e },
        active_focus_lost: GSvgEvent::FocusLost |e: FingerUpEvent| => GSvgFocusLostParam{ e },
        active_clicked: GSvgEvent::Clicked |e: FingerUpEvent| => GSvgClickedParam{ e }
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // ------------------ hover color -----------------------------------------------
        let stroke_hover_color = self.stroke_hover_color.get(self.theme, 25);
        let stroke_focus_color = self.stroke_focus_color.get(self.theme, 50);
        // ------------------ color -----------------------------------------------
        let color = self.color.get(self.theme, 25);

        self.draw_svg.apply_over(
            cx,
            live! {
                stroke_hover_color: (stroke_hover_color),
                stroke_focus_color: (stroke_focus_color),
                color: (color),
                brightness: (self.brightness),
                curve: (self.curve),
                linearize: (self.linearize),
                scale: (self.scale),
                draw_depth: (self.draw_depth),
            },
        );

        self.draw_svg.set_src(self.src.clone());
        Ok(())
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_svg.redraw(cx);
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_svg.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_svg.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_svg.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_svg.apply_over(
            cx,
            live! {
                focus: 1.0,
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_svg.apply_over(
            cx,
            live! {
                focus: 0.0,
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
    setter! {
        GSvg{
            set_theme(theme: Themes){|c, cx| {c.theme = theme; c.render(cx)}},
            set_brightness(brightness: f32){|c, _cx| {c.brightness = brightness; c.draw_svg.brightness = brightness; Ok(())}},
            set_curve(curve: f32){|c, _cx| {c.curve = curve; c.draw_svg.curve = curve; Ok(())}},
            set_linearize(linearize: f32){|c, _cx| {c.linearize = linearize; c.draw_svg.linearize = linearize; Ok(())}},
            set_scale(scale: f64){|c, _cx| {c.scale = scale; c.draw_svg.scale = scale; Ok(())}},
            set_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.color.replace(color); c.draw_svg.color = color; Ok(())}},
            set_draw_depth(draw_depth: f32){|c, _cx| {c.draw_depth = draw_depth; c.draw_svg.draw_depth = draw_depth; Ok(())}},
            set_stroke_hover_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.draw_svg.stroke_hover_color = color; c.stroke_hover_color.replace(color); Ok(())}},
            set_stroke_focus_color(color: String){|c, _cx| {let color = crate::utils::hex_to_vec4(&color)?; c.draw_svg.stroke_focus_color = color; c.stroke_focus_color.replace(color); Ok(())}},
            set_cursor(cursor: MouseCursor){|c, _cx| {c.cursor.replace(cursor); Ok(())}},
            set_grab_key_focus(grab_key_focus: bool){|c, _cx| {c.grab_key_focus = grab_key_focus; Ok(())}},
            set_visible(visible: bool){|c, _cx| {c.visible = visible; Ok(())}},
            set_animation_key(animation_key: bool){|c, _cx| {c.animation_key = animation_key; Ok(())}},
            set_abs_pos(abs_pos: Option<DVec2>){|c, _cx| {c.walk.abs_pos = abs_pos; Ok(())}},
            set_margin(margin: Margin){|c, _cx| {c.walk.margin = margin; Ok(())}},
            set_height(height: Size){|c, _cx| {c.walk.height = height; Ok(())}},
            set_width(width: Size){|c, _cx| {c.walk.width = width; Ok(())}},
            set_scroll(scroll: DVec2){|c, _cx| {c.layout.scroll = scroll; Ok(())}},
            set_clip_x(clip_x: bool){|c, _cx| {c.layout.clip_x = clip_x; Ok(())}},
            set_clip_y(clip_y: bool){|c, _cx| {c.layout.clip_y = clip_y; Ok(())}},
            set_padding(padding: Padding){|c, _cx| {c.layout.padding = padding; Ok(())}},
            set_align(align: Align){|c, _cx| {c.layout.align = align; Ok(())}},
            set_flow(flow: Flow){|c, _cx| {c.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64){|c, _cx| {c.layout.spacing = spacing; Ok(())}},
            set_event_key(event_key: bool){|c, _cx| {c.event_key = event_key; Ok(())}}
        }
    }
    getter! {
        GSvg{
            get_theme(Themes) {|c| {c.theme}},
            get_brightness(f32) {|c| {c.brightness}},
            get_curve(f32) {|c| {c.curve}},
            get_linearize(f32) {|c| {c.linearize}},
            get_scale(f64) {|c| {c.scale}},
            get_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_svg.color)}},
            get_draw_depth(f32) {|c| {c.draw_svg.draw_depth}},
            get_stroke_hover_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_svg.stroke_hover_color)}},
            get_stroke_focus_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_svg.stroke_focus_color)}},
            get_cursor(MouseCursor) {|c| {c.cursor.unwrap_or_default()}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_visible(bool) {|c| {c.visible}},
            get_animation_key(bool) {|c| {c.animation_key}},
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
            get_event_key(bool) {|c| {c.event_key}}
        }
    }
}

impl GSvgRef {
    ref_getter_setter!{
        get_theme, set_theme -> Themes,
        get_brightness, set_brightness -> f32,
        get_curve, set_curve -> f32,
        get_linearize, set_linearize -> f32,
        get_scale, set_scale -> f64,
        get_color, set_color -> String,
        get_draw_depth, set_draw_depth -> f32,
        get_stroke_hover_color, set_stroke_hover_color -> String,
        get_stroke_focus_color, set_stroke_focus_color -> String,
        get_cursor, set_cursor -> MouseCursor,
        get_grab_key_focus, set_grab_key_focus -> bool,
        get_visible, set_visible -> bool,
        get_animation_key, set_animation_key -> bool,
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
        get_event_key, set_event_key -> bool
    }
    ref_redraw!();
    ref_render!();
    ref_area!();
    ref_event_option! {
        hover_in => GSvgHoverParam,
        hover_out => GSvgHoverParam,
        focus => GSvgFocusParam,
        focus_lost => GSvgFocusLostParam,
        clicked => GSvgClickedParam
    }
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
}

impl GSvgSet {
    set_event! {
        hover_in => GSvgHoverParam,
        hover_out => GSvgHoverParam,
        focus => GSvgFocusParam,
        focus_lost => GSvgFocusLostParam,
        clicked => GSvgClickedParam
    }
}
