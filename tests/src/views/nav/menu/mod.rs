use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub MenuPage = {{MenuPage}} {
        style: {basic: {padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}}}
        <CBox> {
            style: {
                basic: {
                    margin: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                    padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                }
            }
            show = {
                style: {
                    basic: {
                        height: Fill,
                        width: Fill,
                        flow: Down,
                        spacing: 10.0,
                        padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                    }
                }
                <GVLayout> {
                    style: {basic: {height: 300.0, width: Fill, padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}}}
                    <GMenuItem> {
                        text: <GLabel> {
                            text: "Menu item"
                        }
                    }
                    <GSubMenu>{
                        header: <GView> {
                            <GLabel>{
                                text: "Sub Menu",
                            }
                        }
                        body: <GView> {
                            <GMenuItem> {
                                text: <GLabel>{
                                    text: "Sub Menu Item 1",
                                }
                            }
                            <GMenuItem> {
                                text: <GLabel>{
                                    text: "Sub Menu Item 2",
                                }
                            }
                        }
                    }
                    <GSubMenu>{
                        header: <GView> {
                            <GLabel>{
                                text: "Sub Menu",
                            }
                        }
                        body: <GView> {
                            <GMenuItem> {
                                text: <GLabel>{
                                    text: "Sub Menu Item 1-1",
                                }
                            }
                            <GSubMenu>{
                                header: <GView> {
                                    <GLabel>{
                                        text: "Sub Menu",
                                    }
                                }
                                body: <GView> {
                                    <GMenuItem> {
                                        text: <GLabel>{
                                            text: "Sub Menu Item 2-1",
                                        }
                                    }
                                    <GMenuItem> {
                                        text: <GLabel>{
                                            text: "Sub Menu Item 2-2",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                <GHLayout> {
                    style: {basic: {padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}}}
                    <GMenu>{
                        body: {
                            <GSubMenu>{
                                header: <GView> {
                                    <GLabel>{
                                        text: "Sub Menu",
                                    }
                                }
                                body: <GView> {
                                    <GMenuItem> {
                                        text: <GLabel>{
                                            text: "Sub Menu Item 1-1",
                                        }
                                    }
                                    <GSubMenu>{
                                        header: <GView> {
                                            <GLabel>{
                                                text: "Sub Menu",
                                            }
                                        }
                                        body: <GView> {
                                            <GMenuItem> {
                                                text: <GLabel>{
                                                    text: "Sub Menu Item 2-1",
                                                }
                                            }
                                            <GMenuItem> {
                                                text: <GLabel>{
                                                    text: "Sub Menu Item 2-2",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                        <GMenu>{
                            style: {
                                basic: {
                                    container: {
                                        theme: Error,
                                    },
                                    body: {
                                        theme: Error,
                                    },
                                    footer: {theme: Error},
                                    header: {theme: Error}
                                }
                            },
                            active: "sub_active",
                            header: <GView> {
                                <GLabel>{
                                    text: "Menu Header",
                                }
                            },
                            body: {
                                <GSubMenu>{
                                    style: {
                                        basic: {
                                            container: {
                                                theme: Error,
                                            },
                                            header: {
                                                theme: Error,
                                            },
                                            body: {
                                                theme: Error,
                                            }
                                        }
                                    },
                                    body: {
                                        <GMenuItem> {
                                            style: {basic: {container: {theme: Error}}}
                                            text: <GLabel>{
                                                text: "Sub Menu Item 0-0",
                                            }
                                        }
                                        <GSubMenu>{
                                            style: {
                                                basic: {
                                                    container: {
                                                        theme: Error,
                                                    },
                                                    header: {
                                                        theme: Error,
                                                    },
                                                    body: {
                                                        theme: Error,
                                                    }
                                                }
                                            },
                                            header: <GView> {
                                                <GLabel>{
                                                    text: "Sub Menu",
                                                }
                                            }
                                            body: <GView> {
                                                <GMenuItem> {
                                                    style: {basic: {container: {theme: Error}}}
                                                    text: <GLabel>{
                                                        text: "Sub Menu Item 0-1-0",
                                                    }
                                                }
                                                <GMenuItem> {
                                                    style: {basic: {container: {theme: Error}}}
                                                    value: "sub_active",
                                                    text: <GLabel>{
                                                        text: "Sub Menu Item 0-1-1",
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                <GMenuItem> {
                                    style: {basic: {container: {theme: Error}}}
                                    text: <GLabel>{
                                        text: "Sub Menu Item 1",
                                    }
                                }
                            },
                            footer: <GView>{
                                <GLabel> {
                                    text: "Menu Footer"
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
pub struct MenuPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for MenuPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for MenuPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for MenuPage {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

widget_node!(MenuPage);
