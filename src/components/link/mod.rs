mod event;
pub mod register;
pub mod types;

pub use event::*;

use types::LinkType;

use crate::shader::draw_link::DrawGLink;
use crate::shader::draw_text::DrawGText;
use crate::themes::Themes;
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::open_browser;
use crate::utils::{get_font_family, set_cursor, BoolToF32, ThemeColor, ToBool};
use crate::{
    active_event, animatie_fn, default_handle_animation, default_hit_finger_down,
    default_hit_hover_in, default_hit_hover_out, event_option, play_animation, prop_getter,
    prop_setter, ref_area, ref_event_option, ref_redraw, ref_render, set_event, set_scope_path,
    set_text_and_visible_fn, widget_area,
};
use makepad_widgets::*;

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::*;

    pub GLinkBase = {{GLink}}{
        height: Fit,
        width: Fit,
        padding: 0,
        font_size: (FONT_SIZE),
        font_family: (FONT_FAMILY),
        align: <ALIGN_CENTER_WALK>{},
        text_walk: {
            height: Fit,
            width: Fit,
        },
        border_radius: 0.0,
        cursor: Hand,
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_link: {focus: 0.0, hover: 0.0}
                        draw_text: {focus: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        focus: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_link: {focus: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {focus: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                focus = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_link: {focus: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {focus: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

/// # GLink component
/// A GLink component is used to create interactive, styled links with hover, focus, and click events.
/// It supports animations and various customizable properties like color, text, and visibility.
///
/// ## Animation
/// The GLink component supports hover and focus animations, transitioning between different visual states.
/// - **hover.off**:
///     - `draw_link.hover`: 0.0 and `draw_text.hover`: 0.0
///     - `draw_link.focus`: 0.0 and `draw_text.focus`: 0.0
///     - Animation transition: uses Forward with a duration of 0.25s
/// - **hover.on**:
///    - `draw_link.hover`: 1.0 and `draw_text.hover`: 1.0
///    - `draw_link.focus`: 0.0 and `draw_text.focus`: 0.0
///    - Animation transition: uses Forward with a duration of 0.25s
/// - **hover.focus**:
///    - `draw_link.hover`: 1.0 and `draw_text.hover`: 1.0
///    - `draw_link.focus`: 1.0 and `draw_text.focus`: 1.0
///    - Animation transition: uses Forward with a duration of 0.25s
/// ## Event
/// GLink handles several user events such as hover and clicks.
/// - `HoverIn`: Triggered when the mouse starts hovering over the link.
/// - `HoverOut`: Triggered when the mouse stops hovering over the link.
/// - `Clicked`: Triggered when the link is clicked.
/// - `Focus`: Triggered when the link receives focus.
/// - `FocusLost`: Triggered when the link loses focus.
///
/// ## Props
/// |macro  |prop                    |description                                 |type              |default                |
/// |-------|------------------------|--------------------------------------------|------------------|-----------------------|
/// |live   |theme                   | Theme of the link                          |`Themes`          |`None`                 |
/// |live   |background_color         | Background color                           |`Option<Vec4>`    |`None`                 |
/// |live   |hover_color              | Hover background color                     |`Option<Vec4>`    |`None`                 |
/// |live   |focus_color              | Focus background color                     |`Option<Vec4>`    |`None`                 |
/// |live   |border_color             | Border color                               |`Option<Vec4>`    |`None`                 |
/// |live   |underline_visible        | Show underline when visible                |`bool`            |`true`                 |
/// |live   |underline_color          | Underline color                            |`Option<Vec4>`    |`None`                 |
/// |live   |underline_hover_color    | Underline color when hovering              |`Option<Vec4>`    |`None`                 |
/// |live   |underline_focus_color    | Underline color when focused               |`Option<Vec4>`    |`None`                 |
/// |live   |underline_width          | Width of the underline                     |`f32`             |`1.0`                  |
/// |live   |border_radius            | Border radius for rounding corners         |`f32`             |`4.0`                  |
/// |live   |round                    | Make the link round                        |`bool`            |`false`                |
/// |live   |background_visible       | Toggle visibility of the background        |`bool`            |`false`                |
/// |live   |text                     | The text content of the link               |`ArcStringMut`    |`""`                   |
/// |live   |font_size                | Size of the font                           |`f64`             |`10.0`                 |
/// |live   |color                    | Text color                                 |`Option<Vec4>`    |`None`                 |
/// |live   |text_hover_color         | Text color when hovered                    |`Option<Vec4>`    |`None`                 |
/// |live   |text_focus_color         | Text color when focused                    |`Option<Vec4>`    |`None`                 |
/// |live   |font_family              | Font family for the text                   |`LiveDependency`  |`None`                 |
/// |live   |cursor                   | Cursor style when hovering over the link   |`Option<MouseCursor>` |`None`             |
/// |live   |href                     | The URL for the link                       |`Option<String>`  |`None`                 |
/// |live   |link_type                | Type of link (internal, external, etc.)    |`LinkType`        |`None`                 |
/// |live   |visible                  | Visibility of the link                     |`bool`            |`true`                 |
/// |live   |draw_text                | Draw settings for text                     |`DrawGText`       |`None`                 |
/// |walk   |abs_pos                  | Absolute position for layout               |`Option<DVec2>`   |`None`                 |
/// |walk   |margin                   | Margin size around the view                |`Margin`          |`Margin::default()`    |
/// |walk   |width                    | Width of the view                          |`Size`            |`Size::default()`      |
/// |walk   |height                   | Height of the view                         |`Size`            |`Size::default()`      |
/// |layout |scroll                   | Scroll position for layout                 |`DVec2`           |`(0.0, 0.0)`           |
/// |layout |clip_x                   | Clip content horizontally                  |`bool`            |`true`                 |
/// |layout |clip_y                   | Clip content vertically                    |`bool`            |`true`                 |
/// |layout |padding                  | Padding within the view                    |`Padding`         |`Padding::default()`   |
/// |layout |align                    | Alignment for content                      |`Align`           |`Align::default()`     |
/// |layout |flow                     | Flow direction of the content              |`Flow`            |`Flow::default()`      |
/// |layout |spacing                  | Spacing between elements                   |`f64`             |`0.0`                  |
#[derive(Live, Widget)]
pub struct GLink {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(true)]
    pub underline_visible: bool,
    #[live]
    pub underline_color: Option<Vec4>,
    #[live]
    pub underline_hover_color: Option<Vec4>,
    #[live]
    pub underline_focus_color: Option<Vec4>,
    #[live(1.0)]
    pub underline_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    #[live(false)]
    pub round: bool,
    #[live(false)]
    pub background_visible: bool,
    // text -----------------
    #[live]
    pub text: ArcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    pub cursor: Option<MouseCursor>,
    // href -------------------
    #[live]
    pub href: Option<String>,
    #[live]
    pub link_type: LinkType,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // define area -----------------
    #[live]
    pub draw_text: DrawGText,
    #[live]
    pub text_walk: Walk,
    #[live(true)]
    pub grab_key_focus: bool,
    // animator -----------------
    #[live(true)]
    pub animation_key: bool,
    #[animator]
    pub animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    pub draw_link: DrawGLink,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for GLink {
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
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let _ = self.set_scope_path(&scope.path);
        let _ = get_font_family(&self.font_family, cx, &mut self.draw_text.text_style.font);
        let _ = self.draw_link.begin(cx, walk, self.layout);
        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());

        self.draw_link.end(cx);
        DrawStep::done()
    }

    set_text_and_visible_fn!();
}

impl LiveHook for GLink {

    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("GLink render error: {:?}", e);
        }
    }
}

impl GLink {
    set_scope_path!();
    play_animation!();
    widget_area! {
        area, draw_link,
        area_text, draw_text
    }
    active_event! {
        active_hover_in: GLinkEvent::HoverIn |e: FingerHoverEvent| => GLinkHoverParam { e },
        active_hover_out: GLinkEvent::HoverOut |e: FingerHoverEvent| => GLinkHoverParam { e },
        active_focus: GLinkEvent::Focus |e: FingerDownEvent| => GLinkFocusParam { e },
        active_focus_lost: GLinkEvent::FocusLost |e: FingerUpEvent| => GLinkFocusLostParam { e }
    }
    pub fn active_clicked(&mut self, cx: &mut Cx, e: FingerUpEvent) {
        if self.event_key {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GLinkEvent::Clicked(GLinkClickedParam {
                        href: self.href.clone(),
                        ty: self.link_type,
                        e,
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: GLinkEvent::HoverIn => GLinkHoverParam,
        hover_out: GLinkEvent::HoverOut => GLinkHoverParam,
        focus: GLinkEvent::Focus => GLinkFocusParam,
        focus_lost: GLinkEvent::FocusLost => GLinkFocusLostParam,
        clicked: GLinkEvent::Clicked => GLinkClickedParam
    }
    pub fn clear_animation(&mut self, cx: &mut Cx) {
        self.draw_link.apply_over(
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
    }
    pub fn redraw(&self, cx: &mut Cx) -> () {
        self.draw_text.redraw(cx);
        self.draw_link.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // backgroud visible is true, means link act as a button, text color should be plain
        let (
            background_color,
            hover_color,
            focus_color,
            text_color,
            text_hover_color,
            text_focus_color,
            underline_color,
            underline_hover_color,
            underline_focus_color,
        ) = if self.background_visible {
            (
                self.background_color.get(self.theme, 500),
                self.hover_color.get(self.theme, 400),
                self.focus_color.get(self.theme, 600),
                self.color.get(self.theme, 50),
                self.text_hover_color.get(self.theme, 25),
                self.text_focus_color.get(self.theme, 100),
                self.underline_color.get(self.theme, 50),
                self.underline_hover_color.get(self.theme, 25),
                self.underline_focus_color.get(self.theme, 100),
            )
        } else {
            (
                self.background_color.get(self.theme, 500),
                self.hover_color.get(self.theme, 400),
                self.focus_color.get(self.theme, 600),
                self.color.get(self.theme, 500),
                self.text_hover_color.get(self.theme, 400),
                self.text_focus_color.get(self.theme, 600),
                self.underline_color.get(self.theme, 500),
                self.underline_hover_color.get(self.theme, 400),
                self.underline_focus_color.get(self.theme, 600),
            )
        };
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ is background_visible -------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ underline -------------------------------------------------
        let underline_visible = self.underline_visible.to_f32();
        // ------------------ round -----------------------------------------------------
        if self.round {
            self.border_radius = match self.walk.height {
                Size::Fixed(h) => (h * 0.25) as f32,
                Size::Fit => {
                    ((self.draw_text.text_style.font_size
                        + self.layout.padding.top
                        + self.layout.padding.bottom)
                        * 0.25) as f32
                }
                _ => panic!("round only support fixed and fit"),
            };
        }
        // apply over props to draw_link ----------------------------------------------
        self.draw_link.apply_over(
            cx,
            live! {
                background_color: (background_color),
                border_color: (border_color),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
                underline_visible: (underline_visible),
                underline_color: (underline_color),
                underline_width: (self.underline_width),
                underline_hover_color: (underline_hover_color),
                underline_focus_color: (underline_focus_color),
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                color: (text_color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
                text_style: {
                    font_size: (self.font_size),
                },
            },
        );

        Ok(())
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                hover: 1.0,
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                hover: 0.0,
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.clear_animation(cx);
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                focus: 1.0
            },
        );
    }
    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_text.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
        self.draw_link.apply_over(
            cx,
            live! {
                focus: 0.0
            },
        );
    }

    pub fn handle_widget_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        hit: Hit,
        focus_area: Area,
    ) {
        let uid = self.widget_uid();
        default_handle_animation!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                // if self.grab_key_focus {
                //     cx.set_key_focus(focus_area);
                // }
                // cx.widget_action(uid, &scope.path, GLinkEvent::Pressed(f_down.clone()));
                // self.animator_play(cx, id!(hover.focus));
                default_hit_finger_down!(self, cx, focus_area, e);
            }
            Hit::FingerHoverIn(e) => {
                // let _ = set_cursor(cx, self.cursor.as_ref());
                // self.animator_play(cx, id!(hover.on));
                // cx.widget_action(uid, &scope.path, GLinkEvent::Hover(h.clone()));
                default_hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                // self.animator_play(cx, id!(hover.off));
                default_hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.device.has_hovers() {
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.play_animation(cx, id!(hover.off));
                    }

                    let _ = self.href.as_ref().map(|x| {
                        #[cfg(not(target_arch = "wasm32"))]
                        open_browser(&x)
                    });

                    cx.widget_action(
                        uid,
                        &scope.path,
                        GLinkEvent::Clicked(GLinkClickedParam {
                            href: self.href.clone(),
                            ty: self.link_type,
                            e,
                        }),
                    );
                } else {
                    self.animator_play(cx, id!(hover.off));
                    cx.widget_action(
                        self.widget_uid(),
                        self.scope_path.as_ref().unwrap(),
                        GLinkEvent::FocusLost(GLinkFocusLostParam { e }),
                    );
                }
            }
            _ => (),
        }
    }
}

impl GLinkRef {
    prop_setter! {
        GLink{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme; Ok(())}},
            set_background_color(color: String) {|c_ref| {c_ref.background_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_hover_color(color: String) {|c_ref| {c_ref.hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_focus_color(color: String) {|c_ref| {c_ref.focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_color(color: String) {|c_ref| {c_ref.border_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_underline_visible(visible: bool) {|c_ref| {c_ref.underline_visible = visible; Ok(())}},
            set_underline_color(color: String) {|c_ref| {c_ref.underline_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_underline_hover_color(color: String) {|c_ref| {c_ref.underline_hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_underline_focus_color(color: String) {|c_ref| {c_ref.underline_focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_underline_width(width: f32) {|c_ref| {c_ref.underline_width = width; Ok(())}},
            set_border_radius(radius: f32) {|c_ref| {c_ref.border_radius = radius; Ok(())}},
            set_round(round: bool) {|c_ref| {c_ref.round = round; Ok(())}},
            set_background_visible(visible: bool) {|c_ref| {c_ref.background_visible = visible; Ok(())}},
            set_text(text: ArcStringMut) {|c_ref| {c_ref.text = text; Ok(())}},
            set_font_size(size: f64) {|c_ref| {c_ref.font_size = size; Ok(())}},
            set_color(color: String) {|c_ref| {c_ref.color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_text_hover_color(color: String) {|c_ref| {c_ref.text_hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_text_focus_color(color: String) {|c_ref| {c_ref.text_focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_font_family(font_family: LiveDependency) {|c_ref| {c_ref.font_family = font_family; Ok(())}},
            set_cursor(cursor: MouseCursor) {|c_ref| {c_ref.cursor.replace(cursor); Ok(())}},
            set_href(href: String) {|c_ref| {c_ref.href.replace(href); Ok(())}},
            set_link_type(link_type: LinkType) {|c_ref| {c_ref.link_type = link_type; Ok(())}},
            set_visible(visible: bool) {|c_ref| {c_ref.visible = visible; Ok(())}},
            set_text_walk(walk: Walk) {|c_ref| {c_ref.text_walk = walk; Ok(())}},
            set_grab_key_focus(grab_key_focus: bool) {|c_ref| {c_ref.grab_key_focus = grab_key_focus; Ok(())}},
            set_animation_key(animation_key: bool) {|c_ref| {c_ref.animation_key = animation_key; Ok(())}},
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key; Ok(())}},
            set_abs_pos(abs_pos: DVec2) {|c_ref| {c_ref.walk.abs_pos.replace(abs_pos); Ok(())}},
            set_margin(margin: Margin) {|c_ref| {c_ref.walk.margin = margin; Ok(())}},
            set_height(height: Size) {|c_ref| {c_ref.walk.height = height; Ok(())}},
            set_width(width: Size) {|c_ref| {c_ref.walk.width = width; Ok(())}},
            set_scroll(scroll: DVec2) {|c_ref| {c_ref.layout.scroll = scroll; Ok(())}},
            set_clip_x(clip_x: bool) {|c_ref| {c_ref.layout.clip_x = clip_x; Ok(())}},
            set_clip_y(clip_y: bool) {|c_ref| {c_ref.layout.clip_y = clip_y; Ok(())}},
            set_padding(padding: Padding) {|c_ref| {c_ref.layout.padding = padding; Ok(())}},
            set_align(align: Align) {|c_ref| {c_ref.layout.align = align; Ok(())}},
            set_flow(flow: Flow) {|c_ref| {c_ref.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64) {|c_ref| {c_ref.layout.spacing = spacing; Ok(())}},
            set_text_height(height: Size) {|c_ref| {c_ref.text_walk.height = height; Ok(())}},
            set_text_width(width: Size) {|c_ref| {c_ref.text_walk.width = width; Ok(())}},
            set_text_abs_pos(abs_pos: DVec2) {|c_ref| {c_ref.text_walk.abs_pos.replace(abs_pos); Ok(())}},
            set_text_margin(margin: Margin) {|c_ref| {c_ref.text_walk.margin = margin; Ok(())}}
        }
    }
    prop_getter! {
        GLink{
            get_theme(Themes) {|| Themes::default()}, {|c_ref| {c_ref.theme}},
            get_background_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_link.background_color)}},
            get_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_link.hover_color)}},
            get_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_link.focus_color)}},
            get_border_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_link.border_color)}},
            get_underline_visible(bool) {|| true}, {|c_ref| {c_ref.draw_link.underline_visible.to_bool()}},
            get_underline_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_link.underline_color)}},
            get_underline_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_link.underline_hover_color)}},
            get_underline_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_link.underline_focus_color)}},
            get_underline_width(f32) {|| 1.0}, {|c_ref| {c_ref.draw_link.underline_width}},
            get_border_radius(f32) {|| 4.0}, {|c_ref| {c_ref.draw_link.border_radius}},
            get_round(bool) {|| false}, {|c_ref| {c_ref.round}},
            get_background_visible(bool) {|| false}, {|c_ref| {c_ref.background_visible}},
            get_font_size(f64) {|| 10.0}, {|c_ref| {c_ref.font_size}},
            get_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_text.color)}},
            get_text_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_text.stroke_hover_color)}},
            get_text_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_text.stroke_focus_color)}},
            // get_font_family(LiveDependency) {|| LiveDependency::default()}, {|c_ref| {c_ref.font_family}},
            get_cursor(MouseCursor) {|| MouseCursor::default()}, {|c_ref| {c_ref.cursor.unwrap_or_default()}},
            get_href(Option<String>) {|| None}, {|c_ref| {c_ref.href.clone()}},
            get_link_type(LinkType) {|| LinkType::default()}, {|c_ref| {c_ref.link_type}},
            get_visible(bool) {|| true}, {|c_ref| {c_ref.visible}},
            get_text_height(Size) {|| Size::default()}, {|c_ref| {c_ref.text_walk.height}},
            get_text_width(Size) {|| Size::default()}, {|c_ref| {c_ref.text_walk.width}},
            get_text_abs_pos(Option<DVec2>) {|| None}, {|c_ref| {c_ref.walk.abs_pos}},
            get_text_margin(Margin) {|| Margin::default()}, {|c_ref| {c_ref.walk.margin}},
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
            get_grab_key_focus(bool) {||false}, {|c_ref| {c_ref.grab_key_focus}},
            get_animation_key(bool) {||true}, {|c_ref| {c_ref.animation_key}},
            get_event_key(bool) {||true}, {|c_ref| {c_ref.event_key}}
        }
    }
    ref_area!();
    ref_redraw!();
    ref_render!();
    ref_event_option! {
        hover_in => GLinkHoverParam,
        hover_out => GLinkHoverParam,
        focus => GLinkFocusParam,
        focus_lost => GLinkFocusLostParam,
        clicked => GLinkClickedParam
    }
    animatie_fn! {
        clear_animation,
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
}

impl GLinkSet {
    set_event! {
        hover_in => GLinkHoverParam,
        hover_out => GLinkHoverParam,
        focus => GLinkFocusParam,
        focus_lost => GLinkFocusLostParam,
        clicked => GLinkClickedParam
    }
}
