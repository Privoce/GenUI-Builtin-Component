use makepad_widgets::*;

use crate::{
    components::view::GView, event_option, getter, ref_actives, ref_area, ref_event_option,
    ref_getter_setter, ref_redraw_mut, ref_render, set_event, setter, themes::Themes,
    utils::ToBool,
};

use super::{
    event::{GRadioGroupEvent, GRadioGroupEventParam},
    GRadioRef, GRadioWidgetRefExt,
};

live_design! {
    link gen_base;

    pub GRadioGroupBase = {{GRadioGroup}} {
        border_radius: 0.0,
        border_width: 0.0,
        spread_radius: 0.0,
        background_visible: false,
        height: Fit,
        width: Fit,
        animation_key: true,
        spacing: 8.0,
    }
}

#[derive(Live, Widget)]
pub struct GRadioGroup {
    #[deref]
    pub deref_widget: GView,
    #[live(-1)]
    pub selected: i32,
}

impl Widget for GRadioGroup {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.is_visible() {
            return;
        }
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        };
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let mut flag = None;
        let mut selected = 0_i32;
        let mut e = None;
        // try only do less to control event loop
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gradio().borrow().map(|radio| {
                if let Some(param) = radio.clicked(&actions) {
                    if param.selected {
                        if (index as i32).ne(&self.selected) {
                            selected = index as i32;
                            flag.replace(param.value);
                        } else {
                            flag = None;
                        }
                        e.replace(param.e);
                    }
                }
            });
            // if flag is true break to stop
            if flag.is_some() {
                break;
            }
        }
        if let Some(value) = flag {
            self.set_selected(cx, selected);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                GRadioGroupEvent::Changed(GRadioGroupEventParam {
                    selected,
                    e: e.unwrap(),
                    value,
                }),
            );
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GRadioGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.selected < 0 {
            let _ = self.find_selected();
        } else {
            self.set_selected(cx, self.selected);
        }
    }
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        self.deref_widget.after_apply_from_doc(cx);
    }
}

impl GRadioGroup {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: i32) -> () {
        self.selected = selected;

        // loop all gradio child and let selected == false except self.selected is true
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gradio().borrow_mut() {
                    child.toggle(cx, index as i32 == selected);
                } else {
                    panic!("GRadioGroup only allows GRadio as child!");
                }
            });
    }
    fn find_selected(&mut self) -> () {
        let mut flag = false;
        let mut selected = 0;
        let _ = self
            .children
            .iter()
            .map(|(_id, child)| {
                if let Some(child) = child.as_gradio().borrow() {
                    child.selected
                } else {
                    panic!("GRadioGroup only allows GRadio as child!");
                }
            })
            .enumerate()
            .for_each(|(index, is_selected)| {
                if is_selected && flag {
                    selected = index;
                    flag = true;
                } else if is_selected && !flag {
                    panic!(
                        "In GRadioGroup only allows one radio be selected! The Second is: {}",
                        index
                    );
                }
            });

        if flag {
            self.selected = selected as i32;
        }
    }
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn get(&self, index: usize) -> Option<(LiveId, GRadioRef)> {
        self.children
            .get(index)
            .map(|(id, child)| (id.clone(), child.as_gradio()))
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        self.deref_widget.render(cx)
    }
    event_option! {
        changed: GRadioGroupEvent::Changed => GRadioGroupEventParam
    }
    pub fn active_selected(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        let value = self
            .get(self.selected as usize)
            .and_then(|(_, child)| child.get_value());

        if let Some(path) = self.scope_path.as_ref() {
            cx.widget_action(
                self.widget_uid(),
                path,
                GRadioGroupEvent::Changed(GRadioGroupEventParam {
                    selected: self.selected,
                    value,
                    e,
                }),
            );
        }
    }
    /// Change the selected radio by index. It will call the changed event.
    pub fn change(&mut self, cx: &mut Cx, index: i32) {
        if index as usize >= self.children.len() {
            panic!("Index out of range!");
        }

        self.set_selected(cx, index);
        self.active_selected(cx, None);
    }
    setter! {
        GRadioGroup{
            set_theme(theme: Themes) {|c, cx| {c.theme = theme; c.render(cx)}},
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
        GRadioGroup{
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
            get_event_key(bool) {|c| {c.event_key}},
            get_selected(i32) {|c| {c.selected}}
        }
    }
}

impl GRadioGroupRef {
    ref_getter_setter! {
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
        get_event_key, set_event_key -> bool,
        get_selected, set_selected -> i32
    }
    ref_event_option! {
        changed => GRadioGroupEventParam
    }
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    pub fn get(&self, index: usize) -> Option<(LiveId, GRadioRef)> {
        self.borrow().map(|c_ref| c_ref.get(index)).flatten()
    }
    pub fn change(&self, cx: &mut Cx, index: i32) {
        self.borrow_mut().map(|mut c_ref| c_ref.change(cx, index));
    }
    ref_actives! {
        active_selected: Option<FingerUpEvent>
    }
}

impl GRadioGroupSet {
    set_event! {
        changed => GRadioGroupEventParam
    }
}
