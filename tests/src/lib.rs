pub use makepad_widgets;
pub use gen_ui;
pub mod app;
pub mod an;
pub mod views;


#[macro_export]
macro_rules! widget_node {
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
            gen_ui::visible!();
        }
    };
}
