use makepad_widgets::*;

use crate::{components::SwitchBasicStyle, prop::traits::ToFloat};

live_design! {
    use link::shaders::*;

    DrawSwitch = {{DrawSwitch}} {
        fn pixel(self) -> vec4 {
            let scale = 0.82;
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let box_size = vec2(self.rect_size.x - self.border_width * 3.0, self.rect_size.y - self.border_width * 3.0);
            let start_point = vec2(self.pos.x + self.border_width, self.pos.y + self.border_width);
            sdf.box_all(
                start_point.x,
                start_point.y,
                box_size.x,
                box_size.y,
                self.border_radius.r,
                self.border_radius.g,
                self.border_radius.b,
                self.border_radius.a
            );
            if self.background_visible == 1.0 {
                sdf.fill_keep(self.background_color);
            }
            sdf.stroke(self.border_color, self.border_width);
            let inner_circle_radius = mix(box_size.y * scale * scale, box_size.y * scale, self.active);
            let h_spacing = (box_size.x * 0.5 - inner_circle_radius) / 2.0;
            let v_spacing = (box_size.y - inner_circle_radius) / 2.0;
            let inner_pos = mix(
                start_point + vec2(v_spacing),
                start_point + vec2(box_size.x, v_spacing) - vec2(inner_circle_radius + v_spacing, 0.0),
                self.active
            );
            let inner_radius = (self.border_radius * inner_circle_radius) / box_size.y;
            sdf.box_all(
                inner_pos.x,
                inner_pos.y,
                inner_circle_radius,
                inner_circle_radius,
                inner_radius.r,
                inner_radius.g,
                inner_radius.b,
                inner_radius.a
            );
            sdf.fill(self.stroke_color);
            return sdf.result
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawSwitch {
    #[deref]
    pub draw_super: DrawQuad,
    // ---- colors
    #[live]
    pub background_color: Vec4, // 盒子的背景色
    #[live]
    pub background_visible: f32,
    #[live]
    pub stroke_color: Vec4, // 盒子中内部绘制的线条颜色
    #[live]
    pub border_color: Vec4, // 盒子的边框颜色
    #[live(1.0)]
    pub border_width: f32,
    #[live]
    pub border_radius: Vec4,
    #[live]
    pub active: f32,
}

impl DrawSwitch {
    pub fn merge(&mut self, other: &SwitchBasicStyle) {
        self.background_color = other.background_color;
        self.background_visible = other.background_visible.to_f32();
        self.stroke_color = other.stroke_color;
        self.border_color = other.border_color;
        // self.size = other.size;
        self.border_width = other.border_width;
        self.border_radius = other.border_radius.into();
    }
}
