use makepad_widgets::{error, Cx, HeapLiveIdPath, Widget};

use crate::themes::Theme;

pub trait Component: Widget
where
    Self::Error: std::fmt::Debug,
{
    type Error;
    type State;
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
    /// ## merge component props from config theme toml
    fn merge_conf_prop(&mut self, cx: &mut Cx) -> ();
    /// ## render component
    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error>;
    // fn area(&self) -> Area;
    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> ();
    fn current_state(&self) -> Self::State;
}

pub trait Prop: Default {
    type State;
    type Basic;
    fn get(&self, state: Self::State) -> &Self::Basic;
}

pub trait BasicProp: Default {
    type State;
    type Colors;

    fn from_state(theme: Theme, state: Self::State) -> Self;
    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors;
}
