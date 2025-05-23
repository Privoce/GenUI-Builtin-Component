use makepad_widgets::*;

#[derive(Clone, Copy, Default, Debug, Live, LiveRegister, LiveHook)]
#[live_ignore]
pub struct BorderRadius {
    #[live]
    pub top: f64,
    #[live]
    pub right: f64,
    #[live]
    pub bottom: f64,
    #[live]
    pub left: f64,
}
