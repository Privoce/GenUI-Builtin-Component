use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub CheckboxPage = {{CheckboxPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GCheckbox>{
                    extra: {
                        <GLabel> {
                            text: "Basic Checkbox: Round "
                        }
                    }
                }
                <GCheckbox> {
                    style: {
                        basic: {
                            checkbox: {
                                theme: Warning,
                                mode: Cross,
                            }
                        }
                    }
                    extra: {
                        <GLabel>{
                            text: "Checkbox: Cross",
                        }
                    }
                }
                    
                <GCheckbox> {
                    style: {
                        basic: {
                            container: {
                                background_visible: true,
                                theme: Info,
                            },
                            checkbox: {
                                theme: Primary,
                                mode: Tick,
                            }
                        },
                        hover: {
                            checkbox: {
                                theme: Warning,
                            }
                        }
                    },
                    extra: {
                        <GLabel> {
                            text: "Checkbox: Tick act as Checkbox button"
                        }
                    }
                }
                <GCheckbox> {
                    style: {
                        basic: {
                            container: {
                                background_visible: true,
                                theme: Error,
                            },
                        },
                    },
                    checkbox_visible: false,
                    extra: {
                        <GLabel> {
                            text: "Checkbox without Checkbox act as button"
                        }
                    }
                }
            }
            desc = {
                text: "Checkbox: Checkbox let people select one option from a set of options"
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
                <GCheckboxGroup>{
                        active: ["1", "2"],
                        style: {
                            basic: {
                                width: 400.0,
                                flow: Down,
                                align: {x: 0.0}
                            }
                        }
                        <GCheckbox>{
                            style: {
                                basic: {
                                    container: {
                                        width: Fill,
                                    }
                                    checkbox: {
                                        theme: Primary,
                                        mode: Tick,
                                    },
                                    extra: {
                                        width: Fill,
                                        align: {x: 0.0}
                                    }
                                }
                            },
                            value: "1",
                            reverse: true,
                            extra: {
                                <GLabel>{
                                    text: "Microphone access"
                                }
                            }
                        }
                        <GCheckbox>{
                            style: {
                                basic: {
                                    container: {
                                        width: Fill,
                                    }
                                    checkbox: {
                                        theme: Primary,
                                        mode: Tick,
                                    }
                                    extra: {
                                        width: Fill,
                                        align: {x: 0.0}
                                    }
                                }
                            },
                            value: "2",
                            reverse: true,
                            extra: {
                                <GLabel>{
                                    text: "Location access"
                                }
                            }
                        }
                        <GCheckbox>{
                            style: {
                                basic: {
                                    container: {
                                        width: Fill,
                                    }
                                    checkbox: {
                                        theme: Primary,
                                        mode: Tick,
                                    }
                                    extra: {
                                        width: Fill,
                                        align: {x: 0.0}
                                    }
                                }
                            },
                            value: "3",
                            reverse: true,
                            extra: {
                                <GLabel>{
                                    text: "Haptics"
                                }
                            }
                        }
                        <GCheckbox>{
                            style: {
                                basic: {
                                    container: {
                                        width: Fill,
                                    }
                                    checkbox: {
                                        theme: Primary,
                                        mode: Tick,
                                    }
                                    extra: {
                                        width: Fill,
                                        align: {x: 0.0}
                                    }
                                }
                            },
                            value: "4",
                            reverse: true,
                            extra: {
                                <GLabel>{
                                    text: "Location access"
                                }
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
pub struct CheckboxPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for CheckboxPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for CheckboxPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for CheckboxPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(CheckboxPage);
