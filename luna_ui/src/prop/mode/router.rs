use makepad_widgets::*;

/// Router Tabbar(Indicator|Menu) Mode
/// - VirtualMenu: virtual route use code to config GMenu (todo!)
/// - VirtualTabbar: virtual route use code to config GTabbar (AbstractGTabbar)(todo!)
/// - Bind: default mode, use dsl declare
/// - Define: define a indicator to call router nav_to
#[derive(Debug, Clone)]
pub enum RouterIndicatorMode {
    // VirtualMenu,
    // VirtualTabbar,
    Bind(LiveId),
    Define,
}

impl Default for RouterIndicatorMode {
    fn default() -> Self {
        Self::Bind(id!(tabbar)[0])
    }
}

impl RouterIndicatorMode {
    /// judge self is bind and eq the input id
    /// - if current is not bind -> false
    /// - or back `bind_id == id`
    pub fn eq_bind(&self, id: &LiveId) -> bool {
        if let RouterIndicatorMode::Bind(bind_id) = self {
            bind_id == id
        } else {
            false
        }
    }
}


#[derive(Clone, Copy, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub enum NavMode {
    #[pick]
    /// History mode
    /// - use history to navigate (nav_to or nav_back) till stack is empty
    /// ```
    /// nav_to: A -> B -> C -> D
    /// nav_back: D -> C -> B -> A
    /// ```
    History,
    /// Stack mode
    /// ```
    /// nav_to: A -> B -> C -> D
    /// nav_back: D -> C -> D -> C
    /// ```
    Switch,
}
