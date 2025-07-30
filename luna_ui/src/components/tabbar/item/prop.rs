use makepad_widgets::*;

use crate::components::{label::LabelBasicProp, svg::SvgBasicProp, view::ViewBasicProp};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct TabbarItemProp {
    #[live]
    pub basic: TabbarItemBasicProp,
    #[live]
    pub hover: TabbarItemBasicProp,
    #[live]
    pub active: TabbarItemBasicProp,
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct TabbarItemBasicProp {
    #[live]
    pub icon: SvgBasicProp,
    #[live]
    pub text: LabelBasicProp,
    #[live]
    pub container: ViewBasicProp,
}