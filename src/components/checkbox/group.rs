use makepad_widgets::*;

use crate::{
    components::view::GView, event_option, ref_getter, prop_setter, ref_actives, ref_area,
    ref_event_option, ref_redraw_mut, ref_render, set_event, themes::Themes, utils::ToBool,
};

use super::{
    event::{GCheckboxGroupEvent, GCheckboxGroupEventParam},
    GCheckboxRef, GCheckboxWidgetRefExt,
};

live_design! {
    link gen_base;
    pub GCheckboxGroupBase = {{GCheckboxGroup}} {
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
pub struct GCheckboxGroup {
    #[deref]
    pub deref_widget: GView,
    // selected indexs of checkbox, if selected.len() == 0, means no checkbox is selected
    #[live(vec![])]
    pub selected: Vec<i32>,
}

impl Widget for GCheckboxGroup {
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

        let e = match event.hits(cx, self.area()) {
            Hit::FingerUp(e) => Some(e),
            _ => None,
        };
        let mut flag = false;
        for (_, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gcheckbox().borrow().map(|checkbox| {
                if let Some(_) = checkbox.clicked(&actions) {
                    // here we just make sure the clicked is exist
                    flag = true;
                }
            });
            if flag {
                break;
            }
        }
        // ok, we know clicked happened, now we need to find selected
        if flag {
            self.find_selected();
            if let Some(path) = self.scope_path.as_ref() {
                let values = self.values();
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    GCheckboxGroupEvent::Changed(GCheckboxGroupEventParam {
                        selected: self.selected.iter().map(|x| *x as usize).collect(),
                        values,
                        e,
                    }),
                );
            }
        }
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GCheckboxGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.selected.len() == 0 {
            let _ = self.find_selected();
        } else {
            self.set_selected(cx, self.selected.clone());
        }
    }
    fn after_apply_from_doc(&mut self, cx:&mut Cx) {
        self.deref_widget.after_apply_from_doc(cx);
    }
}

impl GCheckboxGroup {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: Vec<i32>) -> () {
        // loop all gcheckbox child and let selected == false except self.selected is true
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gcheckbox().borrow_mut() {
                    let selected = &selected[index];

                    child.toggle(cx, *selected == index as i32);
                } else {
                    panic!("GCheckboxGroup only allows GCheckbox as child!");
                }
            });

        self.selected = selected;
    }
    fn find_selected(&mut self) -> () {
        self.selected = self.children.iter().enumerate().fold(
            Vec::new(),
            |mut selected, (index, (_, child))| {
                if let Some(child) = child.as_gcheckbox().borrow() {
                    if child.selected {
                        selected.push(index as i32);
                    }
                } else {
                    panic!("GCheckboxGroup only allows GCheckbox as child!");
                }
                selected
            },
        );
    }
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn get(&self, index: usize) -> Option<(LiveId, GCheckboxRef)> {
        self.children
            .get(index)
            .map(|(id, child)| (id.clone(), child.as_gcheckbox()))
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        self.deref_widget.render(cx)
    }
    pub fn active_selected(&mut self, cx: &mut Cx, e: Option<FingerUpEvent>) -> () {
        // for loop self.selected and get value
        let values = self.values();

        if let Some(path) = self.scope_path.as_ref() {
            cx.widget_action(
                self.widget_uid(),
                path,
                GCheckboxGroupEvent::Changed(GCheckboxGroupEventParam {
                    selected: self.selected.iter().map(|x| *x as usize).collect(),
                    values,
                    e,
                }),
            );
        }
    }
    /// Change the selected radio by index. It will call the changed event.
    pub fn change(&mut self, cx: &mut Cx, index: Vec<usize>) {
        if index.len() >= self.children.len() {
            panic!("Index out of range!");
        }

        self.set_selected(cx, index.iter().map(|x| *x as i32).collect());
        self.active_selected(cx, None);
    }
    pub fn values(&self) -> Vec<Option<String>> {
        self.selected
            .iter()
            .map(|x| {
                if let Some((_, checkbox)) = self.get(*x as usize) {
                    checkbox.value()
                } else {
                    // I am sure this will never happen but may be checkbox.value() can return None!
                    None
                }
            })
            .collect()
    }
    event_option! {
        changed: GCheckboxGroupEvent::Changed => GCheckboxGroupEventParam
    }
}

#[allow(dead_code)]
impl GCheckboxGroupRef {
    pub fn set_selected(&self, cx: &mut Cx, selected: Vec<i32>) -> () {
        self.borrow_mut()
            .map(|mut c_ref| c_ref.set_selected(cx, selected));
    }
    prop_setter! {
        GCheckboxGroup{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme; Ok(())}},
            set_background_color(color: String) {|c_ref| {c_ref.background_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_shadow_color(color: String) {|c_ref| {c_ref.shadow_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_hover_color(color: String) {|c_ref| {c_ref.hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_focus_color(color: String) {|c_ref| {c_ref.focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_color(color: String) {|c_ref| {c_ref.border_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_width(width: f64) {|c_ref| {c_ref.border_width = width as f32; Ok(())}},
            set_border_radius(radius: f64) {|c_ref| {c_ref.border_radius = radius as f32; Ok(())}},
            set_shadow_offset(offset: Vec2) {|c_ref| {c_ref.shadow_offset = offset; Ok(())}},
            set_spread_radius(radius: f64) {|c_ref| {c_ref.spread_radius = radius as f32; Ok(())}},
            set_blur_radius(radius: f64) {|c_ref| {c_ref.blur_radius = radius as f32; Ok(())}},
            set_background_visible(visible: bool) {|c_ref| {c_ref.background_visible = visible; Ok(())}},
            set_visible(visible: bool) {|c_ref| {c_ref.visible = visible; Ok(())}},
            set_cursor(cursor: MouseCursor) {|c_ref| {c_ref.cursor = Some(cursor); Ok(())}},
            set_grab_key_focus(grab: bool) {|c_ref| {c_ref.grab_key_focus = grab; Ok(())}},
            set_block_signal_event(block: bool) {|c_ref| {c_ref.block_signal_event = block; Ok(())}},
            set_abs_pos(pos: DVec2) {|c_ref| {c_ref.walk.abs_pos.replace(pos); Ok(())}},
            set_margin(margin: Margin) {|c_ref| {c_ref.walk.margin = margin; Ok(())}},
            set_height(height: Size) {|c_ref| {c_ref.walk.height = height; Ok(())}},
            set_width(width: Size) {|c_ref| {c_ref.walk.width = width; Ok(())}},
            set_scroll(scroll: DVec2) {|c_ref| {c_ref.layout.scroll = scroll; Ok(())}},
            set_clip_x(clip: bool) {|c_ref| {c_ref.layout.clip_x = clip; Ok(())}},
            set_clip_y(clip: bool) {|c_ref| {c_ref.layout.clip_y = clip; Ok(())}},
            set_padding(padding: Padding) {|c_ref| {c_ref.layout.padding = padding; Ok(())}},
            set_align(align: Align) {|c_ref| {c_ref.layout.align = align; Ok(())}},
            set_flow(flow: Flow) {|c_ref| {c_ref.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64) {|c_ref| {c_ref.layout.spacing = spacing; Ok(())}},
            set_dpi_factor(factor: f64) {|c_ref| {c_ref.dpi_factor.replace(factor); Ok(())}},
            set_optimize(optimize: ViewOptimize) {|c_ref| {c_ref.optimize = optimize; Ok(())}},
            set_capture_overload(overload: bool) {|c_ref| {c_ref.capture_overload = overload; Ok(())}},
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key; Ok(())}}
        }
    }
    ref_getter! {
        GCheckboxGroup{
            get_theme(Themes) {|| Themes::default()}, {|c_ref| {c_ref.theme}},
            get_background_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_view.background_color)}},
            get_shadow_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_view.shadow_color)}},
            get_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_view.hover_color)}},
            get_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_view.focus_color)}},
            get_border_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_view.border_color)}},
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
            get_selected(Vec<i32>) {||Vec::new()}, {|c_ref| {c_ref.selected.clone()}}
        }
    }
    ref_event_option! {
        changed => GCheckboxGroupEventParam
    }
    ref_area!();
    ref_redraw_mut!();
    ref_render!();
    pub fn get(&self, index: usize) -> Option<(LiveId, GCheckboxRef)> {
        self.borrow().map(|c_ref| c_ref.get(index)).flatten()
    }
    pub fn change(&self, cx: &mut Cx, index: Vec<usize>) {
        self.borrow_mut().map(|mut c_ref| c_ref.change(cx, index));
    }
    ref_actives! {
        active_selected: Option<FingerUpEvent>
    }
}

impl GCheckboxGroupSet {
    set_event! {
        changed => GCheckboxGroupEventParam
    }
}
