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
    Changed(MenuChanged),
    None,
}


#[derive(Clone, Debug)]
pub struct MenuChanged {
    pub meta: Option<FingerUpEvent>,
    /// The index of the active radio.
    pub index: i32,
    /// The value of the active radio.
    pub value: Option<String>,
}
