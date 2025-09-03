use makepad_widgets::*;

#[derive(Clone, Copy, Live, LiveHook, LiveRegister, Default)]
#[live_ignore]
pub enum NavMode {
    #[pick]
    #[default]
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

#[allow(unused)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PageType {
    #[default]
    Bar,
    Nav,
    /// no default display page
    None,
}

#[allow(unused)]
impl PageType {
    pub fn live_id(&self) -> LiveId {
        match self {
            PageType::Bar => id!(bar_pages)[0].clone(),
            PageType::Nav => id!(nav_pages)[0].clone(),
            PageType::None => id!(nav_pages)[0].clone(),
        }
    }
}