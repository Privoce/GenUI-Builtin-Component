use luna_ui::{
    components::{lifecycle::LifeCycle, router::GRouterWidgetExt, view::GView},
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
        <GVLayout> {
            app_router = <GRouter> {
                bar_pages = {
                    view_page = <GBarPage> {
                        <ViewPage>{}
                    }
                    button_page = <GBarPage> {
                        <ButtonPage>{}
                    }
                    tabbar = <GTabbar>{
                        <GTabbarItem>{
                            icon: {
                                src: dep("crate://self/resources/wind.svg"),
                            }
                            text: {
                                text: "Config"
                            }
                        }
                        <GTabbarItem>{
                            icon: {
                                 src: dep("crate://self/resources/heavy.svg"),
                            }
                            text: {
                                text: "All"
                            }
                        }
                    }
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
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
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
                    .active(id!(view_page))
                    .build(cx);
            });
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // let _ = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        self.deref_widget.handle_event(cx, event, scope);
    }
}

widget_node!(HomePage);
