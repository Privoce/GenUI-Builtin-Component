use luna_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::cbox::*;

    pub SwitchPage = {{SwitchPage}} {
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GHLayout> {
                    prop: { basic: { height: Fit, width: Fill,}},
                    <GSwitch> {}
                    <GLabel> {
                        text: "basic"
                    }
                }
                <GHLayout> {
                    prop: { basic: { height: Fit, width: Fill,}},
                    <GSwitch>{
                        prop: {
                            basic: {
                                theme: Warning,
                            }
                        }
                        value: true
                    }
                    <GLabel> {
                        text: "active true"
                    }
                }
                <GHLayout> {
                    prop: { basic: { height: Fit, width: Fill,}},
                    <GSwitch>{
                        prop: {
                            basic: {
                                theme: Info,
                            }
                        }
                        disabled: true,
                    }
                    <GLabel> {
                        text: "disabled"
                    }
                }
                
            }
            desc = {
                text: ""
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct SwitchPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for SwitchPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SwitchPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for SwitchPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(SwitchPage);
