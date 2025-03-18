mod event;
pub mod register;

pub use event::*;
use makepad_widgets::*;
use nav_control::NavControl;

use crate::{
    getter, ref_getter_setter, setter, themes::Themes, utils::{BoolToF32, ToBool}
};

use super::{
    image::GImageWidgetExt,
    label::GLabelWidgetExt,
    tool_btn::{GOsType, GToolButtonWidgetExt},
    view::{GView, GViewWidgetExt},
};

live_design! {
    link gen_base;
    use link::shaders::*;

    pub GWindowBase = {{GWindow}}{
        border_radius: 0.0,
        background_visible: true,
        background_color: #22272F,
        flow: Down,
        window: {
            inner_size: vec2(1024, 768)
        },
        pass: {
            clear_color: #1F1E25
        },
        cursor: Default,
        mouse_cursor_size: vec2(20, 20),
        draw_cursor: {
            instance border_width: 1.5
            instance color: #ADBAC7,
            instance border_color: #ADBAC7

            fn get_color(self) -> vec4 {
                return self.color
            }

            fn get_border_color(self) -> vec4 {
                return self.border_color
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.move_to(1.0, 1.0);
                sdf.line_to(self.rect_size.x - 1.0, self.rect_size.y * 0.5)
                sdf.line_to(self.rect_size.x * 0.5, self.rect_size.y - 1.0)
                sdf.close_path();
                sdf.fill_keep(self.get_color())
                if self.border_width > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_width)
                }
                return sdf.result
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GWindow {
    #[live]
    pub os_type: Option<GOsType>,
    #[live]
    pub window: WindowHandle,
    #[live]
    pub pass: Pass,
    #[deref]
    pub deref_widget: GView,
    #[live]
    pub show_title: Option<bool>,
    #[live]
    pub show_icon: Option<bool>,
    #[rust(Texture::new(cx))]
    pub depth_texture: Texture,
    #[rust(DrawList2d::new(cx))]
    pub main_draw_list: DrawList2d,
    #[rust(Overlay::new(cx))]
    pub overlay: Overlay,
    #[live]
    pub cursor_draw_list: DrawList2d,
    #[live]
    pub draw_cursor: DrawQuad,
    #[live]
    pub last_mouse_pos: DVec2,
    #[live]
    pub mouse_cursor_size: DVec2,
    #[live]
    pub nav_control: NavControl,
    #[live]
    pub hide_caption_on_fullscreen: bool,
    #[rust]
    pub btns_width: f64,
    #[rust]
    pub pre_btns_width: f64,
    #[rust(0.5)]
    pub offset: f64,
    #[rust(true)]
    pub redraw_flag: bool,
    #[rust(OsType::Windows)]
    pub current_os: OsType,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GWindow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.begin(cx).is_not_redrawing() {
            self.draw_state.end();
            return DrawStep::done();
        }
        let _ = self.deref_widget.draw_walk(cx, scope, walk)?;
        self.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let handle_min = |w: &mut GWindow, cx: &mut Cx, id_min, id_max, id_close, actions| {
            if w.gtool_button(id_min).clicked(actions).is_some() {
                w.window.minimize(cx);
            }
            if w.gtool_button(id_max).clicked(actions).is_some() {
                if w.window.is_fullscreen(cx) {
                    w.window.restore(cx);
                } else {
                    w.window.maximize(cx);
                }
            }

            if w.gtool_button(id_close).clicked(actions).is_some() {
                w.window.close(cx);
            }
        };
        // ---------------------------------------------------------------------
        let uid = self.widget_uid();
        self.overlay.handle_event(cx, event);
        // self.deref_widget.handle_event(cx, event, scope);
        let is_for_other_window = match event {
            Event::WindowCloseRequested(ev) => ev.window_id != self.window.window_id(),
            Event::WindowClosed(ev) => {
                if ev.window_id == self.window.window_id() {
                    cx.widget_action(uid, &scope.path, WindowAction::WindowClosed)
                }
                true
            }
            Event::WindowGeomChange(ev) => {
                if ev.window_id == self.window.window_id() {
                    match cx.os_type() {
                        OsType::Macos => {
                            if self.hide_caption_on_fullscreen {
                                if ev.new_geom.is_fullscreen && !ev.old_geom.is_fullscreen {
                                    self.view(id!(caption_bar)).set_visible(cx, false);
                                    self.redraw(cx);
                                } else if !ev.new_geom.is_fullscreen && ev.old_geom.is_fullscreen {
                                    self.view(id!(caption_bar)).set_visible(cx, true);
                                    self.redraw(cx);
                                };
                            }
                        }
                        _ => (),
                    }
                    cx.widget_action(uid, &scope.path, WindowAction::WindowGeomChange(ev.clone()));
                    return;
                }
                true
            }
            Event::WindowDragQuery(dq) => {
                if dq.window_id == self.window.window_id() {
                    if self.view(id!(caption_bar)).is_visible() {
                        let size = self.window.get_inner_size(cx);

                        if dq.abs.y < 25. {
                            if dq.abs.x < size.x - 135.0 {
                                dq.response.set(WindowDragQueryResponse::Caption);
                            }
                            cx.set_cursor(MouseCursor::Default);
                        }
                        /*
                        if dq.abs.x < self.caption_size.x && dq.abs.y < self.caption_size.y {
                        }*/
                    }
                }
                true
            }
            Event::TouchUpdate(ev) => ev.window_id != self.window.window_id(),
            Event::MouseDown(ev) => ev.window_id != self.window.window_id(),
            Event::MouseMove(ev) => ev.window_id != self.window.window_id(),
            Event::MouseUp(ev) => ev.window_id != self.window.window_id(),
            Event::Scroll(ev) => ev.window_id != self.window.window_id(),
            _ => false,
        };

        if is_for_other_window {
            cx.widget_action(uid, &scope.path, WindowAction::EventForOtherWindow);
            return;
        } else {
            self.deref_widget.handle_event(cx, event, scope);
        }

        if let Event::Actions(actions) = event {
            match self.current_os {
                OsType::Windows => {
                    let _ = handle_min(
                        self,
                        cx,
                        id!(window_bar.win_btns_wrap.min),
                        id!(window_bar.win_btns_wrap.max),
                        id!(window_bar.win_btns_wrap.close),
                        &actions,
                    );
                }
                OsType::Macos => {
                    let _ = handle_min(
                        self,
                        cx,
                        id!(window_bar.mac_btns_wrap.min),
                        id!(window_bar.mac_btns_wrap.max),
                        id!(window_bar.mac_btns_wrap.close),
                        &actions,
                    );
                }
                OsType::LinuxDirect | OsType::LinuxWindow(_) => {
                    let _ = handle_min(
                        self,
                        cx,
                        id!(window_bar.linux_btns_wrap.min),
                        id!(window_bar.linux_btns_wrap.max),
                        id!(window_bar.linux_btns_wrap.close),
                        &actions,
                    );
                }
                _ => (),
            }
        }

        if let Event::ClearAtlasses = event {
            Cx2d::reset_fonts_atlas(cx);
            Cx2d::reset_icon_atlas(cx);
        }

        if let Event::MouseMove(ev) = event {
            if let OsType::LinuxDirect = cx.os_type() {
                // ok move our mouse cursor
                self.last_mouse_pos = ev.abs;
                self.draw_cursor.update_abs(
                    cx,
                    Rect {
                        pos: ev.abs,
                        size: self.mouse_cursor_size,
                    },
                )
            }
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GWindow {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.window.set_pass(cx, &self.pass);
        self.depth_texture = Texture::new_with_format(
            cx,
            TextureFormat::DepthD32 {
                size: TextureSize::Auto,
                initial: true,
            },
        );
        self.pass
            .set_depth_texture(cx, &self.depth_texture, PassClearDepth::ClearWith(1.0));
    }
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.current_os = cx.os_type().clone();

        // get user want os type
        let show =
            self.os_type
                .as_ref()
                .map_or((true, false, false), |g_os_type| match g_os_type {
                    GOsType::Mac => (false, true, false),
                    GOsType::Linux => (false, false, true),
                    _ => (true, false, false),
                });

        self.render(show);
    }
}

impl GWindow {
    pub fn render(&mut self, show: (bool, bool, bool)) -> () {
        match self.current_os {
            OsType::Windows => {
                // in windows: show icon and title on the left, window buttons are on the right
                self.show_icon(true);
                self.show_title(true);
                self.show_btns(id!(window_bar.win_btns_wrap), show.0);
                self.show_btns(id!(window_bar.mac_btns_wrap), show.1);
                self.show_btns(id!(window_bar.linux_btns_wrap), show.2);
            }
            OsType::Macos => {
                // in macos: do not show icon , show title on the center, window buttons are on the left
                self.show_icon(false);
                self.show_title(true);
                self.show_btns(id!(window_bar.win_btns_wrap), false);
                self.show_btns(id!(window_bar.mac_btns_wrap), false);
                self.show_btns(id!(window_bar.linux_btns_wrap), false);
            }
            OsType::LinuxDirect | OsType::LinuxWindow(_) => {
                // in linux: do not show icon, show  title on the center, window buttons are on the right
                self.show_icon(false);
                self.show_title(true);
                self.show_btns(id!(window_bar.win_btns_wrap), false);
                self.show_btns(id!(window_bar.mac_btns_wrap), false);
                self.show_btns(id!(window_bar.linux_btns_wrap), false);
            }
            _ => {}
        }
        self.show_icon.clone().take().map(|show| {
            self.show_icon(show);
        });

        self.show_title.clone().take().map(|show| {
            self.show_title(show);
        });
    }
    pub fn show_icon(&mut self, show: bool) {
        self.gimage(id!(window_bar.window_title.icon))
            .borrow_mut()
            .map(|mut img| {
                img.visible = show;
            });
    }
    pub fn show_title(&mut self, show: bool) {
        self.glabel(id!(window_bar.window_title.title))
            .borrow_mut()
            .map(|mut label| {
                label.visible = show;
            });
    }
    pub fn get_btns_width(&mut self, cx: &mut Cx) {
        let mut offset = None;

        match self.current_os {
            OsType::Windows => {
                self.gview(id!(window_bar.win_btns_wrap)).borrow().map(|x| {
                    if let Size::Fixed(s) = x.walk.width {
                        self.btns_width = s;
                    } else {
                        self.btns_width = 138.0;
                    }
                });
            }
            OsType::Macos => {
                self.gview(id!(window_bar.mac_btns_wrap)).borrow().map(|x| {
                    if let Size::Fixed(s) = x.walk.width {
                        self.btns_width = s;
                    } else {
                        self.btns_width = 72.0;
                    }
                    offset.replace(DVec2 {
                        x: 0.0,
                        y: self.btns_width,
                    });
                });
            }
            OsType::LinuxDirect | OsType::LinuxWindow(_) => {
                self.gview(id!(window_bar.linux_btns_wrap))
                    .borrow()
                    .map(|x| {
                        if let Size::Fixed(s) = x.walk.width {
                            self.btns_width = s;
                        } else {
                            self.btns_width = 72.0;
                        }
                        offset.replace(DVec2 {
                            x: self.btns_width,
                            y: 0.0,
                        });
                    });
            }
            _ => {
                self.btns_width = 138.0;
            }
        }

        if self.btns_width != self.pre_btns_width {
            self.redraw_flag = true;
            // if is windows offset = 0.0
            let size = self.window.get_inner_size(cx);
            if let Some(offset) = offset {
                let align_x = if offset.x != 0.0 {
                    offset.x / size.x
                } else {
                    -offset.y / size.x
                };
                self.offset = 0.5 + align_x;
            } else {
                self.offset = 6.0 / size.x;
            }
        } else {
            self.redraw_flag = false;
        }
        self.pre_btns_width = self.btns_width;
    }
    pub fn show_btns(&mut self, id: &[LiveId], show: bool) {
        self.gview(id).borrow_mut().map(|mut x| {
            x.visible = show;
        });
    }
    pub fn begin(&mut self, cx: &mut Cx2d) -> Redrawing {
        if !cx.will_redraw(&mut self.main_draw_list, Walk::default()) {
            return Redrawing::no();
        }
        cx.begin_pass(&self.pass, None);
        self.main_draw_list.begin_always(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        self.overlay.begin(cx);
        Redrawing::yes()
    }
    pub fn end(&mut self, cx: &mut Cx2d) {
        // only linux needs to draw the cursor here
        if let OsType::LinuxDirect = cx.os_type() {
            self.cursor_draw_list.begin_overlay_last(cx);
            self.draw_cursor.draw_abs(
                cx,
                Rect {
                    pos: self.last_mouse_pos,
                    size: self.mouse_cursor_size,
                },
            );
            self.cursor_draw_list.end(cx);
        }

        self.overlay.end(cx);
        cx.end_pass_sized_turtle();
        self.main_draw_list.end(cx);
        cx.end_pass(&self.pass);
    }
    pub fn redraw(&mut self, cx: &mut Cx) -> () {
        self.deref_widget.redraw(cx);
    }
    pub fn handle_window_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let Some(action) = action.as_widget_action() {
                if let GWindowEvent::Minimize = action.cast() {
                    self.minimize(cx);
                } else if let GWindowEvent::Maximize = action.cast() {
                    self.maximize(cx);
                }
            }
        }
    }
    pub fn api(&self) -> &WindowHandle {
        &self.window
    }
    pub fn open(&mut self, cx: &mut Cx) -> () {
        self.visible = true;
        self.redraw(cx);
    }
    pub fn close(&mut self, cx: &mut Cx) -> () {
        self.window.close(cx);
    }
    pub fn maximize(&mut self, cx: &mut Cx) -> () {
        self.window.maximize(cx);
    }
    pub fn minimize(&mut self, cx: &mut Cx) -> () {
        self.window.minimize(cx);
    }
    pub fn fullscreen(&mut self, cx: &mut Cx) -> () {
        self.window.fullscreen(cx);
    }
    pub fn can_fullscreen(&mut self, cx: &mut Cx) -> bool {
        self.window.can_fullscreen(cx)
    }
    pub fn is_fullscreen(&mut self, cx: &mut Cx) -> bool {
        self.window.is_fullscreen(cx)
    }
    pub fn size(&self, cx: &mut Cx) -> DVec2 {
        self.window.get_inner_size(cx)
    }
    pub fn pos(&self, cx: &mut Cx) -> DVec2 {
        self.window.get_position(cx)
    }
    setter! {
        GWindow{
            set_theme(theme: Themes) {|c, cx| {c.theme = theme; c.deref_widget.render(cx)}},
            set_background_color(color: Vec4) {|c, _cx| {c.draw_view.background_color = color; Ok(())}},
            set_shadow_color(color: Vec4) {|c, _cx| {c.draw_view.shadow_color = color; Ok(())}},
            set_hover_color(color: Vec4) {|c, _cx| {c.draw_view.hover_color = color; Ok(())}},
            set_focus_color(color: Vec4) {|c, _cx| {c.draw_view.focus_color = color; Ok(())}},
            set_border_color(color: Vec4) {|c, _cx| {c.draw_view.border_color = color; Ok(())}},
            set_border_width(width: f32) {|c, _cx| {c.draw_view.border_width = width ; Ok(())}},
            set_border_radius(radius: f32) {|c, _cx| {c.draw_view.border_radius = radius ; Ok(())}},
            set_shadow_offset(offset: Vec2) {|c, _cx| {c.draw_view.shadow_offset = offset; Ok(())}},
            set_spread_radius(radius: f32) {|c, _cx| {c.draw_view.spread_radius = radius ; Ok(())}},
            set_blur_radius(radius: f32) {|c, _cx| {c.draw_view.blur_radius = radius ; Ok(())}},
            set_background_visible(visible: bool) {|c, _cx| {c.draw_view.background_visible = visible.to_f32(); Ok(())}},
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
            set_os_type(os_type: GOsType) {|c, _cx| {c.os_type.replace(os_type); Ok(())}},
            set_show_title(show_title: bool) {|c, _cx| {c.show_title.replace(show_title); Ok(())}},
            set_show_icon(show_icon: bool) {|c, _cx| {c.show_icon.replace(show_icon); Ok(())}},
            set_btns_width(btns_width: f64) {|c, _cx| {c.btns_width = btns_width; Ok(())}},
            set_pre_btns_width(pre_btns_width: f64) {|c, _cx| {c.pre_btns_width = pre_btns_width; Ok(())}},
            set_offset(offset: f64) {|c, _cx| {c.offset = offset; Ok(())}}
        }
    }
    getter! {
        GWindow{
            get_theme(Themes) {|c| {c.theme}},
            get_background_color(Vec4) {|c| {c.draw_view.background_color}},
            get_shadow_color(Vec4) {|c| {c.draw_view.shadow_color}},
            get_hover_color(Vec4) {|c| {c.draw_view.hover_color}},
            get_focus_color(Vec4) {|c| {c.draw_view.focus_color}},
            get_border_color(Vec4) {|c| {c.draw_view.border_color}},
            get_border_width(f32) {|c| {c.draw_view.border_width}},
            get_border_radius(f32) {|c| {c.draw_view.border_radius}},
            get_shadow_offset(Vec2) {|c| {c.draw_view.shadow_offset}},
            get_spread_radius(f32) {|c| {c.draw_view.spread_radius}},
            get_blur_radius(f32) {|c| {c.draw_view.blur_radius}},
            get_background_visible(bool) {|c| {c.draw_view.background_visible.to_bool()}},
            get_visible(bool) {|c| {c.visible}},
            get_cursor(MouseCursor) {|c| {c.cursor.unwrap_or_default()}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_block_signal_event(bool) {|c| {c.block_signal_event}},
            get_abs_pos(Option<DVec2>) {|c| {c.walk.abs_pos.clone()}},
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
            get_os_type(GOsType) {|c| {c.os_type.unwrap_or_default()}},
            get_show_title(bool) {|c| {c.show_title.unwrap_or(true)}},
            get_show_icon(bool) {|c| {c.show_icon.unwrap_or(false)}},
            get_pre_btns_width(f64) {|c| {c.pre_btns_width}},
            get_offset(f64) {|c| {c.offset}},
            get_current_os(OsType) {|c| {c.current_os.clone()}},
            get_event_key(bool) {|c| {c.event_key}}
        }
    }
}

impl GWindowRef {
    pub fn handle_window_actions(&mut self, cx: &mut Cx, actions: &Actions) -> () {
        if let Some(mut w) = self.borrow_mut() {
            w.handle_window_actions(cx, actions);
        }
    }
    pub fn open(&mut self, cx: &mut Cx) -> () {
        if let Some(mut w) = self.borrow_mut() {
            w.open(cx);
        }
    }
    pub fn close(&mut self, cx: &mut Cx) -> () {
        if let Some(mut w) = self.borrow_mut() {
            w.close(cx);
        }
    }
    pub fn minimize(&mut self, cx: &mut Cx) -> () {
        if let Some(mut w) = self.borrow_mut() {
            w.minimize(cx);
        }
    }
    ref_getter_setter!{
        get_theme, set_theme -> Themes,
        get_background_color, set_background_color -> Vec4,
        get_shadow_color, set_shadow_color -> Vec4,
        get_hover_color, set_hover_color -> Vec4,
        get_focus_color, set_focus_color -> Vec4,
        get_border_color, set_border_color -> Vec4,
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
        get_os_type, set_os_type -> GOsType,
        get_show_title, set_show_title -> bool,
        get_show_icon, set_show_icon -> bool,
        get_pre_btns_width, set_pre_btns_width -> f64,
        get_offset, set_offset -> f64,
        get_event_key, set_event_key -> bool
    }
}
