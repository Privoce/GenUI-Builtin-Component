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
        manuel::{BASIC, CONTAINER, DISABLED, EXTRA, HOVER, ICON, ITEMS, PRESSED, TEXT},
        ApplySlotMapImpl, Applys,
    },
    themes::{Color, Theme},
    try_from_toml_item,
    utils::get_from_itable,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct SubMenuProp {
    #[live(SubMenuBasicProp::default())]
    pub basic: SubMenuBasicProp,
    #[live(SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Hover))]
    pub hover: SubMenuBasicProp,
    #[live(SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Active))]
    pub active: SubMenuBasicProp,
    #[live(SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Disabled))]
    pub disabled: SubMenuBasicProp,
}

impl Prop for SubMenuProp {
    type State = SubMenuState;

    type Basic = SubMenuBasicProp;

    get_get_mut! {
        SubMenuState::Basic => basic,
        SubMenuState::Hover => hover,
        SubMenuState::Active => active,
        SubMenuState::Disabled => disabled
    }

    fn len() -> usize {
        4 * SubMenuBasicProp::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotProp for SubMenuProp {
    type Part = SubMenuPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            SubMenuState::Basic,
            [
                (SubMenuState::Hover, &mut self.hover),
                (SubMenuState::Active, &mut self.active),
                (SubMenuState::Disabled, &mut self.disabled),
            ],
            [
                SubMenuPart::Container,
                SubMenuPart::Icon,
                SubMenuPart::Text,
                SubMenuPart::Extra,
                SubMenuPart::Items,
            ],
        );
    }
}

try_from_toml_item! {
    SubMenuProp {
        basic => BASIC, SubMenuBasicProp::default(), |v| (v, SubMenuState::Basic).try_into(),
        hover => HOVER, SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Hover), |v| (v, SubMenuState::Hover).try_into(),
        active => PRESSED, SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Active), |v| (v, SubMenuState::Active).try_into(),
        disabled => DISABLED, SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Disabled), |v| (v, SubMenuState::Disabled).try_into()
    }, "[component.menu_item] should be a table"
}

impl Default for SubMenuProp {
    fn default() -> Self {
        Self {
            basic: SubMenuBasicProp::default(),
            hover: SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Hover),
            active: SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Active),
            disabled: SubMenuBasicProp::from_state(Theme::default(), SubMenuState::Disabled),
        }
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SubMenuBasicProp {
    #[live(SubMenuBasicProp::default_container(Theme::default(), SubMenuState::Basic))]
    pub container: ViewBasicProp,
    #[live(SubMenuBasicProp::default_icon(Theme::default(), SubMenuState::Basic))]
    pub icon: SvgBasicProp,
    #[live(SubMenuBasicProp::default_text(Theme::default(), SubMenuState::Basic))]
    pub text: LabelBasicProp,
    #[live(SubMenuBasicProp::default_extra(Theme::default(), SubMenuState::Basic))]
    pub extra: ViewBasicProp,
    #[live(SubMenuBasicProp::default_items(Theme::default(), SubMenuState::Basic))]
    pub items: ViewBasicProp,
}

impl BasicProp for SubMenuBasicProp {
    type State = SubMenuState;

    type Colors = (Color, Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            icon: Self::default_icon(theme, state),
            text: Self::default_text(theme, state),
            extra: Self::default_extra(theme, state),
            items: Self::default_items(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicProp::state_colors(theme, state.into())
    }

    fn len() -> usize {
        0
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.icon.sync(state.into());
        self.text.sync(state.into());
        self.extra.sync(state.into());
        self.items.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicProp::live_props().into()),
            (live_id!(header), ViewBasicProp::live_props().into()),
            (live_id!(body), ViewBasicProp::live_props().into()),
            (live_id!(footer), ViewBasicProp::live_props().into()),
            (live_id!(items), ViewBasicProp::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicProp for SubMenuBasicProp {
    type Part = SubMenuPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SubMenuPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            SubMenuPart::Icon => {
                let icon_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.icon
                        .set_from_str_slot(key, value, state.into(), icon_part);
                }
            }
            SubMenuPart::Text => {
                self.text.set_from_str(key, &value.into(), state.into());
            }
            SubMenuPart::Extra => {
                self.extra.set_from_str(key, &value.into(), state.into());
            }
            SubMenuPart::Items => {
                self.items.set_from_str(key, &value.into(), state.into());
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SubMenuPart::Container => self.container.sync(state.into()),
            SubMenuPart::Icon => {
                self.icon.sync_slot(state.into(), SvgPart::Svg);
                self.icon.sync_slot(state.into(), SvgPart::Container);
            }
            SubMenuPart::Text => self.text.sync(state.into()),
            SubMenuPart::Extra => self.extra.sync(state.into()),
            SubMenuPart::Items => self.items.sync(state.into()),
        }
    }
}

impl Default for SubMenuBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), SubMenuState::Basic)
    }
}

impl TryFrom<(&Item, SubMenuState)> for SubMenuBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, SubMenuState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(SubMenuBasicProp::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(SubMenuBasicProp::default_icon(Theme::default(), state)),
            |v| (v, SvgState::from(state)).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(SubMenuBasicProp::default_text(Theme::default(), state)),
            |v| (v, LabelState::from(state)).try_into(),
        )?;

        let extra = get_from_itable(
            inline_table,
            EXTRA,
            || Ok(SubMenuBasicProp::default_extra(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let items = get_from_itable(
            inline_table,
            ITEMS,
            || Ok(SubMenuBasicProp::default_items(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            icon,
            text,
            extra,
            items,
        })
    }
}

impl SubMenuBasicProp {
    pub fn default_container(theme: Theme, state: SubMenuState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_background_visible(true);
        container.set_flow(Flow::Right);
        container
    }
    pub fn default_icon(theme: Theme, state: SubMenuState) -> SvgBasicProp {
        let icon = SvgBasicProp::from_state(theme, state.into());
        icon
    }
    pub fn default_text(theme: Theme, state: SubMenuState) -> LabelBasicProp {
        LabelBasicProp::from_state(theme, state.into())
    }
    pub fn default_extra(theme: Theme, state: SubMenuState) -> ViewBasicProp {
        let mut extra = ViewBasicProp::from_state(theme, state.into());
        extra.set_height(Size::Fill);
        extra.set_width(Size::Fill);
        extra
    }
    pub fn default_items(theme: Theme, state: SubMenuState) -> ViewBasicProp {
        let mut items = ViewBasicProp::from_state(theme, state.into());
        items.set_height(Size::Fit);
        items.set_width(Size::Fill);
        items.set_flow(Flow::Down);
        items
    }
}

component_state! {
    SubMenuState {
        Basic => BASIC,
        Hover => HOVER,
        Active => PRESSED,
        Disabled => DISABLED
    }, _ => SubMenuState::Basic
}

impl ComponentState for SubMenuState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<SubMenuState> for ViewState {
    fn from(value: SubMenuState) -> Self {
        match value {
            SubMenuState::Basic => ViewState::Basic,
            SubMenuState::Hover => ViewState::Hover,
            SubMenuState::Active => ViewState::Pressed,
            SubMenuState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for SubMenuState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => SubMenuState::Basic,
            ViewState::Hover => SubMenuState::Hover,
            ViewState::Pressed => SubMenuState::Active,
            ViewState::Disabled => SubMenuState::Disabled,
        }
    }
}

impl From<SubMenuState> for SvgState {
    fn from(value: SubMenuState) -> Self {
        match value {
            SubMenuState::Basic => SvgState::Basic,
            SubMenuState::Hover => SvgState::Hover,
            SubMenuState::Active => SvgState::Pressed,
            SubMenuState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<SubMenuState> for LabelState {
    fn from(value: SubMenuState) -> Self {
        match value {
            SubMenuState::Basic | SubMenuState::Hover | SubMenuState::Active => {
                LabelState::Basic
            }
            SubMenuState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    SubMenuPart {
        Container => container => CONTAINER,
        Icon => icon => ICON,
        Text => text => TEXT,
        Extra => extra => EXTRA,
        Items => items => ITEMS
    }, SubMenuState
}
