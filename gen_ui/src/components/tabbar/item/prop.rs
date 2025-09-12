use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state, components::{
        label::{LabelBasicStyle, LabelState},
        live_props::LiveProps,
        svg::{SvgBasicStyle, SvgPart, SvgState},
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::{ViewBasicStyle, ViewState},
    }, error::Error, from_prop_to_toml, get_get_mut, prop::{
        manuel::{ACTIVE, BASIC, CONTAINER, DISABLED, HOVER, ICON, TEXT},
        traits::NewFrom,
        ApplySlotMapImpl, Radius,
    }, prop_interconvert, themes::Theme, utils::get_from_itable
};

prop_interconvert! {
    TabbarItemProp {
        basic_prop = TabbarItemBasicStyle;
        basic => BASIC, TabbarItemBasicStyle::default(), |v| (v, TabbarItemState::Basic).try_into(),
        hover => HOVER, TabbarItemBasicStyle::from_state(Theme::default(), TabbarItemState::Hover), |v| (v, TabbarItemState::Hover).try_into(),
        active => ACTIVE, TabbarItemBasicStyle::from_state(Theme::default(), TabbarItemState::Active), |v| (v, TabbarItemState::Active).try_into(),
        disabled => DISABLED, TabbarItemBasicStyle::from_state(Theme::default(), TabbarItemState::Disabled), |v| (v, TabbarItemState::Disabled).try_into()
    }, "[component.tabbar_item] should be a table"
}

impl SlotStyle for TabbarItemProp {
    type Part = TabbarItemPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            TabbarItemState::Basic,
            [
                (TabbarItemState::Hover, &mut self.hover),
                (TabbarItemState::Active, &mut self.active),
                (TabbarItemState::Disabled, &mut self.disabled),
            ],
            [
                TabbarItemPart::Container,
                TabbarItemPart::Icon,
                TabbarItemPart::Text,
            ],
        );
    }
}

impl Style for TabbarItemProp {
    type State = TabbarItemState;

    type Basic = TabbarItemBasicStyle;

    get_get_mut! {
        TabbarItemState::Basic => basic,
        TabbarItemState::Hover => hover,
        TabbarItemState::Active => active,
        TabbarItemState::Disabled => disabled
    }

    fn len() -> usize {
        4 * TabbarItemBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct TabbarItemBasicStyle {
    #[live(Self::default_icon(Theme::default(), TabbarItemState::Basic))]
    pub icon: SvgBasicStyle,
    #[live(Self::default_text(Theme::default(), TabbarItemState::Basic))]
    pub text: LabelBasicStyle,
    #[live(Self::default_container(Theme::default(), TabbarItemState::Basic))]
    pub container: ViewBasicStyle,
}

impl Default for TabbarItemBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), TabbarItemState::default())
    }
}

from_prop_to_toml!{
    TabbarItemBasicStyle {
        icon => ICON,
        text => TEXT,
        container => CONTAINER
    }
}

impl SlotBasicStyle for TabbarItemBasicStyle {
    type Part = TabbarItemPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            TabbarItemPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            TabbarItemPart::Icon => {
                // if is slot, key is part, value is key + value
                let icon_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.icon
                        .set_from_str_slot(key, value, state.into(), icon_part);
                }
            }
            TabbarItemPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            TabbarItemPart::Container => self.container.sync(state.into()),
            TabbarItemPart::Icon => {
                self.icon.sync_slot(state.into(), SvgPart::Svg);
                self.icon.sync_slot(state.into(), SvgPart::Container);
            }
            TabbarItemPart::Text => self.text.sync(state.into()),
        }
    }
}

impl BasicStyle for TabbarItemBasicStyle {
    type State = TabbarItemState;

    type Colors = ();

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            icon: Self::default_icon(theme, state),
            text: Self::default_text(theme, state),
            container: Self::default_container(theme, state),
        }
    }

    fn state_colors(_theme: crate::themes::Theme, _state: Self::State) -> Self::Colors {
        ()
    }

    fn len() -> usize {
        ViewBasicStyle::len() + SvgBasicStyle::len() + LabelBasicStyle::len()
    }

    fn set_from_str(
        &mut self,
        _key: &str,
        _value: &makepad_widgets::LiveValue,
        _state: Self::State,
    ) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.container.sync(state.into());
        self.icon.sync(state.into());
        self.text.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(icon), SvgBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
            (live_id!(container), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }

    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl TryFrom<(&Item, TabbarItemState)> for TabbarItemBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, TabbarItemState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.tabbar_item.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(TabbarItemBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(Self::default_icon(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(Self::default_text(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self {
            icon,
            text,
            container,
        })
    }
}

impl TabbarItemBasicStyle {
    pub fn default_container(theme: Theme, state: TabbarItemState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.height = Size::Fill;
        container.width = Size::Fill;
        container.align = Align::from_f64(0.5);
        container.background_visible = false;
        container.cursor = MouseCursor::Hand;
        container.padding = Padding::from_f64(0.0);
        container.spacing = 0.0;
        container
    }
    pub fn default_text(theme: Theme, state: TabbarItemState) -> LabelBasicStyle {
        let mut text = LabelBasicStyle::from_state(theme, state.into());
        text.flow = Flow::Right;
        text
    }
    pub fn default_icon(theme: Theme, state: TabbarItemState) -> SvgBasicStyle {
        let mut icon = SvgBasicStyle::from_state(theme, state.into());
        icon.container.height = Size::Fixed(32.0);
        icon.container.width = Size::Fixed(64.0);
        icon.container.border_radius = Radius::from_f64(8.0);
        icon.container.background_visible = true;
        icon.container.cursor = MouseCursor::Hand;
        icon
    }
}

component_state! {
    TabbarItemState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    },
    _ => TabbarItemState::Basic
}

impl ComponentState for TabbarItemState {
    fn is_disabled(&self) -> bool {
        matches!(self, TabbarItemState::Disabled)
    }
}

impl From<TabbarItemState> for ViewState {
    fn from(value: TabbarItemState) -> Self {
        match value {
            TabbarItemState::Basic => ViewState::Basic,
            TabbarItemState::Hover => ViewState::Hover,
            TabbarItemState::Active => ViewState::Pressed,
            TabbarItemState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for TabbarItemState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => TabbarItemState::Basic,
            ViewState::Hover => TabbarItemState::Hover,
            ViewState::Pressed => TabbarItemState::Active,
            ViewState::Disabled => TabbarItemState::Disabled,
        }
    }
}

impl From<TabbarItemState> for SvgState {
    fn from(value: TabbarItemState) -> Self {
        match value {
            TabbarItemState::Basic => SvgState::Basic,
            TabbarItemState::Hover => SvgState::Hover,
            TabbarItemState::Active => SvgState::Pressed,
            TabbarItemState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<TabbarItemState> for LabelState {
    fn from(value: TabbarItemState) -> Self {
        match value {
            TabbarItemState::Basic | TabbarItemState::Hover | TabbarItemState::Active => {
                LabelState::Basic
            }
            TabbarItemState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    TabbarItemPart {
        Icon => icon => ICON,
        Text => text => TEXT,
        Container => container => CONTAINER
    }, TabbarItemState
}
