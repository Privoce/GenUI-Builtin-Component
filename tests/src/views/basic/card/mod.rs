use luna_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::cbox::*;

    pub CardPage = {{CardPage}} {
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                <GCard>{}
                <GCard>{
                    prop: {
                        basic: {
                            container: {
                                width: 360.0,
                                theme: Info,
                            }
                        }
                    }
                    header: <GView>{
                        <GLabel> {
                            prop: {
                                basic: {
                                    font_size: 16.0
                                }
                            }
                            text: "Dan-Reyes",
                            mode: Bold
                        }
                    }
                    body: <GView>{
                        <GImage> {
                            src: Live(dep("crate://self/resources/cat.jpg")),
                            prop: {
                                basic: {
                                    height: Fill,
                                    width: Fill,
                                }
                            }
                        }
                    }
                    footer: <GView>{
                        prop: {
                            basic: {
                                align: {
                                    y: 0.5
                                }
                            }
                        }
                        <GLabel> {
                            text: "Star: 12k"
                        }
                        <GView> {
                            prop: {
                                basic: {
                                    height: Fit,
                                    align: {
                                        x: 1.0, y: 0.5
                                    }
                                    padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                                }
                            }
                            <GButton> {
                                prop: {
                                    basic: {
                                        theme: Primary,
                                    }
                                }
                                slot: {
                                    text: "Download"
                                }
                            }
                        }
                    }
                }
            }
            desc = {
                text: "The card has 3 slots, and by default only the background color of the outermost container is retained"
            }
        }
        // --------------------- others -------------------------------------------------------------
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }

            }
            desc = {
                text: "Different Button Component"
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct CardPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for CardPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for CardPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for CardPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let ebtn = self.gbutton(id!(ebtn));
        if let Some(_) = ebtn.clicked(actions) {
            ebtn.borrow_mut().map(|btn| {
                let _ = btn.slot.as_glabel().set_text(cx, "Clicked".to_string());
            });
        } else if let Some(_) = ebtn.hover_in(actions) {
            ebtn.borrow_mut().map(|btn| {
                let _ = btn.slot.as_glabel().set_text(cx, "Hover In".to_string());
            });
        } else if let Some(_) = ebtn.hover_out(actions) {
            ebtn.borrow_mut().map(|btn| {
                let _ = btn.slot.as_glabel().set_text(cx, "Hover Out".to_string());
            });
        } else if let Some(_) = ebtn.finger_down(actions) {
            ebtn.borrow_mut().map(|btn| {
                let _ = btn.slot.as_glabel().set_text(cx, "Finger Down".to_string());
            });
        } else if let Some(_) = ebtn.finger_up(actions) {
            ebtn.borrow_mut().map(|btn| {
                let _ = btn.slot.as_glabel().set_text(cx, "Finger Up".to_string());
            });
        }
    }
}

widget_node!(CardPage);
