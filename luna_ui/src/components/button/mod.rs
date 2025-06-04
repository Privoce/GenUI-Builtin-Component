mod prop;

pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::traits::Component, error::Error, pure_after_apply, set_scope_path,
    shader::draw_view::DrawView, themes::{Conf, Theme},
};

live_design! {
    link luna_basic;

    pub LButtonBase = {{LButton}}
}

#[derive(Live, Widget)]
pub struct LButton {
    #[live]
    pub prop: ButtonProp,
    // --- visible -------------------
    #[live]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    // --- others -------------------
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    // --- draw ----------------------
    #[find]
    #[redraw]
    #[live]
    pub slot: WidgetRef,
    #[redraw]
    #[live]
    pub draw_button: DrawView,
}

impl Widget for LButton {
    fn draw_walk(&mut self, _cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl LiveHook for LButton {
    // pure_after_apply!();
}

impl Component for LButton {
    type Error = Error;

    type State = ButtonState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.button;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        todo!()
    }

    fn current_state(&self) -> Self::State {
        if self.disabled {
            ButtonState::Disabled
        } else {
            self.draw_button.current_state().into()
        }
    }

    set_scope_path!();
}
