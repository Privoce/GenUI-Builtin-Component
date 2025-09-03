#[macro_export]
macro_rules! inherits_view_widget_node {
    ($component: ty) => {
        impl WidgetNode for $component {
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
            crate::visible!();
        }
    };
}

#[macro_export]
macro_rules! inherits_view_livehook {
    () => {
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
    };
}