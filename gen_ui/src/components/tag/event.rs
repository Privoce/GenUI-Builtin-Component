use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum TagEvent {
    HoverIn(TagHoverIn),
    HoverOut(TagHoverOut),
    FingerDown(TagFingerDown),
    Clicked(TagClicked),
    Close(TagClose),
    None,
}

#[derive(Clone, Debug)]
pub struct TagHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TagHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TagFingerDown {
    pub meta: FingerDownEvent,
}

#[derive(Clone, Debug)]
pub struct TagClicked {
    pub meta: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct TagClose {
    pub meta: FingerUpEvent,
}
