use makepad_widgets::*;

use crate::components::{radio::RadioBasicProp, switch::SwitchBasicProp};

live_design! {
    use link::shaders::*;

    DrawSwitch = {{DrawSwitch}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let box_size = vec2(self.rect_size.x - self.border_width * 2.0, self.rect_size.y - self.border_width * 2.0);

            sdf.box(self.pos.x, self.pos.y, box_size.x, box_size.y, border_radius);
            if self.background_visible == 1.0 {
                sdf.fill_keep(self.get_background_color());
            }
            sdf.stroke(self.border_color, self.border_width);
            // let circle = vec2(box_size.y * 0.5 - 1.0);
            // let center = self.rect_size.y * 0.5;
            // let offset = self.rect_size.y - box_size.y;
            // match self.toggle_type{
            //     SwitchType::Round => {
            //         sdf.circle(mix(
            //             mix(circle.x + self.border_width + offset, circle.x + self.border_width + offset * 2.0, self.hover),
            //             mix(self.rect_size.x - circle.x - offset - self.border_width,self.rect_size.x - circle.x - offset * 2.0 - self.border_width, self.hover),
            //             self.active
            //         ), center, circle.x);

            //         sdf.circle(mix(
            //             mix(circle.x + self.border_width + offset, circle.x + self.border_width + offset * 2.0, self.hover),
            //             mix(self.rect_size.x - circle.x - offset - self.border_width,self.rect_size.x - circle.x - offset * 2.0 - self.border_width, self.hover),
            //             self.active
            //         ), center, circle.x);
            //     }
            //     SwitchType::Rect => {
            //         let y = self.border_width + offset * 0.5;
            //         sdf.box(mix(
            //             mix(circle.x + self.border_width - circle.x + offset,circle.x + self.border_width - circle.x + offset * 2.0, self.hover),
            //             mix(self.rect_size.x - circle.x * 2.0 - offset - self.border_width, self.rect_size.x - circle.x * 2.0 - offset * 2.0 - self.border_width, self.hover),
            //             self.active
            //         ), y, circle.x * 2.0, circle.x* 2.0, border_radius);

            //         sdf.box(mix(
            //             mix(circle.x + self.border_width - circle.x + offset,circle.x + self.border_width - circle.x + offset * 2.0, self.hover),
            //             mix(self.rect_size.x - circle.x * 2.0 - offset - self.border_width, self.rect_size.x - circle.x * 2.0 - offset * 2.0 - self.border_width, self.hover),
            //             self.active
            //         ),y , circle.x* 2.0, circle.x* 2.0, border_radius);
            //     }
            // }

            // sdf.blend(self.active)
            // sdf.fill(
            //    self.get_stroke_color()
            // );

            return sdf.result
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawSwitch {
    #[deref]
    pub draw_super: DrawQuad,
    // ---- event state
    #[live]
    pub hover: f32, // 盒子的hover状态
    #[live]
    pub active: f32, // 盒子的选中状态
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
    #[live(2.0)]
    pub border_radius: f32,
}

impl DrawSwitch {

    pub fn merge(&mut self, other: &SwitchBasicProp) {
        self.background_color = other.background_color;
        self.background_visible = other.background_visible.to_f32();
        self.stroke_color = other.stroke_color;
        self.border_color = other.border_color;
        self.size = other.size;
        self.border_width = other.border_width;
        self.mode = other.mode;
    }
    pub fn state_basic(&mut self) {
        if self.hover != 0.0 || self.active != 0.0 {
            self.hover = 0.0;
            self.active = 0.0;
        }
    }
    pub fn state_hover(&mut self) {
        if self.hover != 1.0 {
            self.hover = 1.0;
            self.active = 0.0;
        }
    }
    pub fn state_active(&mut self) {
        if self.active != 1.0 {
            self.hover = 0.0;
            self.active = 1.0;
        }
    }
}