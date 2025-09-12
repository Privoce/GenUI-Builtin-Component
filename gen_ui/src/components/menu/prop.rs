use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::{ViewBasicStyle, ViewState},
        ViewColors,
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        manuel::{BASIC, BODY, CONTAINER, FOOTER, HEADER},
        traits::NewFrom,
        ApplySlotMapImpl, ApplyStateMapImpl, Applys,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    MenuProp {
        basic_prop = MenuBasicStyle;
        basic => BASIC, MenuBasicStyle::default(), |v| (v, MenuState::Basic).try_into()
    }, "[component.menu] should be a table"
}

impl Style for MenuProp {
    type State = MenuState;

    type Basic = MenuBasicStyle;

    get_get_mut! {
        MenuState::Basic => basic
    }

    fn len() -> usize {
        2 * MenuBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, MenuState::Basic, []);
    }
}

impl SlotStyle for MenuProp {
    type Part = MenuPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            MenuState::Basic,
            [],
            [
                MenuPart::Container,
                MenuPart::Header,
                MenuPart::Body,
                MenuPart::Footer,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct MenuBasicStyle {
    #[live(MenuBasicStyle::default_container(Theme::default(), MenuState::Basic))]
    pub container: ViewBasicStyle,
    #[live(MenuBasicStyle::default_header(Theme::default(), MenuState::Basic))]
    pub header: ViewBasicStyle,
    #[live(MenuBasicStyle::default_body(Theme::default(), MenuState::Basic))]
    pub body: ViewBasicStyle,
    #[live(MenuBasicStyle::default_footer(Theme::default(), MenuState::Basic))]
    pub footer: ViewBasicStyle,
}

from_prop_to_toml! {
    MenuBasicStyle {
        container => CONTAINER,
        header => HEADER,
        body => BODY,
        footer => FOOTER
    }
}

impl BasicStyle for MenuBasicStyle {
    type State = MenuState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            header: Self::default_header(theme, state),
            body: Self::default_body(theme, state),
            footer: Self::default_footer(theme, state),
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
        self.header.sync(state.into());
        self.body.sync(state.into());
        self.footer.sync(state.into());
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

impl SlotBasicStyle for MenuBasicStyle {
    type Part = MenuPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            MenuPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            MenuPart::Header => self.header.set_from_str(key, &value.into(), state.into()),
            MenuPart::Body => self.body.set_from_str(key, &value.into(), state.into()),
            MenuPart::Footer => self.footer.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            MenuPart::Container => self.container.sync(state.into()),
            MenuPart::Header => self.header.sync(state.into()),
            MenuPart::Body => self.body.sync(state.into()),
            MenuPart::Footer => self.footer.sync(state.into()),
        }
    }
}

impl Default for MenuBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), MenuState::Basic)
    }
}

impl TryFrom<(&Item, MenuState)> for MenuBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, MenuState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.menu.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(MenuBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let header = get_from_itable(
            inline_table,
            HEADER,
            || Ok(MenuBasicStyle::default_header(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let body = get_from_itable(
            inline_table,
            BODY,
            || Ok(MenuBasicStyle::default_body(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let footer = get_from_itable(
            inline_table,
            FOOTER,
            || Ok(MenuBasicStyle::default_footer(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            header,
            body,
            footer,
        })
    }
}

impl MenuBasicStyle {
    pub fn default_header(theme: Theme, state: MenuState) -> ViewBasicStyle {
        let mut header = Self::default_container(theme, state);
        header.set_height(Size::Fit);
        header.set_width(Size::Fill);
        header
    }
    pub fn default_footer(theme: Theme, state: MenuState) -> ViewBasicStyle {
        Self::default_header(theme, state)
    }
    pub fn default_container(theme: Theme, state: MenuState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_width(Size::Fixed(300.0));
        container.set_height(Size::Fill);
        container.set_background_visible(true);
        container.set_clip_y(true);
        container.set_clip_x(true);
        container
    }
    pub fn default_body(theme: Theme, state: MenuState) -> ViewBasicStyle {
        let mut body = Self::default_container(theme, state);
        body.set_height(Size::Fill);
        body.set_width(Size::Fill);
        body.set_padding(Padding::from_f64(0.0));
        body
    }
}

component_state! {
    MenuState {
        Basic => BASIC
    }, _ => MenuState::Basic
}

impl ComponentState for MenuState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<MenuState> for ViewState {
    fn from(value: MenuState) -> Self {
        match value {
            MenuState::Basic => ViewState::Basic,
        }
    }
}

impl From<ViewState> for MenuState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => MenuState::Basic,
            _ => panic!("MenuState can only be Basic"),
        }
    }
}

component_part! {
    MenuPart {
        Container => container => CONTAINER,
        Header => header => HEADER,
        Body => body => BODY,
        Footer => footer => FOOTER
    }, MenuState
}
