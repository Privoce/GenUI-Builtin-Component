use luna_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::cbox::*;

    pub ImagePage = {{ImagePage}} {
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
                    prop: {
                    basic: {
                            height: Fit,
                            width: Fill,
                            flow: Down,
                        }
                    }
                    <GImage> {
                        src: Live(dep("crate://self/resources/install.png")),
                        prop: {
                            basic: {
                                height: 200.0,
                                width: 400.0,
                            }
                        }
                    }
                    <GLabel>{
                        text: "local image (relative)"
                    }
                }
                <GHLayout> {
                    prop: {
                    basic: {
                            height: Fit,
                            width: Fill,
                            flow: Down,
                        }
                    }
                    <GImage> {
                        src: File("/Users/shengyifei/projects/gen_ui/components/tests/resources/bg.png"),
                    }
                    <GLabel>{
                        text: "local image (absolute)"
                    }
                }
                <GHLayout> {
                    prop: {
                    basic: {
                            height: Fit,
                            width: Fill,
                            flow: Down,
                        }
                    }
                    <GImage> {
                        src: Url("https://aisearch.bj.bcebos.com/homepage/input_panel/aisearch_online.png"),
                        prop: {
                            basic: {
                                height: 80.0,
                                width: 160.0,
                            }
                        }
                    }
                    <GLabel>{
                        text: "url online"
                    }
                }
                <GHLayout> {
                    prop: {
                    basic: {
                            height: Fit,
                            width: Fill,
                            flow: Down,
                        }
                    }
                    <GImage> {
                        src: Url("https://miro.medium.com/v2/resize:fit:1200/0*fmpeXj1eUS-Nrkkv.png"),
                        prop: {
                            basic: {
                                height: 80.0,
                                width: 160.0,
                            }
                        }
                    }
                    <GLabel>{
                        text: "url online"
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
pub struct ImagePage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ImagePage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ImagePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for ImagePage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(ImagePage);
