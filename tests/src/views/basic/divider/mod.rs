use luna_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::cbox::*;

    pub DividerPage = {{DividerPage}} {
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                        spacing: 20.0,
                    }
                }
                <GDivider>{}
                <GDivider>{
                    prop: {basic: {theme: Primary}}
                }
                <GDivider>{
                    prop: {basic: {theme: Info}}
                }
                <GDivider>{
                    prop: {basic: {theme: Success}}
                }
                <GDivider>{
                    prop: {basic: {theme: Warning}}
                }
                <GDivider>{
                    prop: {basic: {theme: Error}}
                }
            }
            desc = {
                text: ""
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct DividerPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for DividerPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for DividerPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for DividerPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(DividerPage);
