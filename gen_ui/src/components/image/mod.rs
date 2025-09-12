mod async_impl;
mod prop;

use makepad_widgets::image_cache::{
    AsyncImageLoad, AsyncLoadResult, ImageCache, ImageCacheImpl, ImageError, ImageFit,
};
pub use prop::*;

use crate::components::image::async_impl::{parse_image_buffer, ImageAsync};
use crate::components::lifecycle::LifeCycle;
use crate::components::traits::{BasicStyle, Component, Style};
use crate::error::Error;
use crate::prop::manuel::{BASIC, LOADING};
use crate::prop::{ApplyStateMap, Src, SrcType};
use crate::shader::draw_image::DrawImg;
use crate::themes::conf::Conf;
use crate::{
    animation_open_then_redraw, lifecycle, play_animation, pure_after_apply, set_index,
    set_scope_path, sync, visible, ComponentAnInit,
};
use makepad_widgets::*;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::str::FromStr;

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GImageBase = {{GImage}} {
        animator: {
            loading = {
                default: off,
                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    apply: {
                        draw_img: {load: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    apply: {
                        draw_img: {load: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GImage {
    #[live]
    pub style: ImageProp,
    #[animator]
    animator: Animator,
    #[live]
    pub draw_img: DrawImg,
    #[rust]
    last_time: Option<f64>,
    #[rust]
    animation_frame: f64,
    #[live(true)]
    pub visible: bool,
    #[rust]
    next_frame: NextFrame,
    #[live]
    pub src: Src,
    #[rust]
    async_image_path: Option<PathBuf>,
    #[rust]
    async_image_size: Option<(usize, usize)>,
    #[rust]
    texture: Option<Texture>,
    // --- animation -----------------
    #[live(ImageAnimation::BounceFps(25.0))]
    pub animation: ImageAnimation,
    #[live(true)]
    pub animation_open: bool,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<ImageState>,
    #[rust]
    pub state: ImageState,
}

impl WidgetNode for GImage {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_img.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        // let _ = self.render(cx);
        self.draw_img.redraw(cx);
    }
    fn state(&self) -> String {
        self.state.to_string()
    }
    fn animation_spread(&self) -> bool {
        true
    }
    visible!();
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
    pure_after_apply!();
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &ImageBasicStyle::live_props(),
            [live_id!(basic), live_id!(hover), live_id!(pressed)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(ImageState::Basic, applys);
                }
                LOADING => {
                    component
                        .apply_state_map
                        .insert(ImageState::Loading, applys);
                }
                _ => {}
            },
        );
    }
}

impl Widget for GImage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }
        cx.global::<ComponentAnInit>().image = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        self.handle_widget_event(cx, event, hit, area);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, mut walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        // alright we get a walk. depending on our aspect ratio
        // we change either nothing, or width or height
        let rect = cx.peek_walk_turtle(walk);
        let dpi = cx.current_dpi_factor();
        let style = self.style.get(self.state);
        let (width, height) = if let Some((w, h)) = &self.async_image_size {
            // still loading

            (*w as f64, *h as f64)
        } else if let Some(image_texture) = &self.texture {
            self.draw_img.draw_vars.set_texture(0, image_texture);
            let (width, height) = image_texture
                .get_format(cx)
                .vec_width_height()
                .unwrap_or((style.min_width as usize, style.min_height as usize));
            if let Some(animation) = image_texture.animation(cx) {
                let (w, h) = (animation.width as f64, animation.height as f64);
                self.next_frame = cx.new_next_frame();
                // we have an animation. lets compute the scale and zoom for a certain frame
                let scale_x = w as f32 / width as f32;
                let scale_y = h as f32 / height as f32;
                self.draw_img.image_scale = vec2(scale_x, scale_y);
                (w, h)
            } else {
                self.draw_img.image_scale = vec2(1.0, 1.0);
                self.draw_img.image_pan = vec2(0.0, 0.0);
                (width as f64 * style.width_scale, height as f64)
            }
        } else {
            self.draw_img.draw_vars.empty_texture(0);
            (style.min_width as f64 / dpi, style.min_height as f64 / dpi)
        };

        let aspect = width / height;
        match style.fit {
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

        self.draw_img.draw_walk(cx, walk);

        DrawStep::done()
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncLoad {
    Yes,
    No,
}

impl GImageRef {
    /// Loads the image at the given `image_path` resource into this `ImageRef`.
    pub fn load_image_dep_by_path(&self, cx: &mut Cx, image_path: &str) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_image_dep_by_path(cx, image_path, 0)
        } else {
            Ok(()) // preserving existing behavior of silent failures.
        }
    }

    /// Loads the image at the given `image_path` on disk into this `ImageRef`.
    pub fn load_image_file_by_path(
        &self,
        cx: &mut Cx,
        image_path: &Path,
    ) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_image_file_by_path(cx, image_path, 0)
        } else {
            Ok(()) // preserving existing behavior of silent failures.
        }
    }

    // /// Loads the image at the given `image_path` on disk into this `ImageRef`.
    // pub fn load_image_file_by_path_async(
    //     &self,
    //     cx: &mut Cx,
    //     image_path: &Path,
    // ) -> Result<(), ImageError> {
    //     if let Some(mut inner) = self.borrow_mut() {
    //         return inner.load_image_file_by_path_async(cx, image_path);
    //     }
    //     Ok(())
    // }

    // /// Loads the image at the given `image_path` on disk into this `ImageRef`.
    // pub fn load_image_from_data_async(
    //     &self,
    //     cx: &mut Cx,
    //     image_path: &Path,
    //     data: Arc<Vec<u8>>,
    // ) -> Result<(), ImageError> {
    //     if let Some(mut inner) = self.borrow_mut() {
    //         return inner.load_image_from_data_async(cx, image_path, data);
    //     }
    //     Ok(())
    // }

    /// Loads a JPEG into this `ImageRef` by decoding the given encoded JPEG `data`.
    pub fn load_jpg_from_data(&self, cx: &mut Cx, data: &[u8]) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_jpg_from_data(cx, data, 0)
        } else {
            Ok(()) // preserving existing behavior of silent failures.
        }
    }

    /// Loads a PNG into this `ImageRef` by decoding the given encoded PNG `data`.
    pub fn load_png_from_data(&self, cx: &mut Cx, data: &[u8]) -> Result<(), ImageError> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.load_png_from_data(cx, data, 0)
        } else {
            Ok(()) // preserving existing behavior of silent failures.
        }
    }

    pub fn set_texture(&self, cx: &mut Cx, texture: Option<Texture>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.texture = texture;
            if cx.in_draw_event() {
                inner.redraw(cx);
            }
        }
    }

    pub fn set_uniform(&self, cx: &Cx, uniform: &[LiveId], value: &[f32]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.draw_img.set_uniform(cx, uniform, value);
        }
    }

    /// See [`GImage::size_in_pixels()`].
    pub fn size_in_pixels(&self, cx: &mut Cx) -> Option<(usize, usize)> {
        if let Some(inner) = self.borrow() {
            inner.size_in_pixels(cx)
        } else {
            None
        }
    }

    /// See [`GImage::has_texture()`].
    pub fn has_texture(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.has_texture()
        } else {
            false
        }
    }
}

impl Component for GImage {
    type Error = Error;

    type State = ImageState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.image;
        self.style = style.clone();
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        // only image do not need merge ------------------------------
        // let style = self.style.get(self.state);
        // self.draw_img.merge(&style.into());
        // -----------------------------------------------------------
        self.lazy_create_image_cache(cx);
        match self.src.clone() {
            Src::None => {}
            Src::Live(live_dependency) => {
                if !live_dependency.as_str().is_empty() {
                    let _ = self.load_image_dep_by_path(cx, live_dependency.as_str(), 0);
                }
            }
            _ => {
                let src = self.src.to_string();
                let _ = self.load(cx, &src);
            }
        }

        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, _hit: Hit, _area: Area) {
        animation_open_then_redraw!(self, cx, event);

        // lets check if we have a post action
        if let Event::Actions(actions) = &event {
            for action in actions {
                if let Some(AsyncImageLoad { image_path, result }) = &action.downcast_ref() {
                    if let Some(result) = result.borrow_mut().take() {
                        // we have a result for the image_cache to load up
                        self.process_async_image_load(cx, image_path, result);
                    }
                    if self.async_image_size.is_some()
                        && self.async_image_path.clone() == Some(image_path.to_path_buf())
                    {
                        // see if we can load from cache
                        self.load_image_from_cache(cx, image_path, 0);
                        self.async_image_size = None;
                        self.animator_play(cx, id!(loading.off));
                        self.redraw(cx);
                    }
                }
            }
        } else if let Event::NetworkResponses(response_events) = &event {
            if self.src.is_url() {
                for response_event in response_events {
                    match &response_event.response {
                        NetworkResponse::HttpResponse(response) => {
                            let image_live_id = LiveId::from_str(&self.src.to_string());
                            if response_event.request_id == image_live_id {
                                if response.status_code == 200 {
                                    // 这是图片的下载请求，请求方式为GET，我们需要转为buf
                                    if let Some(buf) = &response.body {
                                        let buf = buf.clone();
                                        let path_str = self.src.to_string();
                                        let path = PathBuf::from(&path_str);
                                        cx.get_global::<ImageCache>()
                                            .thread_pool
                                            .as_mut()
                                            .unwrap()
                                            .execute_rev(path, move |image_path| {
                                                let result = parse_image_buffer(buf);
                                                Cx::post_action(AsyncImageLoad {
                                                    image_path,
                                                    result: RefCell::new(Some(result)),
                                                });
                                            });
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let style = self.style.get(self.state);
        if let Some(nf) = self.next_frame.is_event(event) {
            // compute the next frame and patch things up
            if let Some(image_texture) = &self.texture {
                let (texture_width, texture_height) = image_texture
                    .get_format(cx)
                    .vec_width_height()
                    .unwrap_or((style.min_width as usize, style.min_height as usize));
                if let Some(animation) = image_texture.animation(cx).clone() {
                    let delta = if let Some(last_time) = &self.last_time {
                        nf.time - last_time
                    } else {
                        0.0
                    };
                    self.last_time = Some(nf.time);
                    let num_frames = animation.num_frames as f64;
                    match self.animation {
                        ImageAnimation::Stop => {}
                        ImageAnimation::Frame(frame) => {
                            self.animation_frame = frame;
                        }
                        ImageAnimation::Factor(pos) => {
                            self.animation_frame = pos * (num_frames - 1.0);
                        }
                        ImageAnimation::Once => {
                            self.animation_frame += 1.0;
                            if self.animation_frame >= num_frames {
                                self.animation_frame = num_frames - 1.0;
                            } else {
                                self.next_frame = cx.new_next_frame();
                            }
                        }
                        ImageAnimation::Loop => {
                            self.animation_frame += 1.0;
                            if self.animation_frame >= num_frames {
                                self.animation_frame = 0.0;
                            }
                            self.next_frame = cx.new_next_frame();
                        }
                        ImageAnimation::Bounce => {
                            self.animation_frame += 1.0;
                            if self.animation_frame >= num_frames * 2.0 {
                                self.animation_frame = 0.0;
                            }
                            self.next_frame = cx.new_next_frame();
                        }
                        ImageAnimation::OnceFps(fps) => {
                            self.animation_frame += delta * fps;
                            if self.animation_frame >= num_frames {
                                self.animation_frame = num_frames - 1.0;
                            } else {
                                self.next_frame = cx.new_next_frame();
                            }
                        }
                        ImageAnimation::LoopFps(fps) => {
                            self.animation_frame += delta * fps;
                            if self.animation_frame >= num_frames {
                                self.animation_frame = 0.0;
                            }
                            self.next_frame = cx.new_next_frame();
                        }
                        ImageAnimation::BounceFps(fps) => {
                            self.animation_frame += delta * fps;
                            if self.animation_frame >= num_frames * 2.0 {
                                self.animation_frame = 0.0;
                            }
                            self.next_frame = cx.new_next_frame();
                        }
                    }
                    // alright now lets turn animation_frame into the right image_pan
                    let last_pan = self.draw_img.image_pan;

                    let frame = if self.animation_frame >= num_frames {
                        num_frames * 2.0 - 1.0 - self.animation_frame
                    } else {
                        self.animation_frame
                    } as usize;

                    let horizontal_frames = texture_width / animation.width;
                    let xpos = ((frame % horizontal_frames) * animation.width) as f32
                        / texture_width as f32;
                    let ypos = ((frame / horizontal_frames) * animation.height) as f32
                        / texture_height as f32;
                    self.draw_img.image_pan = vec2(xpos, ypos);
                    if self.draw_img.image_pan != last_pan {
                        // patch it into the area
                        self.draw_img.update_instance_area_value(cx, id!(image_pan))
                    }
                }
            }
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl GImage {
    pub fn load(&mut self, cx: &mut Cx, src: &str) -> Result<(), Box<dyn std::error::Error>> {
        let src_type = SrcType::from_str(src)?;
        let _ = match src_type {
            SrcType::Path(path_buf) => self.load_from_local_break(cx, path_buf.as_path()),
            SrcType::Url(url) => {
                // use reqwest::get do not jam the main thread
                self.animator_play(cx, id!(loading.on));
                self.load_from_url_break(cx, url)
            }
            SrcType::Base64 { data, ty } => match ty {
                imghdr::Type::Png => self.load_png_from_data(cx, &data, 0).map_err(|e| e.into()),
                imghdr::Type::Jpeg => self.load_jpg_from_data(cx, &data, 0).map_err(|e| e.into()),
                _ => Err(ImageError::UnsupportedFormat.into()),
            },
        }?;
        Ok(())
    }

    pub fn load_from_url_break(
        &mut self,
        cx: &mut Cx,
        url: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (height, width) = self.get_size_when_load();
        let result = self.load_from_url(cx, &url, height, width)?;
        self.handle_async_result(cx, result, PathBuf::from(url));
        Ok(())
    }

    pub fn load_from_local_break<P>(
        &mut self,
        cx: &mut Cx,
        image_path: P,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let (height, width) = self.get_size_when_load();
        let result = self.load_from_local(cx, image_path.as_ref(), height, width)?;
        self.handle_async_result(cx, result, image_path);
        Ok(())
    }
    pub fn handle_async_result<P>(&mut self, cx: &mut Cx, result: AsyncLoadResult, path: P) -> ()
    where
        P: AsRef<std::path::Path>,
    {
        match result {
            AsyncLoadResult::Loading(w, h) => {
                self.async_image_size = Some((w, h));
                self.async_image_path = Some(path.as_ref().to_path_buf());
                self.animator_play(cx, id!(loading.on));
                self.redraw(cx);
            }
            AsyncLoadResult::Loaded => {
                self.load_image_from_cache(cx, path.as_ref(), 0);
                self.async_image_size = None;
                self.animator_play(cx, id!(loading.off));
                self.redraw(cx);
            }
        }
    }

    pub fn get_size_when_load(&self) -> (usize, usize) {
        let style = self.style.get(self.state);
        let height = match style.height {
            Size::Fixed(h) => h,
            _ => style.min_height,
        };
        let width = match style.width {
            Size::Fixed(w) => w,
            _ => style.min_width,
        };
        (height as usize, width as usize)
    }

    /// Returns the original size of the image in pixels (not its displayed size).
    ///
    /// Returns `None` if the image has not been loaded into a texture yet.
    pub fn size_in_pixels(&self, cx: &mut Cx) -> Option<(usize, usize)> {
        self.texture
            .as_ref()
            .and_then(|t| t.get_format(cx).vec_width_height())
    }

    /// True if a texture has been set on this `GImage`.
    pub fn has_texture(&self) -> bool {
        self.texture.is_some()
    }
}

impl ImageAsync for GImage {}
