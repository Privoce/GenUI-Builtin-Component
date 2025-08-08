use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum TagEvent {
    HoverIn(TagHoverIn),
    HoverOut(TagHoverOut),
    Pressed(TagPressed),
    Clicked(TagClicked),
    CloseClicked(TagCloseClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct TagHoverIn {
    pub fe: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TagHoverOut {
    pub fe: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct TagPressed {
    pub fe: FingerDownEvent,
}

#[derive(Clone, Debug)]
pub struct TagClicked {
    pub fe: FingerUpEvent,
}

#[derive(Clone, Debug)]
pub struct TagCloseClicked {
    pub fe: FingerUpEvent,
}
