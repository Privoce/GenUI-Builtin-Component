use makepad_widgets::*;

#[derive(Clone, Debug, DefaultNone)]
pub enum ViewEvent {
    KeyDown(ViewKeyDownEvent),
    None,
}

#[derive(Clone, Debug)]
pub struct ViewKeyDownEvent {
    pub e: KeyEvent,
}
