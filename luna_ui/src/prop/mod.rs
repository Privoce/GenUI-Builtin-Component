mod radius;
pub mod traits;
pub mod manuel;
use std::collections::HashMap;

use makepad_widgets::LiveValue;
pub use radius::Radius;


pub type ApplyMap = HashMap<String, LiveValue>;