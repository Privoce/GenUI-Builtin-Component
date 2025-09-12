use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        label::{LabelBasicStyle, LabelState},
        live_props::LiveProps,
        svg::{SvgBasicStyle, SvgPart, SvgState},
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::{ViewBasicStyle, ViewState},
        ViewColors,
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        manuel::{ACTIVE, BASIC, CONTAINER, DISABLED, EXTRA, HOVER, ICON, TEXT},
        traits::NewFrom,
        ApplySlotMapImpl, Applys,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    MenuItemProp {
        basic_prop = MenuItemBasicStyle;
        basic => BASIC, MenuItemBasicStyle::default(), |v| (v, MenuItemState::Basic).try_into(),
        hover => HOVER, MenuItemBasicStyle::from_state(Theme::default(), MenuItemState::Hover), |v| (v, MenuItemState::Hover).try_into(),
        active => ACTIVE, MenuItemBasicStyle::from_state(Theme::default(), MenuItemState::Active), |v| (v, MenuItemState::Active).try_into(),
        disabled => DISABLED, MenuItemBasicStyle::from_state(Theme::default(), MenuItemState::Disabled), |v| (v, MenuItemState::Disabled).try_into()
    }, "[component.menu_item] should be a table"
}

impl Style for MenuItemProp {
    type State = MenuItemState;

    type Basic = MenuItemBasicStyle;

    get_get_mut! {
        MenuItemState::Basic => basic,
        MenuItemState::Hover => hover,
        MenuItemState::Active => active,
        MenuItemState::Disabled => disabled
    }

    fn len() -> usize {
        4 * MenuItemBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotStyle for MenuItemProp {
    type Part = MenuItemPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            MenuItemState::Basic,
            [
                (MenuItemState::Hover, &mut self.hover),
                (MenuItemState::Active, &mut self.active),
                (MenuItemState::Disabled, &mut self.disabled),
            ],
            [
                MenuItemPart::Container,
                MenuItemPart::Icon,
                MenuItemPart::Text,
                MenuItemPart::Extra,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct MenuItemBasicStyle {
    #[live(MenuItemBasicStyle::default_container(Theme::default(), MenuItemState::Basic))]
    pub container: ViewBasicStyle,
    #[live(MenuItemBasicStyle::default_icon(Theme::default(), MenuItemState::Basic))]
    pub icon: SvgBasicStyle,
    #[live(MenuItemBasicStyle::default_text(Theme::default(), MenuItemState::Basic))]
    pub text: LabelBasicStyle,
    #[live(MenuItemBasicStyle::default_extra(Theme::default(), MenuItemState::Basic))]
    pub extra: ViewBasicStyle,
}

from_prop_to_toml! {
    MenuItemBasicStyle {
        container => CONTAINER,
        icon => ICON,
        text => TEXT,
        extra => EXTRA
    }
}

impl BasicStyle for MenuItemBasicStyle {
    type State = MenuItemState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            icon: Self::default_icon(theme, state),
            text: Self::default_text(theme, state),
            extra: Self::default_extra(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicStyle::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicStyle::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.icon.sync(state.into());
        self.text.sync(state.into());
        self.extra.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(header), ViewBasicStyle::live_props().into()),
            (live_id!(body), ViewBasicStyle::live_props().into()),
            (live_id!(footer), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for MenuItemBasicStyle {
    type Part = MenuItemPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            MenuItemPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            MenuItemPart::Icon => {
                let icon_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.icon
                        .set_from_str_slot(key, value, state.into(), icon_part);
                }
            }
            MenuItemPart::Text => {
                self.text.set_from_str(key, &value.into(), state.into());
            }
            MenuItemPart::Extra => {
                self.extra.set_from_str(key, &value.into(), state.into());
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            MenuItemPart::Container => self.container.sync(state.into()),
            MenuItemPart::Icon => {
                self.icon.sync_slot(state.into(), SvgPart::Svg);
                self.icon.sync_slot(state.into(), SvgPart::Container);
            }
            MenuItemPart::Text => self.text.sync(state.into()),
            MenuItemPart::Extra => self.extra.sync(state.into()),
        }
    }
}

impl Default for MenuItemBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), MenuItemState::Basic)
    }
}

impl TryFrom<(&Item, MenuItemState)> for MenuItemBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, MenuItemState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(MenuItemBasicStyle::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(MenuItemBasicStyle::default_icon(Theme::default(), state)),
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(MenuItemBasicStyle::default_text(Theme::default(), state)),
            |v| (v, LabelState::from(state)).try_into(),
        )?;

        let extra = get_from_itable(
            inline_table,
            EXTRA,
            || Ok(MenuItemBasicStyle::default_extra(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            icon,
            text,
            extra,
        })
    }
}

impl MenuItemBasicStyle {
    pub fn default_container(theme: Theme, state: MenuItemState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_background_visible(true);
        container.set_flow(Flow::Right);
        container.set_cursor(MouseCursor::Hand);
        container.set_margin(Margin::from_f64(0.0));
        container
    }
    pub fn default_icon(theme: Theme, state: MenuItemState) -> SvgBasicStyle {
        let icon = SvgBasicStyle::from_state(theme, state.into());
        icon
    }
    pub fn default_text(theme: Theme, state: MenuItemState) -> LabelBasicStyle {
        LabelBasicStyle::from_state(theme, state.into())
    }
    pub fn default_extra(theme: Theme, state: MenuItemState) -> ViewBasicStyle {
        let mut extra = ViewBasicStyle::from_state(theme, state.into());
        extra.set_height(Size::Fill);
        extra.set_width(Size::Fit);
        extra
    }
}

component_state! {
    MenuItemState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    }, _ => MenuItemState::Basic
}

impl ComponentState for MenuItemState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<MenuItemState> for ViewState {
    fn from(value: MenuItemState) -> Self {
        match value {
            MenuItemState::Basic => ViewState::Basic,
            MenuItemState::Hover => ViewState::Hover,
            MenuItemState::Active => ViewState::Pressed,
            MenuItemState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for MenuItemState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => MenuItemState::Basic,
            ViewState::Hover => MenuItemState::Hover,
            ViewState::Pressed => MenuItemState::Active,
            ViewState::Disabled => MenuItemState::Disabled,
        }
    }
}

impl From<MenuItemState> for SvgState {
    fn from(value: MenuItemState) -> Self {
        match value {
            MenuItemState::Basic => SvgState::Basic,
            MenuItemState::Hover => SvgState::Hover,
            MenuItemState::Active => SvgState::Pressed,
            MenuItemState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<MenuItemState> for LabelState {
    fn from(value: MenuItemState) -> Self {
        match value {
            MenuItemState::Basic | MenuItemState::Hover | MenuItemState::Active => {
                LabelState::Basic
            }
            MenuItemState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    MenuItemPart {
        Container => container => CONTAINER,
        Icon => icon => ICON,
        Text => text => TEXT,
        Extra => extra => EXTRA
    }, MenuItemState
}
