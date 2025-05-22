/// # Generate Set Event Function
/// ```rust
/// impl GBreadCrumbItemSet {
///     set_event!{
///        clicked => FingerUpEvent,
///        hover => FingerHoverEvent
///     }
/// }
/// ```
#[macro_export]
macro_rules! set_event {
    ($($event_fn: ident => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                for item in self.iter() {
                    if let Some(e) = item.$event_fn(actions) {
                        return Some(e);
                    }
                }
                None
            }
        )*
    };
}

#[macro_export]
macro_rules! set_event_bool {
    ($($event_fn: ident),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> bool {
                self.iter().any(|c_ref| c_ref.$event_fn(actions))
            }
        )*
    };
}
