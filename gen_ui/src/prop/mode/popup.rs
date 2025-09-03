use makepad_widgets::*;

/// The `PopupMode` enum represents the different modes for a popup
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy, Debug, Default)]
#[live_ignore]
#[repr(u32)]
pub enum PopupMode {
    #[pick]
    #[default]
    Popover = shader_enum(1),
    ToolTip = shader_enum(2),
    Dialog = shader_enum(3),
    Drawer = shader_enum(4),
}

/// Popup Close Mode
#[derive(Live, LiveHook, PartialEq, Eq, Clone, Copy, Debug)]
#[live_ignore]
#[repr(u32)]
pub enum CloseMode {
    /// Virtual Close, means you can not close if you click the outer, you must call close by code
    Virtual = shader_enum(1),
    #[pick]
    /// Only Outer Can Close Popup, always use when you have no close button in the popup
    Out = shader_enum(2),
}

impl Default for CloseMode {
    fn default() -> Self {
        CloseMode::Out
    }
}