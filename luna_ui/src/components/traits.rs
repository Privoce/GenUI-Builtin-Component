use makepad_widgets::{error, Area, Cx, HeapLiveIdPath, Widget};

pub trait Component: Widget
where
    Self::Error: std::fmt::Debug,
{
    type Error;
    /// ## render component after prop apply
    /// this function should use in LiveHook trait : `fn after_apply_from_doc`
    fn render_after_apply(&mut self, cx: &mut Cx) -> () {
        if !self.visible() {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("{} render error: {:?}", std::any::type_name::<Self>(), e);
        }
    }
    /// ## render component
    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error>;
    fn area(&self) -> Area;
    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> ();
}
