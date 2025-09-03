use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum TabbarItemEvent {
    HoverIn(TabbarItemHoverIn),
    HoverOut(TabbarItemHoverOut),
    Clicked(TabbarItemClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct TabbarItemHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TabbarItemHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TabbarItemClicked {
    pub meta: Option<FingerUpEvent>,
    pub active: bool,
    pub value: String,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum TabbarEvent {
    Changed(TabbarChanged),
    None,
}


#[derive(Clone, Debug)]
pub struct TabbarChanged {
    pub meta: Option<FingerUpEvent>,
    /// The index of the active radio.
    pub index: usize,
    /// The value of the active radio.
    pub value: Option<String>,
}
