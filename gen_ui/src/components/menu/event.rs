use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum MenuItemEvent {
    HoverIn(MenuItemHoverIn),
    HoverOut(MenuItemHoverOut),
    Clicked(MenuItemClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct MenuItemHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct MenuItemHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct MenuItemClicked {
    pub meta: Option<FingerUpEvent>,
    pub active: bool,
    pub value: String,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum MenuEvent {
    HoverIn(MenuHoverIn),
    HoverOut(MenuHoverOut),
    Changed(MenuChanged),
    None,
}

#[derive(Clone, Debug)]
pub struct MenuHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct MenuHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct MenuChanged {
    pub meta: Option<FingerUpEvent>,
    /// The value of the active
    pub active: Option<String>,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum SubMenuEvent {
    // HoverIn(SubMenuHoverIn),
    // HoverOut(SubMenuHoverOut),
    Changed(SubMenuChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct SubMenuChanged {
    pub active: bool,
    pub value: String,
    pub meta: Option<FingerUpEvent>,
}

// #[derive(Debug, Clone)]
// pub struct SubMenuHoverIn {
//     pub meta: FingerHoverEvent,
// }

// #[derive(Debug, Clone)]
// pub struct SubMenuHoverOut {
//     pub meta: FingerHoverEvent,
// }

#[derive(Debug, Clone)]
pub enum MenuActionType {
    SubMenu(SubMenuChanged),
    MenuItem(MenuItemClicked)
}