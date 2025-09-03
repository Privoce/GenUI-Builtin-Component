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

