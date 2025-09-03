use makepad_widgets::*;

use crate::{
    components::{
         router::{event::RouterEvent, GRouter}, svg::GSvgWidgetExt, view::GView,
    },
    inherits_view_livehook, inherits_view_widget_node, prop::traits::LiveIdExp,
};

live_design! {
    link genui_basic;

    pub GPageBase = {{GPage}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GPage {
    #[deref]
    pub deref_widget: GView,
}

inherits_view_widget_node!(GPage);

impl LiveHook for GPage {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }

    inherits_view_livehook!();
}

impl Widget for GPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        for action in &actions {
            if let Some(action) = action.as_widget_action() {
                match action.cast::<RouterEvent>() {
                    RouterEvent::NavTo(path) => {
                        GRouter::nav_to_path(cx, self.widget_uid(), scope, path.as_slice());
                    }
                    RouterEvent::NavBack(_) => {
                        GRouter::nav_back_path(cx, self.widget_uid(), scope);
                    }
                    RouterEvent::None => (),
                }
            }
        }

        if self.gsvg(id!(back_icon)).clicked(&actions).is_some() {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                RouterEvent::NavBack(scope.path.clone().last()),
            );
        }
    }
}

impl GPageRef {
    pub fn set_visible_and_redraw(&mut self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.visible = visible;
            inner.redraw(cx);
        }
    }
}
