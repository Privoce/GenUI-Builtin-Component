use makepad_widgets::{
    event::FingerLongPressEvent, ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent,
    FingerMoveEvent, FingerUpEvent, KeyEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum ViewEvent {
    FingerDown(ViewFingerDown),
    FingerUp(ViewFingerUp),
    LongPress(ViewLongPress),
    Move(ViewMove),
    HoverIn(ViewHoverIn),
    HoverOut(ViewHoverOut),
    HoverOver(ViewHoverOver),
    KeyDown(ViewKeyDown),
    KeyUp(ViewKeyUp),
    Clicked(ViewClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct ViewClicked {
    pub meta: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct ViewKeyDown {
    pub meta: KeyEvent,
}

#[derive(Clone, Debug)]
pub struct ViewKeyUp {
    pub meta: KeyEvent,
}

#[derive(Clone, Debug)]
pub struct ViewFingerDown {
    pub meta: FingerDownEvent,
}

#[derive(Clone, Debug)]
pub struct ViewFingerUp {
    pub meta: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct ViewLongPress {
    pub meta: FingerLongPressEvent,
}

#[derive(Clone, Debug)]
pub struct ViewMove {
    pub meta: FingerMoveEvent,
}

#[derive(Clone, Debug)]
pub struct ViewHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct ViewHoverOver {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct ViewHoverOut {
    pub meta: FingerHoverEvent,
}
