use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub RadioPage = {{RadioPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GRadio>{
                    extra: {
                        <GLabel> {
                            text: "Basic Radio: Round "
                        }
                    }
                }
                <GRadio> {
                    style: {
                        basic: {
                            radio: {
                                theme: Warning,
                                mode: Cross,
                            }
                        }
                    }
                    extra: {
                        <GLabel>{
                            text: "Radio: Cross",
                        }
                    }
                }
                    
                <GRadio> {
                    style: {
                        basic: {
                            container: {
                                background_visible: true,
                                theme: Info,
                            },
                            radio: {
                                theme: Primary,
                                mode: Tick,
                            }
                        },
                        hover: {
                            radio: {
                                theme: Warning,
                            }
                        }
                    },
                    extra: {
                        <GLabel> {
                            text: "Radio: Tick act as radio button"
                        }
                    }
                }
                <GRadio> {
                    style: {
                        basic: {
                            container: {
                                background_visible: true,
                                theme: Error,
                            },
                        },
                    },
                    radio_visible: false,
                    extra: {
                        <GLabel> {
                            text: "Radio without radio act as button"
                        }
                    }
                }
            }
            desc = {
                text: "Radio: Radio let people select one option from a set of options"
            }
        }
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GRadioGroup> {
                    style: {
                        basic: {
                            flow: Down,
                            align: {
                                x: 0.0
                            }
                        }
                    }
                    active: "callisto",
                    <GRadio> {
                        style: {
                            basic: {
                                radio: {theme: Primary, mode: Round},
                                container: {spacing: 20.0}
                            }
                        }
                        value: "none",
                        extra: {
                            <GLabel> {
                                text: "None",
                            }
                        }
                    }
                    <GRadio> {
                        style: {
                            basic: {
                                radio: {theme: Primary, mode: Round},
                                container: {spacing: 20.0}
                            }
                        }
                        value: "callisto",
                        extra: {
                            <GLabel> {
                                text: "Callisto",
                            }
                        }
                    }
                    <GRadio> {
                        style: {
                            basic: {
                                radio: {theme: Primary, mode: Round},
                                container: {spacing: 20.0}
                            }
                        }
                        value: "ganymede",
                        extra: {
                            <GLabel> {
                                text: "Ganymede",
                            }
                        }
                    }
                    <GRadio> {
                        style: {
                            basic: {
                                radio: {theme: Primary, mode: Round},
                                container: {spacing: 20.0}
                            }
                        }
                        value: "luna",
                        extra: {
                            <GLabel> {
                                text: "Luna",
                            }
                        }
                    }
                }
            }
            desc = {
                text: "RadioGroup: A group of radio buttons"
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct RadioPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for RadioPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for RadioPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for RadioPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(RadioPage);
