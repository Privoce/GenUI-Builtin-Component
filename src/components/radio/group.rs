use makepad_widgets::*;

use crate::{
    components::view::GView, event_option, prop_getter, prop_setter, ref_actives, ref_area,
    ref_event_option, ref_redraw_mut, ref_render, set_event, themes::Themes, utils::ToBool,
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
        let mut selected = 0;
        let mut e = None;
        // try only do less to control event loop
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gradio().borrow().map(|radio| {
                if let Some(param) = radio.clicked(&actions) {
                    if param.selected {
                        if (index as i32).ne(&self.selected) {
                            selected = index;
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
            self.set_selected(cx, self.selected as usize);
        }
    }
}

impl GRadioGroup {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: usize) -> () {
        self.selected = selected as i32;

        // loop all gradio child and let selected == false except self.selected is true
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gradio().borrow_mut() {
                    child.toggle(cx, index == selected);
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
    pub fn render(&mut self, cx: &mut Cx) {
        self.deref_widget.render(cx);
    }
    event_option! {
        changed: GRadioGroupEvent::Changed => GRadioGroupEventParam
    }
    pub fn active_selected(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) {
        let value = self
            .get(self.selected as usize)
            .map(|(_, child)| child.value())
            .flatten();

        if let Some(path) = self.scope_path.as_ref() {
            cx.widget_action(
                self.widget_uid(),
                path,
                GRadioGroupEvent::Changed(GRadioGroupEventParam {
                    selected: self.selected as usize,
                    value,
                    e,
                }),
            );
        }
    }
    /// Change the selected radio by index. It will call the changed event.
    pub fn change(&mut self, cx: &mut Cx, index: usize) {
        if index >= self.children.len() {
            panic!("Index out of range!");
        }

        self.set_selected(cx, index);
        self.active_selected(cx, None);
    }
}

impl GRadioGroupRef {
    pub fn set_selected(&self, cx: &mut Cx, selected: usize) -> () {
        self.borrow_mut()
            .map(|mut c_ref| c_ref.set_selected(cx, selected));
    }
    prop_setter! {
        GRadioGroup{
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
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key;}}
        }
    }
    prop_getter! {
        GRadioGroup{
            get_theme(Themes) {|| Themes::default()}, {|c_ref| {c_ref.theme}},
            get_background_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.background_color}},
            get_shadow_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.shadow_color}},
            get_hover_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.hover_color}},
            get_focus_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.focus_color}},
            get_border_color(Vec4) {|| Vec4::default()}, {|c_ref| {c_ref.draw_view.border_color}},
            get_border_width(f64) {|| 0.0}, {|c_ref| {c_ref.draw_view.border_width as f64}},
            get_border_radius(f64) {|| 0.0}, {|c_ref| {c_ref.draw_view.border_radius as f64}},
            get_shadow_offset(Vec2) {|| Vec2::default()}, {|c_ref| {c_ref.draw_view.shadow_offset}},
            get_spread_radius(f64) {|| 0.0}, {|c_ref| {c_ref.draw_view.spread_radius as f64}},
            get_blur_radius(f64) {|| 0.0}, {|c_ref| {c_ref.draw_view.blur_radius as f64}},
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
            get_selected(i32) {||-1}, {|c_ref| {c_ref.selected}}
        }
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
    pub fn change(&self, cx: &mut Cx, index: usize) {
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
