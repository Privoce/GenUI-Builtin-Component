use makepad_widgets::*;

live_design!{
    use link::shaders::*;

    DrawGImage = {{DrawGImage}}{
        texture image: texture2d
        opacity: 1.0
        image_scale: vec2(1.0, 1.0)
        image_pan: vec2(0.0, 0.0)
                
        fn get_color_scale_pan(self, scale: vec2, pan: vec2) -> vec4 {
            return sample2d(self.image, self.pos * scale + pan).xyzw;
        }
                                
        fn get_color(self) -> vec4 {
            return self.get_color_scale_pan(self.image_scale, self.image_pan)
        }
        
        fn pixel(self) -> vec4 {
            let color = self.get_color();
            return Pal::premul(vec4(color.xyz, color.w * self.opacity))
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawGImage {
    #[deref] draw_super: DrawQuad,
    #[live] pub opacity: f32,
    #[live] image_scale: Vec2,
    #[live] image_pan: Vec2,
}
