use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Clone, Debug, DefaultNone)]
pub enum CheckboxEvent {
    HoverIn(CheckboxHoverIn),
    HoverOut(CheckboxHoverOut),
    Clicked(CheckboxClicked),
    None,
}

#[derive(Clone, Debug)]
pub struct CheckboxHoverIn {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct CheckboxHoverOut {
    pub meta: FingerHoverEvent,
}

#[derive(Clone, Debug)]
pub struct CheckboxClicked {
    pub meta: Option<FingerUpEvent>,
    pub active: bool,
    pub value: String,
}

#[derive(Debug, Clone, DefaultNone)]
pub enum CheckboxGroupEvent {
    Changed(CheckboxGroupChanged),
    None,
}

#[derive(Clone, Debug)]
pub struct CheckboxGroupChanged {
    pub meta: Option<FingerUpEvent>,
    /// The index of the active checkbox
    pub index: Vec<i32>,
    /// The value of the active checkbox.
    pub value: Vec<String>,
}
