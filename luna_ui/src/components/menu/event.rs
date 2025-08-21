use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent, LiveId};

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
    /// The index of the active
    pub index: i32,
    /// The value of the active
    pub value: Option<String>,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum SubMenuEvent {
    Changed(SubMenuChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct SubMenuChanged {
    pub active: Option<Vec<String>>,
    pub id: LiveId,
    pub meta: Option<FingerUpEvent>,
}
