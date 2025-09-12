use makepad_widgets::{shader::draw_text::TextStyle, *};

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Style},
    },
    error::Error,
    getter, getter_setter_ref, lifecycle,
    prop::{
        manuel::{BASIC, DISABLED},
        traits::ToColor,
        ApplyStateMap,
    },
    pure_after_apply, set_index, set_scope_path, setter, sync,
    themes::{conf::Conf, Theme},
    visible,
};

mod prop;

pub use prop::*;

use super::traits::Component;

live_design! {
    link genui_basic;
    use link::theme::*;
    pub GLabelBase = {{GLabel}} {
        font_regular: <THEME_FONT_REGULAR>{}
        font_bold: <THEME_FONT_BOLD>{}
        font_italic: <THEME_FONT_ITALIC>{}
        font_bold_italic: <THEME_FONT_BOLD_ITALIC>{}
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GLabel {
    #[live]
    pub style: LabelProp,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    #[live(FontMode::Regular)]
    pub mode: FontMode,
    // --- others ----------------
    #[rust]
    pub area: Area,
    #[live]
    pub text: ArcStringMut,
    #[rust]
    index: usize,
    #[rust]
    pub apply_state_map: ApplyStateMap<LabelState>,
    // --- fonts ----------------
    #[live]
    font_regular: TextStyle,
    #[live]
    font_bold: TextStyle,
    #[live]
    font_italic: TextStyle,
    #[live]
    font_bold_italic: TextStyle,
    // --- draw ------------------
    #[live]
    pub draw_text: DrawText,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    // --- init -----------------
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: LabelState,
    #[rust]
    is_set_disabled_color: bool,
}

impl WidgetNode for GLabel {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_text.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }
    fn animation_spread(&self) -> bool {
        // self.animation_spread
        false
    }

    visible!();
}

impl Widget for GLabel {
    /// calc font height: font_size + (6 * line_spacing), so default height is 19.2
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let state = self.state;
        let walk = walk.with_add_padding(self.style.get(state).padding);
        cx.begin_turtle(
            walk,
            Layout {
                flow: self.style.get(state).flow,
                ..Default::default()
            },
        );

        // // here we need to check if the text is empty, if so we need to set it to a space
        // // or the text draw will not work(seems like lazy drawtext bug)
        // let _ = self.text.as_ref().is_empty().then(|| {
        //     let _ = self.set_text(cx, " ");
        // });
        self.draw_text
            .draw_walk(cx, walk, Align::default(), self.text.as_ref());
        cx.end_turtle_with_area(&mut self.area);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        let _ = self.set_disabled(cx, disabled);
    }
    fn disabled(&self, _cx: &Cx) -> bool {
        self.disabled
    }
}

impl LiveHook for GLabel {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &LabelBasicStyle::live_props(),
            [live_id!(basic), live_id!(disabled)],
            |component| {
                component.lifecycle.next();
            },
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(LabelState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(LabelState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GLabel {
    type Error = Error;
    type State = LabelState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let label_prop = &cx.global::<Conf>().components.label;
        // [sync from conf prop] -----------------------------------------------------
        self.style = label_prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(LabelState::Disabled);
        }
        // dbg!(self.style.basic.color);
        // [sync to draw_text] -------------------------------------------------------
        let is_set_disabled_color = self.is_set_disabled_color();
        if !self.is_set_disabled_color {
            self.is_set_disabled_color = is_set_disabled_color;
        }
        let style = self.style.get_mut(self.state);
        if !self.is_set_disabled_color && self.disabled {
            style.sync(LabelState::Disabled);
        }
        self.draw_text.color = style.color;
        self.draw_text.text_style.font_size = style.font_size;
        self.draw_text.text_style.line_spacing = style.line_spacing;
        self.draw_text.text_style.font_family = match self.mode {
            FontMode::Regular => self.font_regular.font_family.clone(),
            FontMode::Bold => self.font_bold.font_family.clone(),
            FontMode::Italic => self.font_italic.font_family.clone(),
            FontMode::BoldItalic => self.font_bold_italic.font_family.clone(),
        };
        Ok(())
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
    }

    fn clear_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    fn switch_state_and_redraw(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state
    }
    fn switch_state_with_animation(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }
    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }
    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    sync!();
    set_index!();
    lifecycle!();
    set_scope_path!();
}

impl GLabel {
    pub fn is_set_disabled_color(&mut self) -> bool {
        self.apply_state_map
            .get(&LabelState::Disabled)
            .map(|applys| applys.contains_key("color"))
            .unwrap_or(false)
    }
    getter! {
        GLabel{
            get_theme(Theme) {|c| {c.style.basic.get_theme()}},
            get_color(String) {|c| {c.style.basic.get_color().to_hex_string()}},
            get_font_size(f32) {|c| {c.style.basic.get_font_size()}},
            get_line_spacing(f32) {|c| {c.style.basic.get_line_spacing()}},
            get_margin(Margin) {|c| {c.style.basic.get_margin()}},
            get_padding(Padding) {|c| {c.style.basic.get_padding()}},
            get_flow(Flow) {|c| {c.style.basic.get_flow()}},
            get_mode(FontMode) {|c| {c.mode}},
            get_text(String) {|c| {c.text.as_ref().to_string()}},
            get_visible(bool) {|c| {c.visible}},
            get_disabled(bool) {|c| {c.disabled}}
        }
    }
    setter! {
        GLabel{
            set_theme(theme: Theme) {|c, _cx| {c.style.basic.set_theme(theme); Ok(())}},
            set_color(color: String) {|c, _cx| {let color = Vec4::from_hex(&color)?; c.style.basic.set_color(color); Ok(())}},
            set_font_size(font_size: f32) {|c, _cx| {c.style.basic.set_font_size(font_size); Ok(())}},
            set_line_spacing(line_spacing: f32) {|c, _cx| {c.style.basic.set_line_spacing(line_spacing); Ok(())}},
            set_margin(margin: Margin) {|c, _cx| {c.style.basic.set_margin(margin); Ok(())}},
            set_padding(padding: Padding) {|c, _cx| {c.style.basic.set_padding(padding); Ok(())}},
            set_flow(flow: Flow) {|c, _cx| {c.style.basic.set_flow(flow); Ok(())}},
            set_mode(mode: FontMode) {|c, _cx| {c.mode = mode; Ok(())}},
            set_text(text: String) {|c, _cx| {c.text.as_mut_empty().push_str(&text); Ok(())}},
            set_visible(visible: bool) {|c, _cx| {c.visible = visible; Ok(())}},
            set_disabled(disabled: bool) {|c, cx| {c.disabled = disabled; c.redraw(cx); Ok(())}}
        }
    }
}

impl GLabelRef {
    getter_setter_ref! {
        get_theme, set_theme -> Theme,
        get_color, set_color -> String,
        get_font_size, set_font_size -> f32,
        get_line_spacing, set_line_spacing -> f32,
        get_margin, set_margin -> Margin,
        get_padding, set_padding -> Padding,
        get_flow, set_flow -> Flow,
        get_mode, set_mode -> FontMode,
        get_text, set_text -> String,
        get_visible, set_visible -> bool,
        get_disabled, set_disabled -> bool
    }
}
