use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerUpEvent, FingerHoverEvent};

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
    pub value: bool,
}
