use makepad_widgets::{ActionDefaultRef, DefaultNone, FingerHoverEvent, FingerUpEvent};

#[derive(Debug, Clone, DefaultNone)]
pub enum GCheckboxEvent {
    Clicked(GCheckboxClickedParam),
    HoverIn(GCheckboxHoverParam),
    HoverOut(GCheckboxHoverParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GCheckboxClickedParam {
    pub value: Option<String>,
    pub selected: bool,
    pub e: Option<FingerUpEvent>,
}

#[derive(Clone, Debug)]
pub struct GCheckboxHoverParam {
    pub e: Option<FingerHoverEvent>,
}

// -------------------------------------------------------------------------

#[derive(Debug, Clone, DefaultNone)]
pub enum GCheckboxGroupEvent {
    Changed(GCheckboxGroupEventParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GCheckboxGroupEventParam {
    /// The index of the selected checkboxs.
    /// checkbox_group can have multiple selected checkboxs.
    pub selected: Vec<usize>,
    /// The value of the selected checkboxs.
    pub values: Vec<Option<String>>,
    pub e: Option<FingerUpEvent>,
}
