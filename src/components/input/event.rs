use makepad_widgets::{ActionDefaultRef, DefaultNone, KeyCode, KeyEvent, KeyFocusEvent, KeyModifiers};

#[derive(Clone, Debug, DefaultNone)]
pub enum GInputEvent {
    Changed(GInputChangedParam),
    KeyDownUnhandled(KeyEvent),
    Escaped(KeyEvent),
    KeyFocus(KeyFocusEvent),
    KeyFocusLost(KeyFocusEvent),
    None,
}

#[derive(Clone, Debug)]
pub struct GInputChangedParam {
    pub text: String,
    pub ty: InputEventType,
    pub modifiers: Option<KeyModifiers>,
}

#[derive(Clone, Debug)]
pub enum InputEventType {
    KeyDown(KeyCode),
    Input,
    Cut,
}
