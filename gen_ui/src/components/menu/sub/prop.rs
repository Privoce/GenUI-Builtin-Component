use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        label::LabelState,
        live_props::LiveProps,
        svg::SvgState,
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::{ViewBasicStyle, ViewState},
        ViewColors,
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        manuel::{ACTIVE, BASIC, BODY, CONTAINER, DISABLED, HEADER},
        traits::NewFrom,
        ApplySlotMapImpl, Applys,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    SubMenuProp {
        basic_prop = SubMenuBasicStyle;
        basic => BASIC, SubMenuBasicStyle::default(), |v| (v, SubMenuState::Basic).try_into(),
        active => ACTIVE, SubMenuBasicStyle::from_state(Theme::default(), SubMenuState::Active), |v| (v, SubMenuState::Active).try_into(),
        disabled => DISABLED, SubMenuBasicStyle::from_state(Theme::default(), SubMenuState::Disabled), |v| (v, SubMenuState::Disabled).try_into()
    }, "[component.sub_menu] should be a table"
}

impl Style for SubMenuProp {
    type State = SubMenuState;

    type Basic = SubMenuBasicStyle;

    get_get_mut! {
        SubMenuState::Basic => basic,
        SubMenuState::Active => active,
        SubMenuState::Disabled => disabled
    }

    fn len() -> usize {
        4 * SubMenuBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotStyle for SubMenuProp {
    type Part = SubMenuPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            SubMenuState::Basic,
            [
                (SubMenuState::Active, &mut self.active),
                (SubMenuState::Disabled, &mut self.disabled),
            ],
            [
                SubMenuPart::Container,
                SubMenuPart::Header,
                SubMenuPart::Body,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SubMenuBasicStyle {
    #[live(SubMenuBasicStyle::default_container(Theme::default(), SubMenuState::Basic))]
    pub container: ViewBasicStyle,
    #[live(SubMenuBasicStyle::default_header(Theme::default(), SubMenuState::Basic))]
    pub header: ViewBasicStyle,
    #[live(SubMenuBasicStyle::default_body(Theme::default(), SubMenuState::Basic))]
    pub body: ViewBasicStyle,
}

from_prop_to_toml! {
    SubMenuBasicStyle {
        container => CONTAINER,
        header => HEADER,
        body => BODY
    }
}

impl BasicStyle for SubMenuBasicStyle {
    type State = SubMenuState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            header: Self::default_header(theme, state),
            body: Self::default_body(theme, state),
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
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(header), ViewBasicStyle::live_props().into()),
            (live_id!(body), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for SubMenuBasicStyle {
    type Part = SubMenuPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SubMenuPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            SubMenuPart::Header => self.header.set_from_str(key, &value.into(), state.into()),
            SubMenuPart::Body => {
                self.body.set_from_str(key, &value.into(), state.into());
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SubMenuPart::Container => self.container.sync(state.into()),
            SubMenuPart::Header => self.header.sync(state.into()),
            SubMenuPart::Body => self.body.sync(state.into()),
        }
    }
}

impl Default for SubMenuBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), SubMenuState::Basic)
    }
}

impl TryFrom<(&Item, SubMenuState)> for SubMenuBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, SubMenuState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(SubMenuBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let header = get_from_itable(
            inline_table,
            HEADER,
            || Ok(SubMenuBasicStyle::default_header(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let body = get_from_itable(
            inline_table,
            BODY,
            || Ok(SubMenuBasicStyle::default_body(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            header,
            body,
        })
    }
}

impl SubMenuBasicStyle {
    pub fn default_container(theme: Theme, state: SubMenuState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_background_visible(true);
        container.set_flow(Flow::Down);
        container.set_margin(Margin::from_f64(0.0));
        container.set_padding(Padding::from_f64(0.0));
        container.set_spacing(0.0);
        container
    }
    pub fn default_header(theme: Theme, state: SubMenuState) -> ViewBasicStyle {
        let mut header = ViewBasicStyle::from_state(theme, state.into());
        header.set_height(Size::Fixed(42.0));
        header.set_width(Size::Fill);
        header.set_background_visible(true);
        header.set_cursor(MouseCursor::Hand);
        header.set_flow(Flow::Right);
        header.set_margin(Margin::from_f64(0.0));
        header
    }
    pub fn default_body(theme: Theme, state: SubMenuState) -> ViewBasicStyle {
        let mut body = ViewBasicStyle::from_state(theme, state.into());
        body.set_height(Size::Fit);
        body.set_width(Size::Fill);
        body.set_background_visible(true);
        body.set_margin(Margin::from_f64(0.0));
        body.set_flow(Flow::Down);
        body
    }
}

component_state! {
    SubMenuState {
        Basic => BASIC,
        Active => ACTIVE,
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
            SubMenuState::Active => ViewState::Pressed,
            SubMenuState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for SubMenuState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => SubMenuState::Basic,
            ViewState::Pressed => SubMenuState::Active,
            ViewState::Disabled => SubMenuState::Disabled,
            ViewState::Hover => SubMenuState::Basic,
        }
    }
}

impl From<SubMenuState> for SvgState {
    fn from(value: SubMenuState) -> Self {
        match value {
            SubMenuState::Basic => SvgState::Basic,
            SubMenuState::Active => SvgState::Pressed,
            SubMenuState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<SubMenuState> for LabelState {
    fn from(value: SubMenuState) -> Self {
        match value {
            SubMenuState::Basic | SubMenuState::Active => LabelState::Basic,
            SubMenuState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    SubMenuPart {
        Container => container => CONTAINER,
        Header => header => HEADER,
        Body => body => BODY
    }, SubMenuState
}
