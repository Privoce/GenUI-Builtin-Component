use luna_ui::{
    components::{button::GButtonWidgetExt, lifecycle::LifeCycle, router::GRouterWidgetExt, view::{GView, GViewWidgetExt}},
    inherits_view_livehook, inherits_view_widget_node,
};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::basic::view::*;
    use crate::views::basic::button::*;

    pub THomePage = {{THomePage}} {
        <GVLayout> {
            v1 = <GView> {
                prop: {
                    basic: {
                        background_visible: true,
                        height: 200.0,
                        width: 200.0
                    }
                },
                visible: true
            }
            v2 = <GBarPage> {
                prop: {
                    basic: {
                        background_visible: true,
                        height: 200.0,
                        width: 200.0,
                        theme: Info
                    }
                },
                visible: false
            }
            btn1 = <GButton> {

            }
            btn2 = <GButton> {
                slot: {
                    text: "Button 2"
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct THomePage {
    #[deref]
    pub deref_widget: GView,
    #[rust]
    pub lifecycle: LifeCycle,
}

impl LiveHook for THomePage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for THomePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
        
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        // self.deref_widget.handle_event(cx, event, scope);
        let btn1 = self.gbutton(id!(btn1));
        let btn2 = self.gbutton(id!(btn2));
        let v1 = self.gview(id!(v1));
        let v2 = self.gview(id!(v2));
        if let Some(_) = btn1.clicked(&actions) {
           v1.set_visible(cx, !v1.get_visible());
        }
        if let Some(_) = btn2.clicked(&actions) {
           v2.set_visible(cx, !v2.get_visible());
        }
    }
}

widget_node!(THomePage);
