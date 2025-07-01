use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum CardEvent {
    HoverIn(CardHoverIn),
    HoverOut(CardHoverOut),
    None,
}

#[derive(Debug, Clone)]
pub struct CardHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct CardHoverOut {
    pub meta: FingerHoverEvent,
}
