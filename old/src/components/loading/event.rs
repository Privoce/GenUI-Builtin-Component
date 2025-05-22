use makepad_widgets::{ActionDefaultRef, DefaultNone};

#[derive(Clone, Debug, DefaultNone)]
pub enum GLoadingEvent {
    Opened(GLoadingEventParam),
    Closed(GLoadingEventParam),
    None,
}

#[derive(Clone, Debug)]
pub struct GLoadingEventParam;