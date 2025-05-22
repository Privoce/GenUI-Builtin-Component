use makepad_widgets::*;

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