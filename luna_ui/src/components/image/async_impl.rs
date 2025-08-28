use std::{cell::RefCell, fs::File, io::Read, path::PathBuf};

use makepad_widgets::{
    image_cache::{
        AsyncImageLoad, AsyncLoadResult, ImageBuffer, ImageCache, ImageCacheEntry, ImageCacheImpl,
        ImageError,
    },
    *,
};

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen_futures::spawn_local;

// #[cfg(target_arch = "wasm32")]
// use web_sys::{Request, RequestInit, RequestMode, Response};

pub trait ImageAsync: ImageCacheImpl {
    fn load_from_local<P>(
        &mut self,
        cx: &mut Cx,
        path: P,
        height: usize,
        width: usize,
    ) -> Result<AsyncLoadResult, Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        match self.check_and_convert_load_result(cx, path.as_ref()) {
            Ok(result) => Ok(result),
            Err(_) => {
                if cx.get_global::<ImageCache>().thread_pool.is_none() {
                    cx.get_global::<ImageCache>().thread_pool =
                        Some(TagThreadPool::new(cx, cx.cpu_cores().max(3) - 2))
                }
                let (w, h) = Self::image_size_by_path(path.as_ref()).unwrap_or((width, height));

                cx.get_global::<ImageCache>()
                    .map
                    .insert(path.as_ref().to_path_buf(), ImageCacheEntry::Loading(w, h));
                cx.get_global::<ImageCache>()
                    .thread_pool
                    .as_mut()
                    .unwrap()
                    .execute_rev(path.as_ref().to_path_buf(), move |image_path| {
                        if let Ok(data) = fpath_u8(image_path.as_path()) {
                            if image_path.extension().map(|s| s == "jpg").unwrap_or(false) {
                                match ImageBuffer::from_jpg(&data) {
                                    Ok(data) => {
                                        Cx::post_action(AsyncImageLoad {
                                            image_path,
                                            result: RefCell::new(Some(Ok(data))),
                                        });
                                    }
                                    Err(err) => {
                                        Cx::post_action(AsyncImageLoad {
                                            image_path,
                                            result: RefCell::new(Some(Err(err))),
                                        });
                                    }
                                }
                            } else if image_path.extension().map(|s| s == "png").unwrap_or(false) {
                                match ImageBuffer::from_png(&data) {
                                    Ok(data) => {
                                        Cx::post_action(AsyncImageLoad {
                                            image_path,
                                            result: RefCell::new(Some(Ok(data))),
                                        });
                                    }
                                    Err(err) => {
                                        Cx::post_action(AsyncImageLoad {
                                            image_path,
                                            result: RefCell::new(Some(Err(err))),
                                        });
                                    }
                                }
                            } else {
                                Cx::post_action(AsyncImageLoad {
                                    image_path,
                                    result: RefCell::new(Some(Err(ImageError::UnsupportedFormat))),
                                });
                            }
                        } else {
                            Cx::post_action(AsyncImageLoad {
                                image_path: image_path.clone(),
                                result: RefCell::new(Some(Err(ImageError::PathNotFound(
                                    image_path,
                                )))),
                            });
                        }
                    });
                Ok(AsyncLoadResult::Loading(w, h))
            }
        }
    }
    fn check_and_convert_load_result<P>(
        &mut self,
        cx: &mut Cx,
        path: P,
    ) -> Result<AsyncLoadResult, Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        if let Some(entry) = cx.try_get_image_path(path) {
            match entry {
                ImageCacheEntry::Loaded(texture) => {
                    self.set_texture(Some(texture.clone()), 0);
                    Ok(AsyncLoadResult::Loaded)
                }
                ImageCacheEntry::Loading(w, h) => Ok(AsyncLoadResult::Loading(*w, *h)),
            }
        } else {
            Err(Box::new(ImageError::EmptyData))
        }
    }
    fn load_from_url(
        &mut self,
        cx: &mut Cx,
        url: &str,
        height: usize,
        width: usize,
    ) -> Result<AsyncLoadResult, Box<dyn std::error::Error>> {
        // 创建一个基于 URL 的虚拟路径作为缓存键
        let url = url.to_string();
        let path = PathBuf::from(&url);
        match self.check_and_convert_load_result(cx, path.as_path()) {
            Ok(res) => Ok(res),
            Err(_) => {
                // 由于网络上的图片是需要reqwest来加载的，不知道宽高，所以直接使用传入的宽高即可
                cx.get_global::<ImageCache>().map.insert(
                    path.as_path().to_path_buf(),
                    ImageCacheEntry::Loading(width, height),
                );
                let request = HttpRequest::new(url.to_string(), HttpMethod::GET);
                // let request_id = live_id!(ImageDownload);
                let request_id = LiveId::from_str(&url);
                cx.http_request(request_id, request);
                if cx.get_global::<ImageCache>().thread_pool.is_none() {
                    cx.get_global::<ImageCache>().thread_pool =
                        Some(TagThreadPool::new(cx, cx.cpu_cores().max(3) - 2));
                }
                // cx.get_global::<ImageCache>()
                //     .thread_pool
                //     .as_mut()
                //     .unwrap()
                //     .execute_rev(path.as_path().to_path_buf(), move |image_path| {
                //         // 进行下载
                //         // let result = download_blocking(url);

                //         // Cx::post_action(AsyncImageLoad {
                //         //     image_path: image_path.clone(),
                //         //     result: RefCell::new(Some(result)),
                //         // });

                //     });

                Ok(AsyncLoadResult::Loading(width, height))
            }
        }
    }
}

pub trait TryFromCxImage {
    fn try_get_image_path<P>(&mut self, path: P) -> Option<&ImageCacheEntry>
    where
        P: AsRef<std::path::Path>;
}

impl TryFromCxImage for Cx {
    fn try_get_image_path<P>(&mut self, path: P) -> Option<&ImageCacheEntry>
    where
        P: AsRef<std::path::Path>,
    {
        self.get_global::<ImageCache>().map.get(path.as_ref())
    }
}

pub fn parse_image_buffer(buf: Vec<u8>) -> Result<ImageBuffer, ImageError> {
    match imghdr::from_bytes(&buf) {
        Some(ty) => match ty {
            imghdr::Type::Png => ImageBuffer::from_png(&buf),
            imghdr::Type::Jpeg => ImageBuffer::from_jpg(&buf),
            _ => Err(ImageError::UnsupportedFormat),
        },
        None => Err(ImageError::UnsupportedFormat),
    }
}

/// load from path as u8
pub fn fpath_u8<P>(path: P) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let mut file = File::open(path)?;
    let mut content: Vec<u8> = vec![];
    file.read_to_end(&mut content)?;
    Ok(content)
}
