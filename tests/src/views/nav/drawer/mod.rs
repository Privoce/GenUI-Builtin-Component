use luna_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub DrawerPage = {{DrawerPage}} {
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
                <GDrawer> {
                        popup: {
                            popup: {
                                <GButton> {
                                    prop: {
                                        basic: {theme: Success}
                                    }
                                }
                            }
                        }
                        <GButton> {}
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
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
    }
}

widget_node!(DrawerPage);
