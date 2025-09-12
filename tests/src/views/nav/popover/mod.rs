use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub PopoverPage = {{PopoverPage}} {
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
                    <GPopover> {
                        position: TopLeft
                        <GButton> {
                            slot: {text: "TopLeft"}
                        }
                    }
                    <GPopover> {
                        position: Top
                        <GButton> {
                            slot: {text: "Top"}
                        }
                    }
                    <GPopover> {
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
                        <GPopover> {
                            position: LeftTop
                            <GButton> {
                                slot: {text: "LeftTop"}
                            }
                        }
                        <GPopover> {
                            position: Left
                            <GButton> {
                                slot: {text: "Left"}
                            }
                        }
                        <GPopover> {
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
                        <GPopover> {
                            position: RightTop
                            <GButton> {
                                slot: {text: "RightTop"}
                            }
                        }
                        <GPopover> {
                            position: Right
                            <GButton> {
                                slot: {text: "Right"}
                            }
                        }
                        <GPopover> {
                            position: RightBottom
                            <GButton> {
                                slot: {text: "RightBottom"}
                            }
                        }
                    }
                }
                <GHLayout> {
                    style: {basic: {height: 100.0, width: Fill, spacing: 60.0, align: {x:0.5}}},
                    <GPopover> {
                        position: BottomLeft
                        <GButton> {
                            slot: {text: "BottomLeft"}
                        }
                    }
                    <GPopover> {
                        position: Bottom
                        <GButton> {
                            slot: {text: "Bottom"}
                        }
                    }
                    <GPopover> {
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
pub struct PopoverPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for PopoverPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for PopoverPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for PopoverPage {
   
}

widget_node!(PopoverPage);
