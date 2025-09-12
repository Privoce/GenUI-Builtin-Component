use super::event::{CheckboxGroupChanged, CheckboxGroupEvent};
use crate::{
    components::{checkbox::GCheckboxWidgetRefExt, view::GView},
    inherits_view_livehook, inherits_view_widget_node,
};
use makepad_widgets::*;

live_design! {
    link genui_basic;

    pub GCheckboxGroupBase = {{GCheckboxGroup}} {
        style: {
            basic: {
                height: Fit,
                width: Fit,
                flow: Right,
                align: {
                    x: 0.5,
                    y: 0.5,
                },
                spacing: 8.0,
                background_visible: false,
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GCheckboxGroup {
    #[deref]
    pub deref_widget: GView,
    // target active radio, only one radio can be active in a group
    #[live]
    pub active: Vec<String>,
}

inherits_view_widget_node!(GCheckboxGroup);

impl Widget for GCheckboxGroup {
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

        let mut active_indexs = None;
        let mut active_values = None;
        let mut active_event = None;
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gcheckbox().borrow().map(|checkbox| {
                if let Some(param) = checkbox.clicked(&actions) {
                    if param.active && active_indexs.is_none() && active_event.is_none() {
                        active_values
                            .get_or_insert(Vec::new())
                            .push(checkbox.value.to_string());
                        active_indexs.get_or_insert(Vec::new()).push(index as i32);
                        if let Some(meta) = param.meta {
                            active_event.replace(meta);
                        }
                    }
                }
            });
            if active_event.is_some() {
                break;
            }
        }

        if active_event.is_some() {
            // let _ = self.toggle(cx, active_values.as_ref().unwrap().clone(), false);
            let _ = self.find_active();
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CheckboxGroupEvent::Changed(CheckboxGroupChanged {
                    meta: active_event,
                    value: active_values.unwrap(),
                    index: active_indexs.unwrap(),
                }),
            );
        }
    }
}

impl LiveHook for GCheckboxGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.active.is_empty() {
            self.find_active();
        } else {
            self.set_active(cx, self.active.clone());
        }
    }
    inherits_view_livehook!();
}

impl GCheckboxGroup {
    pub fn find_active(&mut self) -> () {
        self.active =
            self.children
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut active, (index, (_, child))| {
                    if let Some(mut child) = child.as_gcheckbox().borrow_mut() {
                        // 判断checkbox的value是否为空，如果是就按照iter的index设置
                        if child.value.is_empty() {
                            child.value = index.to_string();
                        }
                        if child.active {
                            active.push(child.value.to_string());
                        }
                    } else {
                        panic!("GCheckboxGroup only allows GCheckbox as child!");
                    }
                    active
                });
    }
    /// if active is not set(None) in the group: find the active radio in the group
    /// else: set the active radio depending on the value of `active`
    pub fn set_active(&mut self, cx: &mut Cx, active: Vec<String>) -> () {
        self.toggle(cx, active, true);
    }
    pub fn toggle(&mut self, cx: &mut Cx, active: Vec<String>, init: bool) -> () {
        self.active = active;

        self.children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gcheckbox().borrow_mut() {
                    if child.value.is_empty() {
                        child.value = index.to_string();
                    }
                    let active = self.active.contains(&child.value);
                    child.toggle(cx, active, init);
                } else {
                    panic!("GCheckboxGroup only allows GCheckbox as child!")
                }
            });
    }
}
