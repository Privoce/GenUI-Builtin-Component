use makepad_widgets::*;

use crate::{
    components::{radio::GRadioWidgetRefExt, view::GView},
    visible,
};

live_design! {
    link luna_basic;

    pub GRadioGroupBase = {{GRadioGroup}} {
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
pub struct GRadioGroup {
    #[deref]
    pub deref_widget: GView,
    // target active radio, only one radio can be active in a group
    #[live]
    pub active: Option<String>,
}

impl WidgetNode for GRadioGroup {
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

impl Widget for GRadioGroup {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
}

impl LiveHook for GRadioGroup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
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

impl GRadioGroup {
    /// if active is not set(None) in the group: find the active radio in the group
    /// else: set the active radio depending on the value of `active`
    pub fn find_or_set_active(&mut self, cx: &mut Cx) -> () {
        if let Some(active) = self.active.as_ref() {
            
        } else {
            let mut is_active = false;
            let mut active_value = None;
            self.children.iter().for_each(|(_id, child)| {
                if let Some(child) = child.as_gradio().borrow() {
                    if child.active && !is_active {
                        is_active = true;
                        active_value.replace(child.value.to_string());
                    } else if child.active && is_active {
                        panic!(
                            "GRadioGroup can only have one active GRadio, but found multiple: {}",
                            child.value
                        );
                    }
                } else {
                    panic!("GRadioGroup only allows GRadio as child!");
                }
            });
        }
    }
}
