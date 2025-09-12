use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub LinkPage = {{LinkPage}} {
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
                <GLink>{
                    text: "Basic Link"
                }
                <GLink>{
                        style: {
                            basic: {
                                theme: Primary,
                                background_visible: true,
                            }
                        },
                        draw_text: {
                            color: #00f,
                        }
                        text: "Link with background",
                    }
                <GLink>{
                    style: {
                        basic: {
                            theme: Warning,
                            background_visible: true,
                            border_radius: {left: 8.0, right: 8.0, top: 8.0, bottom: 8.0},
                            padding: {left: 12.0, right: 12.0, top: 8.0, bottom: 8.0}
                        }
                    },
                    text: "Bold font, act as button",
                    mode: Bold
                }
                <GLink>{
                    style: {
                        basic: {
                            theme: Info,
                            underline_width: 2.0
                        }
                    },
                    text: "set underline width",
                    mode: Bold
                }
                <GLink> {
                    style: {
                        basic: {
                            theme: Error,
                            underline_visible: false,
                            font_size: 20.0
                        }
                    },
                    text: "set underline visible false",
                    mode: BoldItalic
                }
            }
            desc = {
                text: ""
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct LinkPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LinkPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LinkPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for LinkPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(LinkPage);
