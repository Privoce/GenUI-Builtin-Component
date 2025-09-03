use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum CollapseEvent {
    HoverIn(CollapseHoverIn),
    HoverOut(CollapseHoverOut),
    Changed(CollapseChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct CollapseHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct CollapseHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct CollapseChanged {
    pub meta: Option<FingerUpEvent>,
    pub active: bool,
}
