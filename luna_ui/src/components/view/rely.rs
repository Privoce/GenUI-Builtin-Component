use makepad_widgets::*;

pub struct ViewTextureCache {
    pub pass: Pass,
    pub _depth_texture: Texture,
    pub color_texture: Texture,
}

#[derive(Clone)]
pub enum DrawState {
    Drawing(usize, bool),
    DeferWalk(usize),
}

pub fn is_texture(optimize: ViewOptimize) -> bool {
    matches!(optimize, ViewOptimize::Texture)
}
pub fn is_draw_list(optimize: ViewOptimize) -> bool {
    matches!(optimize, ViewOptimize::DrawList)
}
pub fn needs_draw_list(optimize: ViewOptimize) -> bool {
    return !matches!(optimize, ViewOptimize::None);
}

/// ## ViewState
/// - `None`: No hover or pressed state
/// - `Hover`: The view is hovered
/// - `Pressed`: The view is pressed
#[derive(Debug, Copy, Clone)]
pub enum ViewState {
    None,
    Hover,
    Pressed,
}

impl ViewState {
    pub fn id(&self) -> &[LiveId; 2] {
        match self {
            ViewState::None => id!(hover.off),
            ViewState::Hover => id!(hover.on),
            ViewState::Pressed => id!(hover.pressed),
        }
    }
}
