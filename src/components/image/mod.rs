mod event;
pub mod register;

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use base64::{engine::general_purpose, Engine};
pub use event::*;

use image_cache::{ImageCacheImpl, ImageFit};
use makepad_widgets::{image_cache::ImageError, *};

use crate::{
    active_event, event_option, prop_getter, prop_setter, ref_area, ref_event_option, ref_redraw,
    ref_render, set_event, set_scope_path, shader::draw_image::DrawGImage, utils::set_cursor,
    widget_area,
};

live_design! {
    link gen_base;
    use link::shaders::*;

    pub GImageBase = {{GImage}} {
        width: 32.0,
        height: 32.0,
        // draw_image: {
        //     texture image: texture2d

        //     fn rotation_vertex_expansion(rotation: float, w: float, h: float) -> vec2 {
        //         let horizontal_expansion = (abs(cos(rotation)) * w + abs(sin(rotation)) * h) / w - 1.0;
        //         let vertical_expansion = (abs(sin(rotation)) * w + abs(cos(rotation)) * h) / h - 1.0;

        //         return vec2(horizontal_expansion, vertical_expansion);
        //     }

        //     fn rotate_2d_from_center(coord: vec2, a: float, size: vec2) -> vec2 {
        //         let cos_a = cos(-a);
        //         let sin_a = sin(-a);

        //         let centered_coord = coord - vec2(0.5, 0.5);

        //         // Denormalize the coordinates to use original proportions (between height and width)
        //         let denorm_coord = vec2(centered_coord.x, centered_coord.y * size.y / size.x);
        //         let demorm_rotated = vec2(denorm_coord.x * cos_a - denorm_coord.y * sin_a, denorm_coord.x * sin_a + denorm_coord.y * cos_a);

        //         // Restore the coordinates to use the texture coordinates proportions (between 0 and 1 in both axis)
        //         let rotated = vec2(demorm_rotated.x, demorm_rotated.y * size.x / size.y);

        //         return rotated + vec2(0.5, 0.5);
        //     }

        //     fn get_color(self) -> vec4 {
        //         let rot_padding = rotation_vertex_expansion(self.rotation, self.rect_size.x, self.rect_size.y) / 2.0;

        //         // Current position is a traslated one, so let's get the original position
        //         let current_pos = self.pos.xy - rot_padding;
        //         let original_pos = rotate_2d_from_center(current_pos, self.rotation, self.rect_size);

        //         // Scale the current position by the scale factor
        //         let scaled_pos = original_pos / self.scale;

        //         // Take pixel color from the original image
        //         let color = sample2d(self.image, scaled_pos).xyzw;

        //         let faded_color = color * vec4(1.0, 1.0, 1.0, self.opacity);
        //         return faded_color;
        //     }

        //     fn pixel(self) -> vec4 {
        //         let rot_expansion = rotation_vertex_expansion(self.rotation, self.rect_size.x, self.rect_size.y);

        //         // Debug
        //         // let line_width = 0.01;
        //         // if self.pos.x < line_width || self.pos.x > (self.scale + rot_expansion.x - line_width) || self.pos.y < line_width || self.pos.y > (self.scale + rot_expansion.y - line_width) {
        //         //     return #c86;
        //         // }

        //         let sdf = Sdf2d::viewport(self.pos * self.rect_size);

        //         let translation_offset = vec2(self.rect_size.x * rot_expansion.x / 2.0, self.rect_size.y * self.scale * rot_expansion.y / 2.0);
        //         sdf.translate(translation_offset.x, translation_offset.y);

        //         let center = self.rect_size * 0.5;
        //         sdf.rotate(self.rotation, center.x, center.y);

        //         let scaled_size = self.rect_size * self.scale;
        //         sdf.box(0.0, 0.0, scaled_size.x, scaled_size.y, 1);

        //         sdf.fill_premul(Pal::premul(self.get_color()));
        //         return sdf.result
        //     }

        //     fn vertex(self) -> vec4 {
        //         let rot_expansion = rotation_vertex_expansion(self.rotation, self.rect_size.x, self.rect_size.y);
        //         let adjusted_pos = vec2(
        //             self.rect_pos.x - self.rect_size.x * rot_expansion.x / 2.0,
        //             self.rect_pos.y - self.rect_size.y * rot_expansion.y / 2.0
        //         );

        //         let expanded_size = vec2(self.rect_size.x * (self.scale + rot_expansion.x), self.rect_size.y * (self.scale + rot_expansion.y));
        //         let clipped: vec2 = clamp(
        //             self.geom_pos * expanded_size + adjusted_pos,
        //             self.draw_clip.xy,
        //             self.draw_clip.zw
        //         );

        //         self.pos = (clipped - adjusted_pos) / self.rect_size;
        //         return self.camera_projection * (self.camera_view * (
        //             self.view_transform * vec4(clipped.x, clipped.y, self.draw_depth + self.draw_zbias, 1.)
        //         ));
        //     }

        //     shape: Solid,
        //     fill: Image
        // }
    }
}

/// # GImage Component
/// The `GImage` component handles scalable images with adjustable rotation, scaling, and visibility, supporting event triggers such as hovering and clicking.
///
/// ## Animation
/// No animation is available for the `GImage` component.
///
/// ## Event
/// The `GImage` component can trigger various events in response to user interactions:
/// - `HoverIn(GImageHoverParam)`: Triggered when the mouse pointer enters the image area.
/// - `HoverOut(GImageHoverParam)`: Triggered when the mouse pointer leaves the image area.
/// - `Clicked(GImageClickedParam)`: Triggered when the image is clicked.
///
/// ## Props
/// |macro   |prop             |description                    |type               |default |
/// |--------|-----------------|-------------------------------|-------------------|--------|
/// |live    |visible           |Visibility of the component    |bool               |true    |
/// |live    |grab_key_focus    |Enable key focus grabbing      |bool               |true    |
/// |live    |opacity           |Opacity level                  |f32                |1.0     |
/// |live    |cursor            |Mouse cursor when hovered      |Option<MouseCursor>|None    |
/// |live    |scale             |Scaling factor                 |f64                |1.0     |
/// |live    |fit               |Image fit type                 |ImageFit           |        |
/// |live    |min_width         |Minimum width of the image     |i64                |16      |
/// |live    |min_height        |Minimum height of the image    |i64                |16      |
/// |live    |rotation          |Rotation angle in radians      |f32                |0.0     |
/// | walk   | `abs_pos`           | Absolute position for layout             | `Option<DVec2>`    | `None`   |
/// | walk   | `margin`            | Margin size around the view              | `Margin`           | `Margin::default()` |
/// | walk   | `width`             | Width of the view                        | `Size`             | `Size::default()` |
/// | walk   | `height`            | Height of the view                       | `Size`             | `Size::default()` |
/// | layout | `scroll`            | Scroll position for layout               | `DVec2`            | `(0.0, 0.0)` |
/// | layout | `clip_x`            | Clip content horizontally                | `bool`             | `true`   |
/// | layout | `clip_y`            | Clip content vertically                  | `bool`             | `true`   |
/// | layout | `padding`           | Padding within the view                  | `Padding`          | `Padding::default()` |
/// | layout | `align`             | Alignment for content                    | `Align`            | `Align::default()` |
/// | layout | `flow`              | Flow direction of the content            | `Flow`             | `Flow::default()` |
/// | layout | `spacing`           | Spacing between elements                 | `f64`              | `0.0`    |
/// |live    |draw_image        |The image drawing object       |DrawGView          |        |
/// |live    |src               |Image source dependency        |LiveDependency     |        |
/// |live    |texture           |Texture object                 |Option<Texture>    |None    |
/// |live    |event_key         |Trigger events when true       |bool               |true    |
#[derive(Live, Widget)]
pub struct GImage {
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub grab_key_focus: bool,
    #[live(1.0)]
    pub opacity: f32,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live(1.0)]
    pub scale: f64,
    #[live]
    pub fit: ImageFit,
    #[live(16)]
    pub min_width: i64,
    #[live(16)]
    pub min_height: i64,
    // deref -----------------
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[redraw]
    #[live]
    pub draw_image: DrawGImage,
    #[live]
    pub src: LiveDependency,
    #[rust(Texture::new(cx))]
    pub texture: Option<Texture>,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
}

impl ImageCacheImpl for GImage {
    fn get_texture(&self, _id: usize) -> &Option<Texture> {
        &self.texture
    }

    fn set_texture(&mut self, texture: Option<Texture>, _id: usize) {
        self.texture = texture;
    }
}

impl LiveHook for GImage {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.visible {
            return;
        }

        self.lazy_create_image_cache(cx);
        let src = self.src.clone();
        if !src.as_str().is_empty() {
            let _ = self.load_image_dep_by_path(cx, src.as_str(), 0);
        }
        self.render(cx);
    }
}

impl Widget for GImage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let rect = cx.peek_walk_turtle(walk);
        let dpi = cx.current_dpi_factor();
        let (width, height) = if let Some(image_texture) = &self.texture {
            self.draw_image.draw_vars.set_texture(0, image_texture);
            let (width, height) = image_texture
                .get_format(cx)
                .vec_width_height()
                .unwrap_or((self.min_width as usize, self.min_height as usize));
            (width as f64 * self.scale, height as f64)
        } else {
            self.draw_image.draw_vars.empty_texture(0);
            (self.min_width as f64 / dpi, self.min_height as f64 / dpi)
        };
        let aspect = width / height;
        match self.fit {
            ImageFit::Size => {
                walk.width = Size::Fixed(width);
                walk.height = Size::Fixed(height);
            }
            ImageFit::Stretch => {}
            ImageFit::Horizontal => {
                walk.height = Size::Fixed(rect.size.x / aspect);
            }
            ImageFit::Vertical => {
                walk.width = Size::Fixed(rect.size.y * aspect);
            }
            ImageFit::Smallest => {
                let walk_height = rect.size.x / aspect;
                if walk_height > rect.size.y {
                    walk.width = Size::Fixed(rect.size.y * aspect);
                } else {
                    walk.height = Size::Fixed(walk_height);
                }
            }
            ImageFit::Biggest => {
                let walk_height = rect.size.x / aspect;
                if walk_height < rect.size.y {
                    walk.width = Size::Fixed(rect.size.y * aspect);
                } else {
                    walk.height = Size::Fixed(walk_height);
                }
            }
        }
        self.draw_walk_rotated_image(cx, walk);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );

        self.handle_widget_event(cx, hit, sweep_area)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        let focus_area = self.area();
        let hit = event.hits(cx, self.area());
        self.handle_widget_event(cx, hit, focus_area)
    }
    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl GImage {
    set_scope_path!();
    widget_area! {
        area, draw_image
    }
    active_event! {
        active_hover_in: GImageEvent::HoverIn |e: FingerHoverEvent| => GImageHoverParam{ e },
        active_hover_out: GImageEvent::HoverOut |e: FingerHoverEvent| => GImageHoverParam{ e },
        active_clicked: GImageEvent::Clicked |e: FingerUpEvent| => GImageClickedParam{ e }
    }
    event_option! {
        hover_in: GImageEvent::HoverIn => GImageHoverParam,
        hover_out: GImageEvent::HoverOut => GImageHoverParam,
        clicked: GImageEvent::Clicked => GImageClickedParam
    }
    pub fn redraw(&self, cx: &mut Cx) {
        self.draw_image.redraw(cx);
    }
    pub fn render(&mut self, cx: &mut Cx) {
        self.draw_image.apply_over(
            cx,
            live! {
                image_scale: (self.scale),
                opacity: (self.opacity),
            },
        );

        // self.draw_image.redraw(cx);
        self.lazy_create_image_cache(cx);
        let source = self.src.clone();
        if source.as_str().len() > 0 {
            let _ = self.load_image_dep_by_path(cx, source.as_str(), 0);
        }
    }
    pub fn draw_walk_rotated_image(&mut self, cx: &mut Cx2d, walk: Walk) -> () {
        if let Some(image_texture) = &self.texture {
            self.draw_image.draw_vars.set_texture(0, image_texture);
        }
        self.draw_image.draw_walk(cx, walk);
    }
    pub fn handle_widget_event(&mut self, cx: &mut Cx, hit: Hit, focus_area: Area) {
        match hit {
            Hit::FingerDown(_) => {
                if self.grab_key_focus {
                    cx.set_key_focus(focus_area);
                }
            }
            Hit::FingerHoverIn(e) => {
                let _ = set_cursor(cx, self.cursor.as_ref());
                self.active_hover_in(cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.active_hover_out(cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    self.active_clicked(cx, e);
                }
            }
            _ => (),
        }
    }
}

pub enum SrcType {
    Path(PathBuf),
    Url(String),
    Base64 { data: Vec<u8>, ty: imghdr::Type },
}

impl FromStr for SrcType {
    type Err = ImageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with("data:image") {
            // remove the prefix, split `,`
            let base_slice = s.split(',').collect::<Vec<&str>>();
            let ty = base_slice.get(0).map_or_else(
                || Err(ImageError::UnsupportedFormat),
                |ty| match *ty {
                    "data:image/png;base64" => Ok(imghdr::Type::Png),
                    "data:image/jpeg;base64" => Ok(imghdr::Type::Jpeg),
                    _ => return Err(ImageError::UnsupportedFormat),
                },
            )?;
            base_slice
                .get(1)
                .map_or(Err(ImageError::UnsupportedFormat), |data| {
                    let buf = general_purpose::STANDARD
                        .decode(data)
                        .map_err(|_| ImageError::UnsupportedFormat)?;
                    Ok(SrcType::Base64 { data: buf, ty })
                })
        } else if s.starts_with("http") {
            Ok(SrcType::Url(s.to_string()))
        } else {
            PathBuf::from_str(s)
                .map(|path| SrcType::Path(path))
                .map_err(|_| ImageError::UnsupportedFormat)
        }
    }
}

impl GImageRef {
    /// ## Example
    /// ```rust
    /// let img = self.ui.gimage(id!(img));
    /// // ---- path ----
    /// let current = current_dir().unwrap().join("resources/facebook.png");
    /// img.load(cx, current.display().to_string().as_str()).unwrap();
    /// // ---- base64 ----
    /// let PNG: &str = "data:image/png;base64, ....";
    /// img.load(cx, PNG).unwrap();
    /// // ---- url ----
    /// img.load(cx, "https://avatars.githubusercontent.com/u/67356158?s=48&v=4").unwrap();
    /// ```
    pub fn load(&self, cx: &mut Cx, src: &str) -> Result<(), Box<dyn std::error::Error>> {
        /// load from path as u8
        fn fpath_u8<P>(path: P) -> Result<Vec<u8>, Box<dyn std::error::Error>>
        where
            P: AsRef<Path>,
        {
            let mut file = File::open(path)?;
            let mut content: Vec<u8> = vec![];
            file.read_to_end(&mut content)?;
            Ok(content)
        }

        fn from_path<P>(
            img: &GImageRef,
            cx: &mut Cx,
            path: P,
        ) -> Result<(), Box<dyn std::error::Error>>
        where
            P: AsRef<Path>,
        {
            match imghdr::from_file(path.as_ref())? {
                Some(ty) => match ty {
                    imghdr::Type::Png => {
                        let buf = fpath_u8(path.as_ref())?;
                        let _ = img.load_png_from_data(cx, &buf)?;
                        Ok(())
                    }
                    imghdr::Type::Jpeg => {
                        let buf = fpath_u8(path.as_ref())?;
                        let _ = img.load_jpg_from_data(cx, &buf)?;
                        Ok(())
                    }
                    _ => return Err(ImageError::UnsupportedFormat.into()),
                },
                None => Err(ImageError::UnsupportedFormat.into()),
            }
        }

        fn from_bytes(
            img: &GImageRef,
            cx: &mut Cx,
            buf: Vec<u8>,
        ) -> Result<(), Box<dyn std::error::Error>> {
            match imghdr::from_bytes(&buf) {
                Some(ty) => match ty {
                    imghdr::Type::Png => img.load_png_from_data(cx, &buf).map_err(|e| e.into()),
                    imghdr::Type::Jpeg => img.load_jpg_from_data(cx, &buf).map_err(|e| e.into()),
                    _ => Err(ImageError::UnsupportedFormat.into()),
                },
                None => Err(ImageError::UnsupportedFormat.into()),
            }
        }

        let src_type = SrcType::from_str(src)?;
        let _ = match src_type {
            SrcType::Path(path_buf) => from_path(self, cx, path_buf),
            SrcType::Url(url) => {
                // use reqwest::get do not jam the main thread
                let (sender, reciver) = std::sync::mpsc::channel();
                std::thread::spawn(move || {
                    let buf = reqwest::blocking::get(&url)
                        .map_err(|e| e.to_string())
                        .and_then(|res| res.bytes().map_err(|e| e.to_string()))
                        .map(|bytes| bytes.to_vec());
                    sender.send(buf).unwrap();
                });
                let buf = reciver.recv()??;
                from_bytes(self, cx, buf)
            }
            SrcType::Base64 { data, ty } => match ty {
                imghdr::Type::Png => self.load_png_from_data(cx, &data).map_err(|e| e.into()),
                imghdr::Type::Jpeg => self.load_jpg_from_data(cx, &data).map_err(|e| e.into()),
                _ => Err(ImageError::UnsupportedFormat.into()),
            },
        }?;
        self.redraw(cx);
        Ok(())
    }
    /// Loads the image at the given `image_path` resource into this `ImageRef`.
    pub fn load_image_dep_by_path(&self, cx: &mut Cx, image_path: &str) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_image_dep_by_path(cx, image_path, 0)
        } else {
            Ok(())
        }
    }

    /// Loads the image at the given `image_path` on disk into this `ImageRef`.
    pub fn load_image_file_by_path(&self, cx: &mut Cx, image_path: &str) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_image_file_by_path(cx, image_path, 0)
        } else {
            Ok(())
        }
    }

    /// Loads a JPEG into this `ImageRef` by decoding the given encoded JPEG `data`.
    pub fn load_jpg_from_data(&self, cx: &mut Cx, data: &[u8]) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_jpg_from_data(cx, data, 0)
        } else {
            Ok(())
        }
    }

    /// Loads a PNG into this `ImageRef` by decoding the given encoded PNG `data`.
    pub fn load_png_from_data(&self, cx: &mut Cx, data: &[u8]) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_png_from_data(cx, data, 0)
        } else {
            Ok(())
        }
    }
    prop_setter! {
        GImage{
            set_visible(visible: bool) {|c_ref|{c_ref.visible = visible;}},
            set_grab_key_focus(grab_key_focus: bool) {|c_ref|{c_ref.grab_key_focus = grab_key_focus;}},
            set_opacity(opacity: f32) {|c_ref|{c_ref.opacity = opacity;}},
            set_cursor(cursor: MouseCursor) {|c_ref|{c_ref.cursor.replace(cursor);}},
            set_scale(scale: f64) {|c_ref|{c_ref.scale = scale;}},
            set_fit(fit: ImageFit) {|c_ref|{c_ref.fit = fit;}},
            set_min_width(min_width: i64) {|c_ref|{c_ref.min_width = min_width;}},
            set_min_height(min_height: i64) {|c_ref|{c_ref.min_height = min_height;}},
            set_abs_pos(abs_pos: DVec2) {|c_ref|{c_ref.walk.abs_pos.replace(abs_pos);}},
            set_margin(margin: Margin) {|c_ref|{c_ref.walk.margin = margin;}},
            set_height(height: Size) {|c_ref|{c_ref.walk.height = height;}},
            set_width(width: Size) {|c_ref|{c_ref.walk.width = width;}},
            set_scroll(scroll: DVec2) {|c_ref|{c_ref.layout.scroll = scroll;}},
            set_clip_x(clip_x: bool) {|c_ref|{c_ref.layout.clip_x = clip_x;}},
            set_clip_y(clip_y: bool) {|c_ref|{c_ref.layout.clip_y = clip_y;}},
            set_padding(padding: Padding) {|c_ref|{c_ref.layout.padding = padding;}},
            set_align(align: Align) {|c_ref|{c_ref.layout.align = align;}},
            set_flow(flow: Flow) {|c_ref|{c_ref.layout.flow = flow;}},
            set_spacing(spacing: f64) {|c_ref|{c_ref.layout.spacing = spacing;}},
            set_event_key(event_key: bool) {|c_ref|{c_ref.event_key = event_key;}}
        }
    }
    prop_getter! {
        GImage{
            get_visible(bool) {|| true}, {|c_ref| c_ref.visible},
            get_grab_key_focus(bool) {|| true}, {|c_ref| c_ref.grab_key_focus},
            get_opacity(f32) {|| 1.0}, {|c_ref| c_ref.opacity},
            get_cursor(MouseCursor) {|| Default::default()}, {|c_ref| c_ref.cursor.unwrap_or_default()},
            get_scale(f64) {|| 1.0}, {|c_ref| c_ref.scale},
            get_fit(ImageFit) {|| Default::default()}, {|c_ref| c_ref.fit},
            get_min_width(i64) {|| 16}, {|c_ref| c_ref.min_width},
            get_min_height(i64) {|| 16}, {|c_ref| c_ref.min_height},
            get_abs_pos(Option<DVec2>) {||None}, {|c_ref| {c_ref.walk.abs_pos}},
            get_margin(Margin) {||Margin::default()}, {|c_ref| {c_ref.walk.margin}},
            get_height(Size) {||Size::default()}, {|c_ref| {c_ref.walk.height}},
            get_width(Size) {||Size::default()}, {|c_ref| {c_ref.walk.width}},
            get_scroll(DVec2) {||DVec2::default()}, {|c_ref| {c_ref.layout.scroll}},
            get_clip_x(bool) {||true}, {|c_ref| {c_ref.layout.clip_x}},
            get_clip_y(bool) {||true}, {|c_ref| {c_ref.layout.clip_y}},
            get_padding(Padding) {||Padding::default()}, {|c_ref| {c_ref.layout.padding}},
            get_align(Align) {||Align::default()}, {|c_ref| {c_ref.layout.align}},
            get_flow(Flow) {||Flow::default()}, {|c_ref| {c_ref.layout.flow}},
            get_spacing(f64) {||0.0}, {|c_ref| {c_ref.layout.spacing}},
            get_event_key(bool) {||true}, {|c_ref| c_ref.event_key}
        }
    }
    ref_redraw!();
    ref_area!();
    ref_render!();
    ref_event_option! {
        hover_in => GImageHoverParam,
        hover_out => GImageHoverParam,
        clicked => GImageClickedParam
    }
}

impl GImageSet {
    set_event! {
        hover_in => GImageHoverParam,
        hover_out => GImageHoverParam,
        clicked => GImageClickedParam
    }
}
