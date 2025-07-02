use makepad_widgets::{DeferWalk, LiveId, SmallVec};

pub type DeferWalks = SmallVec<[(LiveId, DeferWalk); 1]>;
