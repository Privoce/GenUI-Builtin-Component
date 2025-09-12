use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub RouterPage = {{RouterPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fill,
                        width: Fill,
                        flow: Down,
                        spacing: 20.0,
                    }
                }
                <GHLayout> {
                    style: {basic: {height: Fit}}
                    n1 = <GButton>{slot: {text: "Nav Page 1"}}
                    n2 = <GButton>{slot: {text: "Nav Page 2"}}
                }
                page_router = <GRouter> {
                    bar_pages = {
                        style:{basic: {height: Fill, width: Fill, background_visible: true, background_color: #666}}
                        bpage1 = <GBarPage> {
                            style:{basic: {height: Fill, width: Fill, background_visible: true}}
                            <GLabel> {text: "Bar Page 1"}
                        }
                        bpage2 = <GBarPage> {
                            <GLabel> {text: "Bar Page 2"}
                        }
                        bpage3 = <GBarPage> {
                            <GLabel> {text: "Bar Page 3"}
                        }
                        tabbar = <GTabbar> {
                            <GTabbarItem> {
                                icon: <GSvg> {
                                    src: dep("crate://self/resources/news.svg"),
                                }
                                text: <GLabel> {
                                    text: "News"
                                }
                            }
                            <GTabbarItem> {
                                icon: <GSvg> {
                                    src: dep("crate://self/resources/global.svg"),
                                }
                                text: <GLabel> {
                                    text: "Global"
                                }
                            }
                            <GTabbarItem> {
                                icon: <GSvg> {
                                    src: dep("crate://self/resources/star.svg"),
                                }
                                text: <GLabel> {
                                    text: "For You"
                                }
                            }
                        }
                    }
                    nav_pages = {
                        style:{basic: {height: Fill, width: Fill, background_visible: true, background_color: #666}}
                        npage1 = <GNavPage> {
                            <GLabel> {
                                text: "Nav Page 1"
                            }
                        }
                        npage2 = <GNavPage> {
                            <GLabel> {
                                text: "Nav Page 2"
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
pub struct RouterPage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    pub lifecycle: LifeCycle,
}

impl LiveHook for RouterPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for RouterPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        if self.lifecycle.is_created() {
            let router = self.grouter(id!(page_router));
            router.borrow_mut().map(|mut router| {
                router
                    .init(
                        ids!(bpage1, bpage2, bpage3),
                        Some(ids!(npage1, npage2)),
                        None,
                    )
                    .active(id!(bpage1))
                    .build(cx);
            });
            self.lifecycle.next();
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // self.match_event(cx, event);
        // self.deref_widget.handle_event(cx, event, scope)
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        let router = self.grouter(id!(page_router));
        let n1 = self.gbutton(id!(n1));
        let n2 = self.gbutton(id!(n2));
        if let Some(_) = n1.clicked(&actions) {
            router.nav_to(cx, id!(npage1));
        }
        if let Some(_) = n2.clicked(&actions) {
            router.nav_to(cx, id!(npage2));
        }

        router.borrow_mut().map(|mut route| {
            route.handle_nav_events(cx, &actions);
        });
    }
}

widget_node!(RouterPage);
