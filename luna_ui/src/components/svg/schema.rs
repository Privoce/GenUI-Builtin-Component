use makepad_widgets::*;

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct IconData {
    #[live]
    pub src: LiveDependency,
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub disabled: bool,
}
