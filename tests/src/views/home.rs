use luna_ui::{
    components::*,
    inherits_view_livehook, inherits_view_widget_node,
};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::basic::view::*;
    use crate::views::basic::button::*;

    pub HomePage = {{HomePage}} {
        prop: {
            basic: {
                flow: Right,
                padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
            }
        }
        menu = <GMenu> {
            prop: {
                basic: {
                    container: {
                        theme: Primary,
                    },
                    body: {
                        theme: Primary,
                    }
                }
            },
            // active: "tab_view",
            body: {
                <GSubMenu> {
                    prop: {
                        basic: {
                            container: {
                                theme: Primary,
                            },
                            header: {
                                theme: Primary,
                            },
                            body: {
                                theme: Primary,
                            }
                        }
                    },
                    header: {
                        <GLabel> {
                            text: "Basic Components"
                        }
                    },
                    body: {
                        <GMenuItem> {
                            prop: {
                                basic: {
                                    container: {
                                        theme: Primary,
                                    }
                                }
                            }
                            text: {
                                text: "View"
                            },
                            value: "tab_view"
                        }
                        <GMenuItem> {
                            prop: {
                                basic: {
                                    container: {
                                        theme: Primary,
                                    }
                                }
                            }
                            text: {
                                text: "Button"
                            },
                            value: "tab_button"
                        }
                    }
                }
            }
        }
        <GVLayout> {
            app_router = <GRouter> {
                bar_pages = {
                    view_page = <GBarPage> {
                        <ViewPage>{}
                    }
                    button_page = <GBarPage> {
                        <ButtonPage>{}
                    }
                    // tabbar = <GTabbar>{
                    //     <GTabbarItem>{
                    //         icon: {
                    //             src: dep("crate://self/resources/wind.svg"),
                    //         }
                    //         text: {
                    //             text: "Config"
                    //         }
                    //     }
                    //     <GTabbarItem>{
                    //         icon: {
                    //              src: dep("crate://self/resources/heavy.svg"),
                    //         }
                    //         text: {
                    //             text: "All"
                    //         }
                    //     }
                    // }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct HomePage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    pub lifecycle: LifeCycle,
}

impl LiveHook for HomePage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.deref_widget.after_new_before_apply(cx);
    }
    fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.before_apply(cx, apply, index, nodes);
    }
    fn after_update_from_doc(&mut self, cx: &mut Cx) {
        self.deref_widget.after_update_from_doc(cx);
    }
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        self.deref_widget.after_apply_from_doc(cx);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.deref_widget.after_new_from_doc(cx);
        self.gmenu(id!(menu)).borrow_mut().map(|mut menu| {
            menu.set_active(cx, Some("tab_button".to_string()));
        });
    }
    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        self.deref_widget
            .apply_value_instance(cx, apply, index, nodes)
    }
}

impl Widget for HomePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        if self.lifecycle.is_created() {
            let router = self.grouter(id!(app_router));
            router.borrow_mut().map(|mut router| {
                router
                    .init(ids!(view_page, button_page), None, None)
                    .active(id!(button_page))
                    .build(cx);
            });
            let menu = self.gmenu(id!(menu));

            self.lifecycle.next();
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        // self.deref_widget.handle_event(cx, event, scope);
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope);
    }
}

impl MatchEvent for HomePage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.grouter(id!(app_router));
        if let Some(MenuChanged { meta, active }) = self.gmenu(id!(menu)).changed(actions) {
            if let Some(active) = active {
                match active.as_str() {
                    "tab_view" => {
                        router.nav_to(cx, id!(view_page));
                    }
                    "tab_button" => {
                        router.nav_to(cx, id!(button_page));
                    }
                    _ => {}
                }
            }
        }

        router.borrow_mut().map(|mut route| {
            route.handle_nav_events(cx, &actions);
        });
    }
}

widget_node!(HomePage);
