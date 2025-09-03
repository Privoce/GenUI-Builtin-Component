use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum ButtonEvent {
    HoverIn(ButtonHoverIn),
    HoverOut(ButtonHoverOut),
    Clicked(ButtonClicked),
    FingerUp(ButtonFingerUp),
    FingerDown(ButtonFingerDown),
    None,
}

#[derive(Debug, Clone)]
pub struct ButtonHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct ButtonHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Debug, Clone)]
pub struct ButtonClicked {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct ButtonFingerUp {
    pub meta: FingerUpEvent,
}

#[derive(Debug, Clone)]
pub struct ButtonFingerDown {
    pub meta: FingerDownEvent,
}
