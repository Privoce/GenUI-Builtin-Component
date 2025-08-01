use makepad_widgets::*;

use crate::{component_state, components::{label::LabelBasicProp, svg::SvgBasicProp, traits::{ComponentState, Part}, view::ViewBasicProp}, prop::manuel::{ACTIVE, BASIC, DISABLED, HOVER}};

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

component_state! {
    TabbarItemState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    },
    _ => TabbarItemState::Basic
}

impl ComponentState for TabbarItemState {
    fn is_disabled(&self) -> bool {
        matches!(self, TabbarItemState::Disabled)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TabbarItemPart {
    Icon,
    Text,
    Container,
}

impl Part for TabbarItemPart {
    type State = TabbarItemState;
    fn to_live_id(&self) -> LiveId {
        match self {
            TabbarItemPart::Container => live_id!(container),
            TabbarItemPart::Icon => live_id!(icon),
            TabbarItemPart::Text => live_id!(text),
        }
    }
}
