mod event;
pub mod register;

pub use event::*;
// use event::*;
use makepad_widgets::*;

use std::rc::Rc;

use crate::{
    getter, ref_area, ref_getter_setter, ref_redraw_mut, setter,
    shader::manual::{CloseMode, PopupMode, Position, TriggerMode},
    themes::Themes,
    utils::ToBool,
};
use icon_atlas::RefCell;

use super::{
    popup::{GPopup, GPopupContainer},
    view::GView,
};

live_design! {
    link gen_base;

    pub GDropDownBase = {{GDropDown}} {}
}

#[derive(Live, Widget)]
pub struct GDropDown {
    #[rust]
    pub mode: PopupMode,
    #[deref]
    pub deref_widget: GView,
    #[live]
    pub popup: Option<LivePtr>,
    #[live]
    pub position: Position,
    #[live]
    pub trigger_mode: TriggerMode,
    #[live]
    pub opened: bool,
    #[live(6.0)]
    pub offset: f32,
    #[live]
    pub offset_x: f32,
    #[live]
    pub offset_y: f32,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    /// if proportion > 1.0, we think that is height/width (dep on position)(TODO: fix this)
    #[live(0.4)]
    pub proportion: f32,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub close_mode: CloseMode,
    #[rust(true)]
    pub redraw_flag: bool,
}

#[derive(Default, Clone)]
pub struct PopupMenuGlobal {
    pub map: Rc<RefCell<ComponentMap<LivePtr, GPopup>>>,
}

impl LiveHook for GDropDown {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.popup.is_none() || !apply.from.is_from_doc() || !self.visible {
            return;
        }
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut global_map = global.map.borrow_mut();
        global_map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));
        let popup = self.popup.unwrap();
        let popup = global_map.get_or_insert(cx, popup, |cx| GPopup::new_from_ptr(cx, Some(popup)));
        self.close_mode = popup.close_mode;
        self.mode = popup.mode;
    }
}

impl Widget for GDropDown {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        cx.add_nav_stop(self.area(), NavRole::DropDown, Margin::default());

        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.begin(cx);
            match self.mode {
                PopupMode::Popup | PopupMode::ToolTip => {
                    let area = self.area().rect(cx);
                    let angle_offset = self.position.angle_offset(area.size);
                    popup_menu.draw_container(
                        cx,
                        scope,
                        Some(self.position),
                        angle_offset,
                        &mut self.redraw_flag,
                    );
                    let container = popup_menu.container_area().rect(cx);
                    let mut shift = match self.position {
                        Position::Bottom => DVec2 {
                            x: -container.size.x / 2.0 + area.size.x / 2.0,
                            y: area.size.y + self.offset as f64,
                        },
                        Position::BottomLeft => DVec2 {
                            x: 0.0,
                            y: area.size.y + self.offset as f64,
                        },
                        Position::BottomRight => DVec2 {
                            x: area.size.x - container.size.x,
                            y: area.size.y + self.offset as f64,
                        },
                        Position::Top => DVec2 {
                            x: 0.0 - area.size.x / 2.0,
                            y: -self.offset as f64 - container.size.y,
                        },
                        Position::TopLeft => DVec2 {
                            x: 0.0,
                            y: -self.offset as f64 - container.size.y,
                        },
                        Position::TopRight => DVec2 {
                            x: area.size.x - container.size.x,
                            y: -self.offset as f64 - container.size.y,
                        },
                        Position::Left => DVec2 {
                            x: -self.offset as f64 - container.size.x,
                            y: area.size.y / 2.0 - container.size.y / 2.0,
                        },
                        Position::LeftTop => DVec2 {
                            x: -self.offset as f64 - container.size.x,
                            y: 0.0,
                        },
                        Position::LeftBottom => DVec2 {
                            x: -self.offset as f64 - container.size.x,
                            y: 0.0 - container.size.y + area.size.y,
                        },
                        Position::Right => DVec2 {
                            x: area.size.x + self.offset as f64,
                            y: area.size.y / 2.0 - container.size.y / 2.0,
                        },
                        Position::RightTop => DVec2 {
                            x: area.size.x + self.offset as f64,
                            y: 0.0,
                        },
                        Position::RightBottom => DVec2 {
                            x: area.size.x + self.offset as f64,
                            y: 0.0 - container.size.y + area.size.y,
                        },
                    };

                    shift.x += self.offset_x as f64;
                    shift.y += self.offset_y as f64;

                    popup_menu.end(cx, scope, self.area(), shift);
                }

                PopupMode::Dialog => {
                    popup_menu.draw_container(cx, scope, None, 0.0, &mut false);
                    popup_menu.end(cx, scope, Area::Empty, DVec2::default());
                }
                PopupMode::Drawer => {
                    let _ = popup_menu.draw_container_drawer(
                        cx,
                        scope,
                        self.position,
                        self.proportion,
                        &mut self.redraw_flag,
                    );
                    popup_menu.end(cx, scope, Area::Empty, DVec2::default());
                }
            }
        }

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.handle_event_with(cx, event, scope, self.area());
            if let Event::MouseDown(e) = event {
                match self.mode {
                    PopupMode::Popup | PopupMode::ToolTip => {
                        let is_in = popup_menu.menu_contains_pos(cx, e.abs);
                        self.close_inner(cx, GDropDownToggleKind::Other, is_in);
                    }

                    PopupMode::Dialog | PopupMode::Drawer => {
                        let is_in = popup_menu.container_contains_pos(cx, e.abs);
                        self.close_inner(cx, GDropDownToggleKind::Other, is_in);
                    }
                }
                return;
            }
        }

        match event.hits_with_sweep_area(cx, self.area(), self.area()) {
            // template remove -------------------------------------------------------------------
            // Hit::KeyFocus(_) => {
            //     // self.animator_play(cx, id!(focus.on));
            // }
            // Hit::KeyFocusLost(k_e) => {
            //     // self.toggle_inner(cx, GDropDownToggleKind::KetFocusLost(k_e.clone()), false);
            //     // self.animator_play(cx, id!(hover.off));
            //     // self.draw_view.redraw(cx);
            // }
            // template remove -------------------------------------------------------------------
            Hit::FingerDown(e, _) => {
                cx.set_key_focus(self.area());
                if self.trigger_mode.is_press() {
                    self.open_inner(cx, GDropDownToggleKind::Press(e));
                }
            }
            Hit::FingerHoverIn(e, _) => {
                cx.set_cursor(MouseCursor::Hand);
                if self.trigger_mode.is_hover() {
                    self.open_inner(cx, GDropDownToggleKind::Hover(e));
                }
            }
            Hit::FingerHoverOut(f) => {
                cx.set_cursor(MouseCursor::Default);
                if self.trigger_mode.is_hover() {
                    self.close_inner(cx, GDropDownToggleKind::Hover(f), false);
                }
            }
            Hit::FingerUp(e) => {
                if e.is_over && self.trigger_mode.is_click() {
                    self.open_inner(cx, GDropDownToggleKind::Click(e));
                } else {
                    // focus lost
                    self.close_inner(cx, GDropDownToggleKind::Other, false);
                }
            }
            _ => {}
        }
    }
    fn visible(&self) -> bool {
        self.visible
    }
}

impl GDropDown {
    pub fn render(&mut self, _cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn area(&self) -> Area {
        self.deref_widget.area
    }
    pub fn changed(&mut self, actions: &Actions) -> Option<GDropDownChangedParam> {
        if let GDropDownEvent::Changed(e) = actions.find_widget_action(self.widget_uid()).cast() {
            Some(e)
        } else {
            None
        }
    }
    pub fn redraw(&mut self, cx: &mut Cx) -> () {
        self.deref_widget.redraw(cx);
    }
    pub fn open(&mut self, cx: &mut Cx) {
        self.open_inner(cx, GDropDownToggleKind::Other);
    }
    pub fn close(&mut self, cx: &mut Cx) {
        // this close is virtual close
        if !self.opened {
            return;
        }
        // we don't need to care close mode here
        self.opened = false;
        self.redraw(cx);
        cx.sweep_unlock(self.area());
        self.active_toggled(cx, GDropDownToggleKind::Other);
        self.redraw_flag = true;
    }
    pub fn toggle(&mut self, cx: &mut Cx) {
        if self.opened {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }
    /// open the popup only inner control
    fn open_inner(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind) {
        if self.opened {
            return;
        }
        self.opened = true;
        self.redraw(cx);
        cx.sweep_lock(self.area());
        self.active_toggled(cx, e_kind);
    }
    /// close the popup only inner control
    fn close_inner(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind, is_in: bool) {
        // here is a quick return to optimize
        if !self.opened {
            return;
        }
        let mut flag = false;
        match self.close_mode {
            CloseMode::Out => {
                if !is_in {
                    flag = true;
                }
            }
            CloseMode::Virtual => {
                flag = false;
            }
        }
        if flag {
            self.opened = false;
            self.redraw(cx);
            cx.sweep_unlock(self.area());
            self.active_toggled(cx, e_kind);
        }
        self.redraw_flag = true;
    }
    fn active_toggled(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind) {
        cx.widget_action(
            self.widget_uid(),
            self.scope_path.as_ref().unwrap(),
            GDropDownEvent::Changed(GDropDownChangedParam {
                e: e_kind,
                opened: self.opened,
            }),
        );
    }
    #[allow(dead_code)]
    fn toggle_inner(&mut self, cx: &mut Cx, e_kind: GDropDownToggleKind, is_in: bool) {
        // we should check the close mode to make sure the close is correct (but only when opened)
        if self.opened {
            self.close_inner(cx, e_kind, is_in);
        } else {
            // if not opened, we should open it
            self.open_inner(cx, e_kind);
        }
    }

    pub fn get<F>(&mut self, cx: &mut Cx, mut f: F) -> ()
    where
        F: FnMut(&mut Cx, &Self, &GPopupContainer),
    {
        let global = cx.global::<PopupMenuGlobal>().clone();
        let map = global.map.borrow_mut();
        let popup_menu = map.get(&self.popup.unwrap()).unwrap();
        let _ = f(cx, self, popup_menu.get());
    }
    pub fn get_mut<F>(&mut self, cx: &mut Cx, mut f: F) -> ()
    where
        F: FnMut(&mut Cx, &mut Self, &mut GPopupContainer),
    {
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut map = global.map.borrow_mut();
        let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
        let _ = f(cx, self, popup_menu.get_mut());
    }
    setter! {
        GDropDown{
            set_mode(mode: PopupMode) {|c, _cx| {c.mode = mode; Ok(())}},
            set_position(position: Position) {|c, _cx| {c.position = position; Ok(())}},
            set_trigger_mode(trigger_mode: TriggerMode) {|c, _cx| {c.trigger_mode = trigger_mode; Ok(())}},
            set_opened(opened: bool) {|c, _cx| {c.opened = opened; Ok(())}},
            set_offset(offset: f32) {|c, _cx| {c.offset = offset; Ok(())}},
            set_offset_x(offset_x: f32) {|c, _cx| {c.offset_x = offset_x; Ok(())}},
            set_offset_y(offset_y: f32) {|c, _cx| {c.offset_y = offset_y; Ok(())}},
            set_proportion(proportion: f32) {|c, _cx| {c.proportion = proportion; Ok(())}},
            set_close_mode(close_mode: CloseMode) {|c, _cx| {c.close_mode = close_mode; Ok(())}},
            set_theme(theme: Themes) {|c, _cx| {c.theme = theme; Ok(())}},
            set_background_color(color: String) {|c, _cx| {c.background_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_shadow_color(color: String) {|c, _cx| {c.shadow_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_hover_color(color: String) {|c, _cx| {c.hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_focus_color(color: String) {|c, _cx| {c.focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_color(color: String) {|c, _cx| {c.border_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_width(width: f64) {|c, _cx| {c.border_width = width as f32; Ok(())}},
            set_border_radius(radius: f64) {|c, _cx| {c.border_radius = radius as f32; Ok(())}},
            set_shadow_offset(offset: Vec2) {|c, _cx| {c.shadow_offset = offset; Ok(())}},
            set_spread_radius(radius: f64) {|c, _cx| {c.spread_radius = radius as f32; Ok(())}},
            set_blur_radius(radius: f64) {|c, _cx| {c.blur_radius = radius as f32; Ok(())}},
            set_background_visible(visible: bool) {|c, _cx| {c.background_visible = visible; Ok(())}},
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
            set_event_key(event_key: bool) {|c, _cx| {c.event_key = event_key; Ok(())}}
        }
    }
    getter! {
        GDropDown{
            get_mode(PopupMode) {|c| {c.mode}},
            get_position(Position) {|c| {c.position}},
            get_trigger_mode(TriggerMode) {|c| {c.trigger_mode}},
            get_opened(bool) {|c| {c.opened}},
            get_offset(f32)  {|c| {c.offset}},
            get_offset_x(f32) {|c| {c.offset_x}},
            get_offset_y(f32) {|c| {c.offset_y}},
            get_proportion(f32) {|c| {c.proportion}},
            get_close_mode(CloseMode) {|c| {c.close_mode}},
            get_theme(Themes) {|c| {c.theme}},
            get_background_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.background_color)}},
            get_shadow_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.shadow_color)}},
            get_hover_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.hover_color)}},
            get_focus_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.focus_color)}},
            get_border_color(String) {|c| {crate::utils::vec4_to_hex(&c.draw_view.border_color)}},
            get_border_width(f64) {|c| {c.draw_view.border_width as f64}},
            get_border_radius(f64) {|c| {c.draw_view.border_radius as f64}},
            get_shadow_offset(Vec2) {|c| {c.draw_view.shadow_offset}},
            get_spread_radius(f64) {|c| {c.draw_view.spread_radius as f64}},
            get_blur_radius(f64) {|c| {c.draw_view.blur_radius as f64}},
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
            get_event_key(bool) {|c| {c.event_key}}
        }
    }
}

impl GDropDownRef {
    ref_getter_setter! {
        get_mode, set_mode -> PopupMode,
        get_position, set_position -> Position,
        get_trigger_mode, set_trigger_mode -> TriggerMode,
        get_opened, set_opened -> bool,
        get_offset, set_offset -> f32,
        get_offset_x, set_offset_x -> f32,
        get_offset_y, set_offset_y -> f32,
        get_proportion, set_proportion -> f32,
        get_close_mode, set_close_mode -> CloseMode,
        get_theme, set_theme -> Themes,
        get_background_color, set_background_color -> String,
        get_shadow_color, set_shadow_color -> String,
        get_hover_color, set_hover_color -> String,
        get_focus_color, set_focus_color -> String,
        get_border_color, set_border_color -> String,
        get_border_width, set_border_width -> f64,
        get_border_radius, set_border_radius -> f64,
        get_shadow_offset, set_shadow_offset -> Vec2,
        get_spread_radius, set_spread_radius -> f64,
        get_blur_radius, set_blur_radius -> f64,
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
        get_event_key, set_event_key -> bool
    }
    ref_redraw_mut!();
    ref_area!();
    /// open the popup
    pub fn open(&mut self, cx: &mut Cx) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.open(cx);
        }
    }
    /// close the popup
    pub fn close(&mut self, cx: &mut Cx) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.close(cx);
        }
    }
    /// get the popup and inner container, the container is the real popup(you need to control)
    pub fn get<F>(&self, cx: &mut Cx, f: F) -> ()
    where
        F: FnMut(&mut Cx, &GDropDown, &GPopupContainer),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.get(cx, f);
        }
    }
    /// ## get as mut ref
    /// ```rust
    /// let mut pop = self.gdrop_down(id!(pop));
    /// pop.get_mut(cx, |cx, pop, container| {
    ///     let close = container.gbutton(id!(close));
    ///
    ///     if close.clicked(&actions).is_some() {
    ///         pop.close(cx);
    ///     }
    /// });
    /// ```
    pub fn get_mut<F>(&mut self, cx: &mut Cx, f: F) -> ()
    where
        F: FnMut(&mut Cx, &mut GDropDown, &mut GPopupContainer),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.get_mut(cx, f);
        }
    }
    /// ## toggle the popup
    /// If you don't know the state of the popup, you can use this method to toggle the popup
    ///
    /// This is a easy way to control the popup, and do not worry, open or close fn has been optimized
    pub fn toggle(&mut self, cx: &mut Cx) {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.toggle(cx);
        }
    }
    pub fn changed(&mut self, actions: &Actions) -> Option<GDropDownChangedParam> {
        if let Some(mut c_ref) = self.borrow_mut() {
            c_ref.changed(actions)
        } else {
            None
        }
    }
}
