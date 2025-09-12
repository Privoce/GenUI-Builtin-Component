use gen_ui::{
    components::*,
    inherits_view_livehook,
};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub ButtonPage = {{ButtonPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                <GButton> {}
                <GButton> {
                    style: {
                        basic: {
                            theme: Primary,
                        }
                    }
                }
                <GButton> {
                    style: {
                        basic: {
                            theme: Success,
                        }
                    }
                }
                <GButton> {
                    style: {
                        basic: {
                            theme: Info,
                        }
                    }
                }
                <GButton> {
                    style: {
                        basic: {
                            theme: Warning,
                        }
                    }
                }
                <GButton> {
                    style: {
                        basic: {
                            theme: Error,
                        }
                    }
                }
            }
            desc = {
                text: "Basic Button Component"
            }
        }
        // --------------------- others -------------------------------------------------------------
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GHLayout>{
                    style: {
                        basic: {
                            height: Fit,
                            width: Fill,
                            margin: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0},
                            padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                        }
                    }
                    <GButton> {
                        slot: {
                            text: "Only Text"
                        }
                    }
                    <GButton> {
                        style: {
                            basic: {
                                theme: Primary,
                            }
                        },
                        slot: <GHLayout>{
                            style: {
                                basic: {
                                    width: Fit,
                                    height: Fit,
                                    align: {
                                        x: 0.5,
                                        y: 0.5
                                    },
                                    margin: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0},
                                    padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                                },
                            }
                            <GSvg> {
                                style: {
                                    basic: {
                                        svg: {height: 18.0, width: 18.0, theme: Primary},
                                    }
                                },
                                src: dep("crate://self/resources/heavy.svg"),
                            }
                            <GLabel> {
                                text: "Bold text + Icon",
                                mode: Bold
                            }
                        }
                    }
                    <GButton> {
                        style: {
                            basic: {
                                theme: Success,
                            }
                        }
                        slot: <GSvg> {
                            style: {
                                basic: {
                                    svg: {height: 20.0, width: 20.0},
                                }
                            },
                            src: dep("crate://self/resources/heavy.svg"),
                        }
                    }
                    <GButton> {
                        style: {
                            basic: {
                                theme: Info,
                                border_radius: {left: 10.0, right: 10.0, top: 10.0, bottom: 10.0},
                                padding: {left: 10.0, right: 10.0, top: 10.0, bottom: 10.0},
                            }
                        }
                        slot: <GSvg> {
                                style: {
                                    basic: {
                                        svg: {height: 20.0, width: 20.0, theme: Primary},
                                    }
                                },
                                src: dep("crate://self/resources/heavy.svg"),
                            }
                    }
                }
                <GHLayout> {
                    style: {
                        basic: {
                            height: Fit,
                            width: Fill,
                            margin: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0},
                            padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                        }
                    },
                    <GButton> {
                        style: {
                            basic: {
                                theme: Info,
                                spread_radius: 4.0,
                                blur_radius: 4.0,
                            }
                        }
                        slot: {
                            text: "with shadow"
                        }
                    }
                    <GButton> {
                        style: {
                            basic: {
                                theme: Warning,
                                border_radius: {left: 10.0, right: 10.0, top: 10.0, bottom: 10.0}
                            }
                        },
                        slot: {
                            text: "Round Button"
                        }
                    }
                    <GButton> {
                        style: {
                            basic: {
                                theme: Error,
                                border_radius: {left: 4.0, right: 10.0, top: 6.0, bottom: 2.0}
                            }
                        },
                        slot: {
                            text: "Different Radius"
                        }
                    }
                }
            }
            desc = {
                text: "Different Button Component"
            }
        }
        // --------------------- event handling ---------------------------------------------------------
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                ebtn = <GButton> {
                    slot: {
                        text: "None"
                    }
                }
                <GButton> {
                    disabled: true,
                    slot: {
                        text: "Disabled"
                    }
                }
            }
            desc = {
                text: "Button Event"
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct ButtonPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ButtonPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ButtonPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for ButtonPage {
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

widget_node!(ButtonPage);
