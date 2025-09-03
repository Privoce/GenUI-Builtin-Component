use makepad_widgets::{ActionDefaultRef, DefaultNone, HeapLiveIdPath, LiveId};

use crate::components::router::schema::PageType;

#[derive(Debug, Clone, DefaultNone)]
pub enum RouterEvent {
    NavTo(LiveId),
    NavBack(LiveId),
    // Init(RouterInit),
    None,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct RouterInit {
    pub active_page: Option<HeapLiveIdPath>,
    pub page_type: PageType,
}