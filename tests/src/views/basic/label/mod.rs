use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub LabelPage = {{LabelPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GLabel>{
                    text: "Normal Label"
                }
                <GLabel>{
                    text: "Bold Label",
                    mode: Bold
                }
                <GLabel>{
                    text: "Italic Label",
                    mode: Italic,
                }
                <GLabel>{
                    text: "BoldItalic Label",
                    mode: BoldItalic,
                }
                <GLabel>{
                    style: {
                        basic: {
                            font_size: 20.0,
                            color: #f00
                        }
                    },
                    text: "BoldItalic Label font size: 20.0, color: #f00",
                    mode: BoldItalic,
                }
                <GLabel>{
                    text: "Disabled",
                    mode: Bold,
                    disabled: true
                }
                <GLabel>{
                    style: {
                        basic: {
                            color: #ff0
                        }
                    },
                    text: "Disabled but set basic color (use disabled color)",
                    mode: Bold,
                    disabled: true
                }
                <GLabel>{
                    style: {
                        disabled: {
                            color: #ff0
                        }
                    },
                    text: "Disabled but set disabled color",
                    mode: Bold,
                    disabled: true
                }
            }
            desc = {
                text: ""
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct LabelPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for LabelPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for LabelPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for LabelPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(LabelPage);
