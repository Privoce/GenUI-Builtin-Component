use makepad_widgets::{ActionDefaultRef, DefaultNone, LiveId};

#[derive(Debug, Clone, DefaultNone)]
pub enum RouterEvent {
    NavTo(LiveId),
    NavBack(LiveId),
    None,
}
