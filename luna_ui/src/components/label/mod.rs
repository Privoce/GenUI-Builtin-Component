use makepad_widgets::*;

use crate::{
    error::Error,
    getter, pure_after_apply, set_scope_path,
    themes::{Conf, Theme},
};

mod prop;

pub use prop::*;

use super::traits::Component;

live_design! {
    link luna_basic;

    pub LLabelBase = {{LLabel}} {}
}

#[derive(Live, Widget)]
pub struct LLabel {
    #[live]
    pub prop: LabelProp,
    #[live(true)]
    pub visible: bool,
    #[rust]
    area: Area,
    #[live]
    text: ArcStringMut,
    // --- draw ------------------
    #[redraw]
    #[live]
    pub draw_text: DrawText,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl Widget for LLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let walk = walk.with_add_padding(self.prop.padding);
        cx.begin_turtle(
            walk,
            Layout {
                flow: self.prop.flow,
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
    
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
}

impl Component for LLabel {
    type Error = Error;
    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let label_prop = &cx.global::<Conf>().components.label;
        // [sync from conf prop] -----------------------------------------------------
        self.prop = label_prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        // [sync to draw_text] -------------------------------------------------------
        self.draw_text.color = self.prop.color;
        self.draw_text.text_style.font_size = self.prop.font_size;
        self.draw_text.text_style.line_spacing = self.prop.line_spacing;
        Ok(())
    }

    set_scope_path!();
}

impl LLabel {
    getter! {
        LLabel{
            get_theme(Theme) {|c| {c.prop.theme}}
        }
    }

    pub fn area(&self) -> Area {
        self.area
    }
}
