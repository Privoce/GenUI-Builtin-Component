use makepad_widgets::*;

use crate::{
    components::traits::Prop,
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

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct LLabel {
    #[live]
    pub prop: LabelProp,
    #[live(false)]
    pub disabled: bool,
    #[live(true)]
    pub visible: bool,
    #[rust]
    area: Area,
    #[live]
    text: ArcStringMut,
    // --- draw ------------------
    #[live]
    pub draw_text: DrawText,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
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

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
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
        Ok(())
    }

    fn current_state(&self) -> Self::State {
        if self.disabled {
            LabelState::Disabled
        } else {
            LabelState::None
        }
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    set_scope_path!();
    
    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
    }
    
    fn clear_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    fn switch_state_and_redraw(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }
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
