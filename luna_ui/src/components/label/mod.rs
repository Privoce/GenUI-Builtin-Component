use std::{collections::HashMap, sync::Arc};

use makepad_widgets::{
    makepad_live_compiler::LiveFont,
    shader::draw_text::{FontFamily, TextStyle},
    *,
};

use crate::{
    components::{lifecycle::LifeCycle, traits::Prop}, error::Error, getter, lifecycle, prop::{manuel::{BASIC, DISABLED}, ApplyStateMap}, pure_after_apply, set_index, set_scope_path, themes::{Conf, Theme}, utils::makepad_resource_dir
};

mod prop;

pub use prop::*;

use super::traits::Component;

live_design! {
    link luna_basic;
    use link::theme::*;
    pub LLabelBase = {{LLabel}} {
        font_regular: <THEME_FONT_REGULAR>{}
        font_bold: <THEME_FONT_BOLD>{}
        font_italic: <THEME_FONT_ITALIC>{}
        font_bold_italic: <THEME_FONT_BOLD_ITALIC>{}

    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct LLabel {
    #[live]
    pub prop: LabelProp,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    #[live]
    pub mode: FontMode,
    // --- others ----------------
    #[rust]
    area: Area,
    #[live]
    pub text: ArcStringMut,
    #[rust]
    index: usize,
    #[rust]
    apply_state_map: ApplyStateMap<LabelState>,
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
}

impl WidgetNode for LLabel {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let state = self.current_state();
        let prop = self.prop.get(state);
        Walk {
            abs_pos: Default::default(),
            margin: prop.margin,
            width: Size::Fit,
            height: Size::Fit,
        }
    }

    fn area(&self) -> Area {
        self.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_text.redraw(cx);
    }
}

impl Widget for LLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let state = self.current_state();
        let walk = walk.with_add_padding(self.prop.get(state).padding);
        cx.begin_turtle(
            walk,
            Layout {
                flow: self.prop.get(state).flow,
                ..Default::default()
            },
        );

        // here we need to check if the text is empty, if so we need to set it to a space
        // or the text draw will not work(seems like lazy drawtext bug)
        let _ = self.text.as_ref().is_empty().then(|| {
            let _ = self.set_text(cx, " ");
        });

        self.draw_text
            .draw_walk(cx, walk, Align::default(), self.text.as_ref());
        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl LiveHook for LLabel {
    pure_after_apply!();

    // fn after_apply_from_doc(&mut self, cx: &mut Cx) {
    //     self.render_after_apply(cx);
    // }

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if self.lifecycle.is_created() {
            self.index = index;
            self.lifecycle.next();
        }
        let live_props = [
            live_id!(theme),
            live_id!(color),
            live_id!(font_size),
            live_id!(line_spacing),
            live_id!(margin),
            live_id!(padding),
            live_id!(flow),
        ];
        for prefix in [live_id!(basic), live_id!(disabled)] {
            let mut applys = HashMap::new();
            for path in live_props {
                if let Some(i) = nodes.child_by_path(
                    index,
                    &[
                        live_id!(prop).as_field(),
                        prefix.as_field(),
                        path.as_field(),
                    ],
                ) {
                    let node = &nodes[i];
                    applys.insert(node.id.to_string(), node.value.clone());
                }
            }
            match prefix.to_string().as_str() {
                BASIC => {
                    self.apply_state_map.insert(LabelState::Basic, applys);
                }
                DISABLED => {
                    self.apply_state_map.insert(LabelState::Disabled, applys);
                }
                _ => {}
            }
        }
    }
}

impl Component for LLabel {
    type Error = Error;
    type State = LabelState;
    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let label_prop = &cx.global::<Conf>().components.label;
        // [sync from conf prop] -----------------------------------------------------
        self.prop = label_prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.current_state();
        // [sync to draw_text] -------------------------------------------------------
        self.draw_text.color = self.prop.get(state).color;
        self.draw_text.text_style.font_size = self.prop.get(state).font_size;
        self.draw_text.text_style.line_spacing = self.prop.get(state).line_spacing;
        self.draw_text.text_style.font_family = match self.mode {
            FontMode::Regular => self.font_regular.font_family.clone(),
            FontMode::Bold => self.font_bold.font_family.clone(),
            FontMode::Italic => self.font_italic.font_family.clone(),
            FontMode::BoldItalic => self.font_bold_italic.font_family.clone(),
        };
        Ok(())
    }

    fn current_state(&self) -> Self::State {
        if self.disabled {
            LabelState::Disabled
        } else {
            LabelState::Basic
        }
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

    fn switch_state(&mut self, _state: Self::State) -> () {
        ()
    }
    fn switch_state_with_animation(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }
    fn set_animation(&mut self, cx: &mut Cx) -> () {
        ()
    }
    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        self.prop.sync(&self.apply_state_map);
    }
    set_index!();
    lifecycle!();
    set_scope_path!();
}

impl LLabel {
    getter! {
        LLabel{
            get_theme(Theme) {|c| {let state = c.current_state(); c.prop.get(state).theme}}
        }
    }

    // pub fn area(&self) -> Area {
    //     self.area
    // }
}
