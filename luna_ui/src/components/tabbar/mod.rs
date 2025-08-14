mod event;
pub mod item;
mod prop;
// pub mod virt;
mod schema;

use crate::{
    components::view::GView, event_option, event_option_ref, inherits_view_livehook,
    inherits_view_widget_node,
};
pub use event::*;
use item::*;
use makepad_widgets::*;
pub use prop::*;
pub use schema::*;

live_design! {
    link genui_basic;

    pub GTabbarBase = {{GTabbar}} {
        prop: {
            basic: {
                height: 60.0,
                width: Fill,
                flow: Right,
                align: {
                    x: 0.5,
                    y: 0.5,
                },
                spacing: 8.0,
                background_visible: true,
                theme: Primary,
                border_radius: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0},
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GTabbar {
    #[deref]
    pub deref_widget: GView,
    #[live]
    pub active: Option<String>,
}

inherits_view_widget_node!(GTabbar);

impl Widget for GTabbar {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible() {
            return;
        }
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        };
        let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));

        let mut active_index = None;
        let mut active_value = None;
        let mut active_event = None;
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gtabbar_item().borrow().map(|radio| {
                if let Some(param) = radio.clicked(&actions) {
                    if param.active && active_index.is_none() && active_event.is_none() {
                        active_value.replace(param.value);
                        active_index = Some(index);
                        active_event = param.meta;
                    }
                }
            });
            if active_index.is_some() {
                break;
            }
        }
        if active_index.is_some() && active_value.is_some() {
            let _ = self.toggle(cx, active_value.clone(), false);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                TabbarEvent::Changed(TabbarChanged {
                    meta: active_event,
                    value: active_value,
                    index: active_index.unwrap() as i32,
                }),
            );
        }
    }
}

impl LiveHook for GTabbar {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if let Some(active) = self.active.as_ref() {
            self.set_active(cx, Some(active.to_string()));
        } else {
            self.find_active();
        }
    }
    inherits_view_livehook!();
}

impl GTabbar {
    pub fn find_active(&mut self) -> () {
        if self.active.is_some() {
            return;
        }

        let mut active_value = None;
        self.children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gtabbar_item().borrow_mut() {
                    // 判断tabbar的value是否为空，如果是就按照iter的index设置
                    if child.value.is_empty() {
                        child.value = index.to_string();
                    }
                    if child.active && active_value.is_none() {
                        active_value.replace(child.value.to_string());
                    } else if child.active && active_value.is_some() {
                        panic!(
                            "GTabbar can only have one active GTabbarItem, but found multiple: {}",
                            child.value
                        );
                    }
                } else {
                    panic!("GTabbar only allows GTabbarItem as child!");
                }
            });

        if let Some(active_value) = active_value {
            self.active.replace(active_value);
        }
    }
    /// if active is not set(None) in the group: find the active radio in the group
    /// else: set the active radio depending on the value of `active`
    pub fn set_active(&mut self, cx: &mut Cx, active_value: Option<String>) -> () {
        self.toggle(cx, active_value, true);
    }
    pub fn toggle(&mut self, cx: &mut Cx, active_value: Option<String>, init: bool) -> () {
        self.active = active_value;

        self.children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gtabbar_item().borrow_mut() {
                    if child.value.is_empty() {
                        child.value = index.to_string();
                    }
                    let active = child.value.eq(self.active.as_ref().unwrap());
                    child.toggle(cx, active, init);
                } else {
                    panic!("GTabbar only allows GTabbarItem as child!")
                }
            });
    }
    pub fn set_active_index(&mut self, cx: &mut Cx, index: usize) -> () {
        let mut active_value = None;
        self.children
            .iter()
            .enumerate()
            .for_each(|(i, (_id, child))| {
                if let Some(mut child) = child.as_gtabbar_item().borrow_mut() {
                    let active = i == index;
                    if active {
                        active_value.replace(child.value.to_string());
                    }
                    child.toggle(cx, active, false);
                } else {
                    panic!("GTabbar only allows GTabbarItem as child!")
                }
            });

        if let Some(active_value) = active_value {
            self.active.replace(active_value);
        }
    }
}

impl GTabbar {
    event_option! {
        changed: TabbarEvent::Changed => TabbarChanged
    }
}

impl GTabbarRef {
    event_option_ref! {
        changed => TabbarChanged
    }
}
