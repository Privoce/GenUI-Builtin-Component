pub mod manuel;
mod radius;
pub mod traits;
use std::collections::HashMap;

use makepad_widgets::LiveValue;
pub use radius::Radius;

/// ApplyMap is a mapping from a state to a LiveValue, used for applying properties in animations or props
/// means: if in Button, use ApplyMap<ButtonState>
pub type ApplyStateMap<State> = HashMap<State, HashMap<String, LiveValue>>;
