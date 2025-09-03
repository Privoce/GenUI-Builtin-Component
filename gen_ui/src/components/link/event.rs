use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum LinkEvent {
    HoverIn(LinkHoverIn),
    HoverOut(LinkHoverOut),
    FingerUp(LinkFingerUp),
    FingerDown(LinkFingerDown),
    Clicked(LinkClicked),
    None,
}

#[derive(Debug, Clone)]
pub struct LinkHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct LinkHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct LinkClicked {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct LinkFingerUp {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct LinkFingerDown {
    pub meta: FingerDownEvent,
}
