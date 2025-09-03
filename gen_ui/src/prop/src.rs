use std::{path::PathBuf, str::FromStr};

use base64::{engine::general_purpose, Engine};
use makepad_widgets::{image_cache::ImageError, *};

#[derive(Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum Src {
    #[pick]
    None,
    #[live(Default::default())]
    Live(LiveDependency),
    #[live(Default::default())]
    Base64(String),
    #[live(Default::default())]
    Url(String),
    #[live(Default::default())]
    File(String),
}

impl Default for Src {
    fn default() -> Self {
        Src::None
    }
}

impl Src {
    pub fn is_empty(&self) -> bool {
        match self {
            Src::None => true,
            Src::Live(live_dependency) => live_dependency.as_str().is_empty(),
            Src::Base64(b) => b.is_empty(),
            Src::Url(url) => url.is_empty(),
            Src::File(path) => path.is_empty(),
        }
    }
    pub fn is_url(&self) -> bool {
        matches!(self, Src::Url(_))
    }
    pub fn is_file(&self) -> bool {
        matches!(self, Src::File(_))
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Src::None)
    }
    pub fn is_live(&self) -> bool {
        matches!(self, Src::Live(_))
    }
    pub fn is_base64(&self) -> bool {
        matches!(self, Src::Base64(_))
    }
}

impl ToString for Src {
    fn to_string(&self) -> String {
        match self {
            Src::None => "".to_string(),
            Src::Live(live_dependency) => live_dependency.as_str().to_string(),
            Src::Base64(b) => b.to_string(),
            Src::Url(url) => url.to_string(),
            Src::File(path) => path.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
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
        } else if s.starts_with("http") || s.starts_with("https") {
            Ok(SrcType::Url(s.to_string()))
        } else {
            PathBuf::from_str(s)
                .map(|path| SrcType::Path(path))
                .map_err(|_| ImageError::UnsupportedFormat)
        }
    }
}
