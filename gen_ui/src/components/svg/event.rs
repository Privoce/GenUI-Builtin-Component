use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum SvgEvent {
    HoverIn(SvgHoverIn),
    HoverOut(SvgHoverOut),
    Clicked(SvgClicked),
    FingerUp(SvgFingerUp),
    FingerDown(SvgFingerDown),
    None,
}

#[derive(Debug, Clone)]
pub struct SvgHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct SvgHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct SvgClicked {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct SvgFingerUp {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct SvgFingerDown {
    pub meta: FingerDownEvent,
}
