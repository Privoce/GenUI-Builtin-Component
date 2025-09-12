use makepad_widgets::*;

use crate::{components::LinkBasicStyle, prop::traits::ToFloat, shader::draw_view::DrawView};

live_design! {
    use link::shaders::*;
    DrawLink = {{DrawLink}}{
        // [pixel shader] -------------------------------------------------------------------------
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size3);
            // - [draw shadow and blur] -----------------------------------------------------------
            if sdf.shape > -1.0 {
                if self.spread_radius > 0.0 || self.blur_radius > 0.0 {
                    let shadow_offset = self.shadow_offset + self.rect_shift;
                    let total_shadow_size = self.spread_radius + self.blur_radius;
                    let shadow_lower = vec2(total_shadow_size) + shadow_offset;
                    let shadow_upper = self.rect_size + vec2(self.spread_radius * 2.0) + shadow_offset;
                    if self.border_radius.x != 0.0 || self.border_radius.y != 0.0 ||
                        self.border_radius.z != 0.0 || self.border_radius.w != 0.0 {
                        let max_border_radius = max(
                            max(
                                max(self.border_radius.x, self.border_radius.y),
                                max(self.border_radius.z, self.border_radius.w)
                            ), 1.0
                        );
                        let v = GaussShadow::rounded_box_shadow(
                            shadow_lower,
                            shadow_upper,
                            self.pos * self.rect_size3,
                            self.blur_radius,
                            max_border_radius
                        );
                        let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                        sdf.clear(shadow_color);
                    } else {
                        let v = GaussShadow::box_shadow(
                            shadow_lower,
                            shadow_upper,
                            self.pos * self.rect_size3,
                            self.blur_radius
                        );
                        let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                        sdf.clear(shadow_color);
                    }
                }
            }


            // - [basic sdf for draw a view] ------------------------------------------------------
            let border_width = self.border_width;
            let total_shadow_size = self.spread_radius + self.blur_radius;

            // - [background color if visible] ----------------------------------------------------
            if self.background_visible == 1.0 {
                // 使用calculated位置而不是原始rect_size
                sdf.box_all(
                    self.sdf_rect_pos.x,
                    self.sdf_rect_pos.y,
                    self.sdf_rect_size.x,
                    self.sdf_rect_size.y,
                    max(self.border_radius.x, 1.0),
                    max(self.border_radius.y, 1.0),
                    max(self.border_radius.z, 1.0),
                    max(self.border_radius.w, 1.0)
                );
                sdf.fill(self.get_background_color());
            }
            // - [draw underline] -----------------------------------------------------------------
            if self.underline_visible == 1.0 {
                // 通过font_size来计算下划线位置，font一定在盒子的中间
                if self.underline_width > 0.0 {
                    let underline_y = self.pos.y + (self.rect_size.y - self.font_size - self.underline_width) / 2.0;
                    let offset = self.underline_width + 1.0;
                    let max_border_offset = max(self.border_radius.x, max(self.border_radius.y, max(self.border_radius.z, self.border_radius.w))) * 2.0;
                    let underline_h = self.rect_size.x - max_border_offset;
                    sdf.rect(0.0 + max_border_offset / 2.0 , underline_y + offset + self.font_size, underline_h, self.underline_width);
                    sdf.fill(self.underline_color);
                }
            }

            // - [border with and color if width bigger than 0] -----------------------------------
            if border_width > 0.0 {
                sdf.stroke(self.get_border_color(), border_width);
            }

            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawLink {
    #[deref]
    pub draw_super: DrawView,
    #[live(1.0)]
    pub underline_visible: f32,
    #[live]
    pub underline_color: Vec4,
    #[live(1.0)]
    pub underline_width: f32,
    #[live(12.0)]
    pub font_size: f32,
}

impl DrawLink {
    pub fn merge(&mut self, other: &LinkBasicStyle) {
        self.underline_color = other.underline_color;
        self.underline_visible = other.underline_visible.to_f32();
        self.underline_width = other.underline_width;
        self.background_color = other.background_color;
        self.border_color = other.border_color;
        self.border_width = other.border_width;
        self.border_radius = other.border_radius.into();
        self.shadow_color = other.shadow_color.into();
        self.spread_radius = other.spread_radius;
        self.blur_radius = other.blur_radius;
        self.shadow_offset = other.shadow_offset;
        self.background_visible = other.background_visible.to_f32();
        self.rotation = other.rotation;
        self.scale = other.scale;
        self.font_size = other.font_size;
        // self.draw_super.merge(&other.into());
    }
}
