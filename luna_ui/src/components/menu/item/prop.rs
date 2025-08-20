use std::str::FromStr;

use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        label::{LabelBasicProp, LabelState},
        live_props::LiveProps,
        svg::{SvgBasicProp, SvgPart, SvgState},
        traits::{BasicProp, ComponentState, Part, Prop, SlotBasicProp, SlotProp},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    get_get_mut,
    prop::{
        manuel::{BASIC, CONTAINER, DISABLED, EXTRA, HOVER, ICON, PRESSED, TEXT},
        ApplySlotMapImpl, Applys,
    },
    themes::{Color, Theme},
    try_from_toml_item,
    utils::get_from_itable,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct MenuItemProp {
    #[live(MenuItemBasicProp::default())]
    pub basic: MenuItemBasicProp,
    #[live(MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Hover))]
    pub hover: MenuItemBasicProp,
    #[live(MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Active))]
    pub active: MenuItemBasicProp,
    #[live(MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Disabled))]
    pub disabled: MenuItemBasicProp,
}

impl Prop for MenuItemProp {
    type State = MenuItemState;

    type Basic = MenuItemBasicProp;

    get_get_mut! {
        MenuItemState::Basic => basic,
        MenuItemState::Hover => hover,
        MenuItemState::Active => active,
        MenuItemState::Disabled => disabled
    }

    fn len() -> usize {
        4 * MenuItemBasicProp::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotProp for MenuItemProp {
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

try_from_toml_item! {
    MenuItemProp {
        basic => BASIC, MenuItemBasicProp::default(), |v| (v, MenuItemState::Basic).try_into(),
        hover => HOVER, MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Hover), |v| (v, MenuItemState::Hover).try_into(),
        active => PRESSED, MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Active), |v| (v, MenuItemState::Active).try_into(),
        disabled => DISABLED, MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Disabled), |v| (v, MenuItemState::Disabled).try_into()
    }, "[component.menu_item] should be a table"
}

impl Default for MenuItemProp {
    fn default() -> Self {
        Self {
            basic: MenuItemBasicProp::default(),
            hover: MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Hover),
            active: MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Active),
            disabled: MenuItemBasicProp::from_state(Theme::default(), MenuItemState::Disabled),
        }
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct MenuItemBasicProp {
    #[live(MenuItemBasicProp::default_container(Theme::default(), MenuItemState::Basic))]
    pub container: ViewBasicProp,
    #[live(MenuItemBasicProp::default_icon(Theme::default(), MenuItemState::Basic))]
    pub icon: SvgBasicProp,
    #[live(MenuItemBasicProp::default_text(Theme::default(), MenuItemState::Basic))]
    pub text: LabelBasicProp,
    #[live(MenuItemBasicProp::default_extra(Theme::default(), MenuItemState::Basic))]
    pub extra: ViewBasicProp,
}

impl BasicProp for MenuItemBasicProp {
    type State = MenuItemState;

    type Colors = (Color, Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            icon: Self::default_icon(theme, state),
            text: Self::default_text(theme, state),
            extra: Self::default_extra(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicProp::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicProp::len()
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
            (live_id!(container), ViewBasicProp::live_props().into()),
            (live_id!(header), ViewBasicProp::live_props().into()),
            (live_id!(body), ViewBasicProp::live_props().into()),
            (live_id!(footer), ViewBasicProp::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicProp for MenuItemBasicProp {
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

impl Default for MenuItemBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), MenuItemState::Basic)
    }
}

impl TryFrom<(&Item, MenuItemState)> for MenuItemBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, MenuItemState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(MenuItemBasicProp::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(MenuItemBasicProp::default_icon(Theme::default(), state)),
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(MenuItemBasicProp::default_text(Theme::default(), state)),
            |v| (v, LabelState::from(state)).try_into(),
        )?;

        let extra = get_from_itable(
            inline_table,
            EXTRA,
            || Ok(MenuItemBasicProp::default_extra(Theme::default(), state)),
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

impl MenuItemBasicProp {
    pub fn default_container(theme: Theme, state: MenuItemState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_background_visible(true);
        container.set_flow(Flow::Right);
        container
    }
    pub fn default_icon(theme: Theme, state: MenuItemState) -> SvgBasicProp {
        let icon = SvgBasicProp::from_state(theme, state.into());
        icon
    }
    pub fn default_text(theme: Theme, state: MenuItemState) -> LabelBasicProp {
        LabelBasicProp::from_state(theme, state.into())
    }
    pub fn default_extra(theme: Theme, state: MenuItemState) -> ViewBasicProp {
        let mut extra = ViewBasicProp::from_state(theme, state.into());
        extra.set_height(Size::Fill);
        extra.set_width(Size::Fill);
        extra
    }
}

component_state! {
    MenuItemState {
        Basic => BASIC,
        Hover => HOVER,
        Active => PRESSED,
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
