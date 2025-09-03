use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Prop},
        ViewBasicProp, ViewState,
    },
    error::Error,
    from_inherit_to_view_basic_prop, get_get_mut, inherits_view_basic_prop,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, CURSOR, DISABLED, FLOW, HEIGHT, MARGIN,
            PADDING, ROTATION, SCALE, SHADOW_COLOR, SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME,
            WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl, Radius,
    },
    state_colors,
    themes::{Color, Theme, TomlValueTo},
    try_from_toml_item,
    utils::get_from_itable,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct TabbarProp {
    #[live(TabbarBasicProp::default())]
    pub basic: TabbarBasicProp,
    #[live(TabbarBasicProp::from_state(Theme::default(), TabbarState::Disabled))]
    pub disabled: TabbarBasicProp,
}

impl Default for TabbarProp {
    fn default() -> Self {
        Self {
            basic: TabbarBasicProp::default(),
            disabled: TabbarBasicProp::from_state(Theme::default(), TabbarState::Disabled),
        }
    }
}

impl Prop for TabbarProp {
    type State = TabbarState;
    type Basic = TabbarBasicProp;

    get_get_mut! {
        TabbarState::Basic => basic,
        TabbarState::Disabled => disabled
    }

    fn len() -> usize {
        TabbarBasicProp::len() * 3
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            TabbarState::Basic,
            [(TabbarState::Disabled, &mut self.disabled)],
        );
    }
}

try_from_toml_item! {
    TabbarProp {
        basic => BASIC, TabbarBasicProp::default(),|v| (v, TabbarState::Basic).try_into(),
        disabled => DISABLED, TabbarBasicProp::from_state(Theme::default(), TabbarState::Disabled), |v| (v, TabbarState::Disabled).try_into()
    }, "[component.tabbar] should be a table"
}

inherits_view_basic_prop! {
    TabbarBasicProp {
        border_width: 0.0,
        border_radius: Radius::new(0.0),
        spread_radius: 0.0,
        blur_radius: 0.0,
        shadow_offset: vec2(0.0, 0.0),
        background_visible: true,
        rotation: 0.0,
        scale: 1.0,
        padding: Padding::from_f64(0.0),
        margin: Margin::from_f64(0.0),
        clip_x: false,
        clip_y: false,
        align: Align::from_f64(0.5),
        cursor: MouseCursor::default(),
        flow: Flow::Right,
        spacing: 8.0,
        height: Size::Fixed(64.0),
        width: Size::Fill,
        abs_pos: None,
    },
    TabbarState,
    "tabbar",
    {
        TabbarState::Basic => (500, 500, 400),
        TabbarState::Disabled => (300, 300, 200)
    }
}

from_inherit_to_view_basic_prop!(TabbarBasicProp);

component_state! {
    TabbarState {
        Basic => BASIC,
        Disabled => DISABLED
    },
    _ => TabbarState::Basic
}

impl From<TabbarState> for ViewState {
    fn from(state: TabbarState) -> Self {
        match state {
            TabbarState::Basic => ViewState::Basic,
            TabbarState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for TabbarState {
    fn from(state: ViewState) -> Self {
        match state {
            ViewState::Basic => TabbarState::Basic,
            ViewState::Disabled => TabbarState::Disabled,
            _ => TabbarState::Basic,
        }
    }
}

impl ComponentState for TabbarState {
    fn is_disabled(&self) -> bool {
        matches!(self, TabbarState::Disabled)
    }
}
