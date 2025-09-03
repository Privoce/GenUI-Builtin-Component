use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum SwitchEvent {
    HoverIn(SwitchHoverIn),
    HoverOut(SwitchHoverOut),
    Clicked(SwitchClicked),
    Changed(SwitchChanged),
    None,
}

#[derive(Clone, Debug)]
pub struct SwitchHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct SwitchHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct SwitchClicked {
    pub meta: FingerUpEvent,
    pub value: bool,
}

#[derive(Clone, Debug)]
pub struct SwitchChanged {
    pub meta: Option<FingerUpEvent>,
    pub value: bool,
}
