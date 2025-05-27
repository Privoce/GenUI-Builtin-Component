use makepad_widgets::*;

use crate::{
    error::Error, getter, pure_after_apply, set_scope_path, themes::{Conf, Theme}
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
    pub theme: Theme,
    #[live]
    pub color: Vec4,
    #[live]
    pub font_size: f32,
    #[live]
    pub line_spacing: f32,
    #[live]
    pub visible: bool,
    #[walk]
    walk: Walk,
    #[live]
    align: Align,
    #[live(Flow::RightWrap)]
    flow: Flow,
    #[live]
    padding: Padding,
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
        let walk = walk.with_add_padding(self.padding);
        cx.begin_turtle(
            walk,
            Layout {
                flow: self.flow,
                ..Default::default()
            },
        );

        // here we need to check if the text is empty, if so we need to set it to a space
        // or the text draw will not work(seems like lazy drawtext bug)
        let _ = self.text.as_ref().is_empty().then(|| {
            let _ = self.set_text(cx, " ");
        });

        self.draw_text
            .draw_walk(cx, walk, self.align, self.text.as_ref());
        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl LiveHook for LLabel {
    pure_after_apply!();
}

impl Component for LLabel {
    type Error = Error;

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let label_prop = &cx.global::<Conf>().components.label;
        // [sync from conf prop] -----------------------------------------------------
        self.theme = label_prop.theme;
        self.color = label_prop.color;
        self.font_size = label_prop.font_size;
        self.line_spacing = label_prop.line_spacing;
        self.walk.margin = label_prop.margin;
        self.padding = label_prop.padding;
        // [sync to draw_text] -------------------------------------------------------
        self.draw_text.color = self.color;
        self.draw_text.text_style.font_size = self.font_size;
        self.draw_text.text_style.line_spacing = self.line_spacing;
        Ok(())
    }

    fn area(&self) -> Area {
        self.area
    }

    set_scope_path!();
}

impl LLabel {
    getter!{
        LLabel{
            get_theme(Theme) {|c| {c.theme}}
        }
    }
}
