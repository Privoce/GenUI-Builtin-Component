use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub TooltipPage = {{TooltipPage}} {
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
                <GHLayout> {
                    style: {basic: {height: 100.0, width: Fill, spacing: 60.0, align: {x:0.5}}},
                    <GToolTip> {
                        position: TopLeft
                        <GButton> {
                            slot: {text: "TopLeft"}
                        }
                    }
                    <GToolTip> {
                        position: Top
                        <GButton> {
                            slot: {text: "Top"}
                        }
                    }
                    <GToolTip> {
                        position: TopRight
                        <GButton> {
                            slot: {text: "TopRight"}
                        }
                    }
                }
                <GHLayout>{
                    style: {basic: {height: 200.0, width: Fill}},
                    <GVLayout> {
                        style: {basic: {height: Fill, width: 180.0, spacing: 20.0, align: {y:0.5}}},
                        <GToolTip> {
                            position: LeftTop
                            <GButton> {
                                slot: {text: "LeftTop"}
                            }
                        }
                        <GToolTip> {
                            position: Left
                            <GButton> {
                                slot: {text: "Left"}
                            }
                        }
                        <GToolTip> {
                            position: LeftBottom
                            <GButton> {
                                slot: {text: "LeftBottom"}
                            }
                        }
                    }
                    <GView>{
                        style: {basic: {height: Fill}},
                    }
                    <GVLayout> {
                        style: {basic: {height: Fill, width: 180.0, spacing: 20.0, align: {y:0.5}}},
                        <GToolTip> {
                            position: RightTop
                            <GButton> {
                                slot: {text: "RightTop"}
                            }
                        }
                        <GToolTip> {
                            position: Right
                            <GButton> {
                                slot: {text: "Right"}
                            }
                        }
                        <GToolTip> {
                            position: RightBottom
                            <GButton> {
                                slot: {text: "RightBottom"}
                            }
                        }
                    }
                }
                <GHLayout> {
                    style: {basic: {height: 100.0, width: Fill, spacing: 60.0, align: {x:0.5}}},
                    <GToolTip> {
                        position: BottomLeft
                        <GButton> {
                            slot: {text: "BottomLeft"}
                        }
                    }
                    <GToolTip> {
                        position: Bottom
                        <GButton> {
                            slot: {text: "Bottom"}
                        }
                    }
                    <GToolTip> {
                        position: BottomRight
                        <GButton> {
                            slot: {text: "BottomRight"}
                        }
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
pub struct TooltipPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TooltipPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TooltipPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for TooltipPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {}
}

widget_node!(TooltipPage);
