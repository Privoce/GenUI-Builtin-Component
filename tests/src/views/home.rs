use luna_ui::{components::*, inherits_view_livehook, inherits_view_widget_node};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::basic::view::*;
    use crate::views::basic::button::*;
    use crate::views::basic::label::*;
    use crate::views::basic::svg::*;
    use crate::views::basic::image::*;
    use crate::views::basic::card::*;
    use crate::views::form::radio::*;
    use crate::views::form::checkbox::*;
    use crate::views::form::switch::*;

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
                                text: "Label"
                            },
                            value: "tab_label"
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
                        <GMenuItem> {
                            prop: {
                                basic: {
                                    container: {
                                        theme: Primary,
                                    }
                                }
                            }
                            text: {
                                text: "Svg"
                            },
                            value: "tab_svg"
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
                                text: "Image"
                            },
                            value: "tab_image"
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
                                text: "Card"
                            },
                            value: "tab_card"
                        }
                    }
                }
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
                            text: "Form Components"
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
                                text: "Radio"
                            },
                            value: "tab_radio"
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
                                text: "Checkbox"
                            },
                            value: "tab_checkbox"
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
                                text: "Switch"
                            },
                            value: "tab_switch"
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
                    label_page = <GBarPage> {
                        <LabelPage>{}
                    }
                    svg_page = <GBarPage> {
                        <SvgPage>{}
                    }
                    image_page = <GBarPage> {
                        <ImagePage>{}
                    }
                    card_page = <GBarPage> {
                        <CardPage>{}
                    }
                    radio_page = <GBarPage> {
                        <RadioPage>{}
                    }
                    checkbox_page = <GBarPage> {
                        <CheckboxPage>{}
                    }
                    switch_page = <GBarPage> {
                        <SwitchPage>{}
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
            menu.set_active(cx, Some("tab_card".to_string()));
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
                    .init(
                        ids!(
                            view_page,
                            button_page,
                            label_page,
                            svg_page,
                            image_page,
                            card_page,
                            radio_page,
                            checkbox_page,
                            switch_page
                        ),
                        None,
                        None,
                    )
                    .active(id!(card_page))
                    .build(cx);
            });
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
                    "tab_label" => {
                        router.nav_to(cx, id!(label_page));
                    }
                    "tab_svg" => {
                        router.nav_to(cx, id!(svg_page));
                    }
                    "tab_image" => {
                        router.nav_to(cx, id!(image_page));
                    }
                    "tab_card" => {
                        router.nav_to(cx, id!(card_page));
                    }
                    "tab_radio" => {
                        router.nav_to(cx, id!(radio_page));
                    }
                    "tab_checkbox" => {
                        router.nav_to(cx, id!(checkbox_page));
                    }
                    "tab_switch" => {
                        router.nav_to(cx, id!(switch_page));
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
