use crate::{component_state, components::{traits::ComponentState, view::ViewState}, prop::manuel::BASIC};

component_state! {
    RouterState {
        Basic => BASIC
    }, _ => RouterState::Basic
}

impl ComponentState for RouterState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<RouterState> for ViewState {
    fn from(value: RouterState) -> Self {
        match value {
            RouterState::Basic => ViewState::Basic,
        }
    }
}