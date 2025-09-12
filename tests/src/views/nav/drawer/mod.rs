use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub DrawerPage = {{DrawerPage}} {
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
                <GDrawer> {
                    popup: {
                        popup: {
                            <GHLayout> {
                                style: {basic: {height: 80.0}}
                                <GVLayout> {
                                    style: {basic: {align: {x: 0.5}}}
                                    <IconPlus> {}
                                    <GLabel> {text: "Add to"}
                                }
                                <GVLayout> {
                                    style: {basic: {align: {x: 0.5}}}
                                    <IconClose> {}
                                    <GLabel> {text: "Close"}
                                }
                                <GVLayout> {
                                    style: {basic: {align: {x: 0.5}}}
                                    <IconHome> {}
                                    <GLabel> {text: "Home"}
                                }
                                <GVLayout> {
                                    style: {basic: {align: {x: 0.5}}}
                                    <IconGift> {}
                                    <GLabel> {text: "Gift"}
                                }
                            }
                            <GDivider> {}
                        }
                    }
                    <GButton> {slot: {text: "Bottom"}}
                }
                <GDrawer> {
                    popup: {
                        popup: {
                            <GButton> {
                                style: {
                                    basic: {theme: Success}
                                }
                            }
                        }
                    },
                    position: Top,
                    <GButton> {slot: {text: "Top"}}
                }
                <GDrawer> {
                    popup: {
                        popup: {
                            <GButton> {
                                style: {
                                    basic: {theme: Success}
                                }
                            }
                        }
                    },
                    position: Left,  
                    <GButton> {slot: {text: "Left"}}
                }
                <GDrawer> {
                    popup: {
                        popup: {
                            <GButton> {
                                style: {
                                    basic: {theme: Success}
                                }
                            }
                        }
                    },
                    position: Right,
                    <GButton> {slot: {text: "Right"}}
                }
            }
            desc = {
                text: ""
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct DrawerPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for DrawerPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for DrawerPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for DrawerPage {

}

widget_node!(DrawerPage);
