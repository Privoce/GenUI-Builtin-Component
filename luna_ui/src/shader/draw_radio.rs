use makepad_widgets::*;

use crate::{
    components::RadioPartProp,
    prop::{traits::ToFloat, ActiveMode},
};

live_design! {
    use link::shaders::*;

    DrawRadio = {{DrawRadio}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // draw background circle
            let center = vec2(self.rect_size.x * 0.5, self.rect_size.y * 0.5);
            let r = self.rect_size.x * 0.5 - self.border_width;
            sdf.circle(center.x, center.y, r);
            if self.background_visible == 1.0 {
                sdf.fill_keep(self.background_color);
            }
            // draw border
            sdf.stroke(self.border_color, self.border_width);
            // draw inner
            match self.mode {
                ActiveMode::Round => {
                    // draw a circle with 1/4 rect_size.x
                    sdf.circle(center.x, center.y, self.rect_size.x * 0.275);
                    sdf.fill(self.stroke_color);
                }
                ActiveMode::Tick => {
                    let stroke_width = self.size * 0.1;
                    let szs = self.rect_size.x * 0.22;
                    sdf.move_to(center.x - szs, center.y);
                    sdf.line_to(center.x, center.y + szs);
                    sdf.line_to(center.x + szs, center.y - szs);
                    sdf.stroke(self.stroke_color, stroke_width);
                }
                ActiveMode::Cross => {
                    // draw a easy round rectangle
                    let cross_height_width = vec2(self.rect_size.x * 0.75, self.rect_size.y * 0.25);
                    let cross_x_y = vec2(center.x - cross_height_width.x * 0.5, center.y - cross_height_width.y * 0.5);
                    let border_radius = cross_height_width.y * 0.25;
                    sdf.box(
                        cross_x_y.x,
                        cross_x_y.y,
                        cross_height_width.x,
                        cross_height_width.y,
                        border_radius
                    );
                    sdf.fill(self.stroke_color);
                }
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawRadio {
    #[deref]
    pub draw_super: DrawQuad,
    // ---- colors
    #[live]
    pub background_color: Vec4,
    #[live(1.0)]
    pub background_visible: f32,
    #[live]
    pub stroke_color: Vec4,
    #[live]
    pub border_color: Vec4,
    // ---- size
    #[live(16.0)]
    pub size: f32,
    #[live(1.0)]
    pub border_width: f32,
    // ---- type
    #[live]
    pub mode: ActiveMode,
}

impl DrawRadio {
    pub fn apply_type(&mut self, mode: ActiveMode) {
        self.mode = mode;
    }
    pub fn merge(&mut self, other: &RadioPartProp) {
        self.background_color = other.background_color;
        self.background_visible = other.background_visible.to_f32();
        self.stroke_color = other.stroke_color;
        self.border_color = other.border_color;
        self.size = other.size;
        self.border_width = other.border_width;
        self.mode = other.mode;
    }
}
