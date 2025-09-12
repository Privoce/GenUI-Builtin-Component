use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub CollapsePage = {{CollapsePage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                        spacing: 20.0,
                    }
                }
                <GCollapse> {}
                <GCollapse> {
                    active: true,
                    position: Top,
                    header: <GView> {
                        <GLabel> {
                            text: "Position: Top, active true"
                        }
                    }
                    body: <GView> {
                        <GLabel> {
                            text: "Collapse Body"
                        }
                    }
                } 
                <GCollapse> {
                    style: {
                        basic: {
                            header: {
                                theme: Primary,
                                width: 200.0,
                            },
                            body: {
                                theme: Primary
                            }
                        }
                    },
                    position: Left,
                    header: <GView> {
                        <GLabel> {
                            text: "Position: Left"
                        }
                    }
                    body: <GView> {
                        <GLabel> {
                            text: "Theme: Primary"
                        }
                    }
                }
                <GCollapse> {
                        style: {
                            basic: {
                                header: {
                                    theme: Info,
                                    width: 200.0,
                                },
                                body: {
                                    theme: Info,
                                    height: 300.0,
                                }
                            }
                        },
                        position: Right,
                        active: true,
                    }
            }
            desc = {
                text: ""
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct CollapsePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for CollapsePage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for CollapsePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for CollapsePage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {
    }
}

widget_node!(CollapsePage);
