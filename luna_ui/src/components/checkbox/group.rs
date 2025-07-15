use super::event::{CheckboxChanged, CheckboxGroupEvent};
use crate::{
    components::{checkbox::GCheckboxWidgetRefExt, view::GView},
    visible,
};
use makepad_widgets::*;

live_design! {
    link luna_basic;

    pub GCheckboxGroupBase = {{GCheckboxGroup}} {
        prop: {
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
    pub active: Option<String>,
}

impl WidgetNode for GCheckboxGroup {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        self.deref_widget.uid_to_widget(uid)
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.deref_widget.find_widgets(path, cached, results);
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.deref_widget.walk(cx)
    }

    fn area(&self) -> Area {
        self.deref_widget.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
    visible!();
}

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
        let mut active_events = None;
        for (index, (_id, child)) in self.children.iter().enumerate() {
            let _ = child.as_gcheckbox().borrow().map(|radio| {
                if let Some(param) = radio.clicked(&actions) {
                    if param.active && active_indexs.is_none() && active_events.is_none() {
                        active_values.replace(param.value);
                        active_indexs = Some(index);
                        active_events = param.meta;
                    }
                }
            });
            if active_indexs.is_some() {
                break;
            }
        }
        if active_indexs.is_some() && active_values.is_some() {
            let _ = self.set_active(cx, active_values.clone());
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CheckboxGroupEvent::Changed(CheckboxChanged {
                    meta: active_events,
                    values: active_values,
                    indexs: active_indexs.unwrap() as i32,
                }),
            );
        }
    }
}

impl LiveHook for GCheckboxGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if let Some(active) = self.active.as_ref() {
            self.set_active(cx, Some(active.to_string()));
        } else {
            self.find_active();
        }
    }
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        self.deref_widget.after_apply_from_doc(cx);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.deref_widget.after_new_from_doc(cx);
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

impl GCheckboxGroup {
    pub fn find_active(&mut self) -> () {
        if self.active.is_some() {
            return;
        }

        let mut active_value = None;
        self.children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gcheckbox().borrow_mut() {
                    // 判断radio的value是否为空，如果是就按照iter的index设置
                    if child.value.is_empty() {
                        child.value = index.to_string();
                    }
                    if child.active && active_value.is_none() {
                        active_value.replace(child.value.to_string());
                    } else if child.active && active_value.is_some() {
                        panic!(
                            "GCheckboxGroup can only have one active GCheckbox, but found multiple: {}",
                            child.value
                        );
                    }
                } else {
                    panic!("GCheckboxGroup only allows GCheckbox as child!");
                }
            });

        if let Some(active_value) = active_value {
            self.active.replace(active_value);
        }
    }
    /// if active is not set(None) in the group: find the active radio in the group
    /// else: set the active radio depending on the value of `active`
    pub fn set_active(&mut self, cx: &mut Cx, active_value: Option<String>) -> () {
        self.active = active_value;

        self.children
            .iter()
            .enumerate()
            .for_each(|(index, (_id, child))| {
                if let Some(mut child) = child.as_gcheckbox().borrow_mut() {
                    if child.value.is_empty() {
                        child.value = index.to_string();
                    }
                    let active = child.value.eq(self.active.as_ref().unwrap());
                    child.toggle(cx, active);
                } else {
                    panic!("GCheckboxGroup only allows GCheckbox as child!")
                }
            });
    }
}
