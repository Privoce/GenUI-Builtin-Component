use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum RadioEvent {
    HoverIn(RadioHoverIn),
    HoverOut(RadioHoverOut),
    Clicked(RadioClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct RadioHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct RadioHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct RadioClicked {
    pub meta: Option<FingerUpEvent>,
    pub active: bool,
    pub value: String,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum RadioGroupEvent {
    Changed(RadioGroupChanged),
    None,
}

#[derive(Clone, Debug)]
pub struct RadioGroupChanged {
    pub meta: Option<FingerUpEvent>,
    /// The index of the active radio.
    pub index: i32,
    /// The value of the active radio.
    pub value: Option<String>,
}
