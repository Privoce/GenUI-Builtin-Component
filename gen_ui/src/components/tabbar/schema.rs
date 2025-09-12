use crate::components::{svg::IconData, tabbar::item::TabbarItemProp};
use makepad_widgets::*;

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct TabbarItemData {
    #[live]
    pub value: String,
    #[live]
    pub text: String,
    #[live]
    pub icon: IconData,
    #[live]
    pub style: TabbarItemProp,
}
