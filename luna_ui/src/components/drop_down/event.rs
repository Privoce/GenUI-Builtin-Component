use makepad_widgets::{
    ActionDefaultRef, DefaultNone, FingerDownEvent, FingerHoverEvent, FingerUpEvent,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum DropDownEvent {
    Changed(DropDownChanged),
    None,
}

#[derive(Debug, Clone)]
pub struct DropDownChanged {
    pub meta: DropDownToggleEvent,
    pub opened: bool,
}

#[derive(Debug, Clone, Default)]
pub enum DropDownToggleEvent {
    Click(FingerUpEvent),
    Hover(FingerHoverEvent),
    Press(FingerDownEvent),
    // KetFocusLost(KeyFocusEvent),
    #[default]
    Other,
}
