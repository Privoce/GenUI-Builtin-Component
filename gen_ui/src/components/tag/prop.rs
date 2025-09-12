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
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        manuel::{BASIC, CLOSE, CONTAINER, DISABLED, HOVER, ICON, PRESSED, TEXT},
        traits::NewFrom,
        ApplySlotMapImpl, Radius,
    },
    prop_interconvert,
    themes::Theme,
    utils::get_from_itable,
};

prop_interconvert! {
    TagProp {
        basic_prop = TagBasicStyle;
        basic => BASIC, TagBasicStyle::default(), |v| (v, TagState::Basic).try_into(),
        hover => HOVER, TagBasicStyle::from_state(Theme::default(), TagState::Hover), |v| (v, TagState::Hover).try_into(),
        pressed => PRESSED, TagBasicStyle::from_state(Theme::default(), TagState::Pressed), |v| (v, TagState::Pressed).try_into(),
        disabled => DISABLED, TagBasicStyle::from_state(Theme::default(), TagState::Disabled), |v| (v, TagState::Disabled).try_into()
    }, "[component.tag] should be a table"
}

impl SlotStyle for TagProp {
    type Part = TagPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            TagState::Basic,
            [
                (TagState::Hover, &mut self.hover),
                (TagState::Pressed, &mut self.pressed),
                (TagState::Disabled, &mut self.disabled),
            ],
            [
                TagPart::Container,
                TagPart::Icon,
                TagPart::Text,
                TagPart::Close,
            ],
        );
    }
}

impl Style for TagProp {
    type State = TagState;

    type Basic = TagBasicStyle;

    get_get_mut! {
        TagState::Basic => basic,
        TagState::Hover => hover,
        TagState::Pressed => pressed,
        TagState::Disabled => disabled
    }

    fn len() -> usize {
        4 * TagBasicStyle::len()
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
pub struct TagBasicStyle {
    #[live(TagBasicStyle::default_icon(Theme::default(), TagState::default()))]
    pub icon: SvgBasicStyle,
    #[live(TagBasicStyle::default_text(Theme::default(), TagState::default()))]
    pub text: LabelBasicStyle,
    #[live(TagBasicStyle::default_close(Theme::default(), TagState::default()))]
    pub close: SvgBasicStyle,
    #[live(TagBasicStyle::default_container(Theme::default(), TagState::default()))]
    pub container: ViewBasicStyle,
}

impl Default for TagBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), TagState::default())
    }
}

impl SlotBasicStyle for TagBasicStyle {
    type Part = TagPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            TagPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            TagPart::Icon => {
                // if is slot, key is part, value is key + value
                let icon_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.icon
                        .set_from_str_slot(key, value, state.into(), icon_part);
                }
            }
            TagPart::Text => self.text.set_from_str(key, &value.into(), state.into()),
            TagPart::Close => {
                let close_part = SvgPart::from_str(key).unwrap();
                for (key, value) in value.as_kvs() {
                    self.close
                        .set_from_str_slot(key, value, state.into(), close_part);
                }
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            TagPart::Container => self.container.sync(state.into()),
            TagPart::Icon => {
                self.icon.sync_slot(state.into(), SvgPart::Svg);
                self.icon.sync_slot(state.into(), SvgPart::Container);
            }
            TagPart::Text => self.text.sync(state.into()),
            TagPart::Close => {
                self.close.sync_slot(state.into(), SvgPart::Svg);
                self.close.sync_slot(state.into(), SvgPart::Container);
            }
        }
    }
}

impl BasicStyle for TagBasicStyle {
    type State = TagState;

    type Colors = ();

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            icon: SvgBasicStyle::from_state(theme, state.into()),
            text: Self::default_text(theme, state),
            close: Self::default_close(theme, state),
            container: Self::default_container(theme, state),
        }
    }

    fn state_colors(_theme: crate::themes::Theme, _state: Self::State) -> Self::Colors {
        ()
    }

    fn len() -> usize {
        ViewBasicStyle::len() + SvgBasicStyle::len() + LabelBasicStyle::len() + SvgBasicStyle::len()
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
        self.close.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(icon), SvgBasicStyle::live_props().into()),
            (live_id!(text), LabelBasicStyle::live_props().into()),
            (live_id!(close), SvgBasicStyle::live_props().into()),
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

from_prop_to_toml! {
    TagBasicStyle {
        icon => ICON,
        text => TEXT,
        close => CLOSE,
        container => CONTAINER
    }
}

impl TryFrom<(&Item, TagState)> for TagBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, TagState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.tag.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(TagBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let icon = get_from_itable(
            inline_table,
            ICON,
            || Ok(SvgBasicStyle::default()),
            |v| (v, state.into()).try_into(),
        )?;

        let text = get_from_itable(
            inline_table,
            TEXT,
            || Ok(Self::default_text(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        let close = get_from_itable(
            inline_table,
            CLOSE,
            || Ok(Self::default_close(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self {
            icon,
            text,
            close,
            container,
        })
    }
}

impl TagBasicStyle {
    pub fn default_container(theme: Theme, state: TagState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.height = Size::Fit;
        container.width = Size::Fit;
        container.align = Align::from_f64(0.5);
        container.background_visible = true;
        container.set_padding(Padding::from_all(4.0, 8.0, 4.0, 8.0));
        container.set_border_radius(Radius::new(2.0));
        container.set_flow(Flow::Right);
        container.set_spacing(4.0);
        container
    }

    pub fn default_text(theme: Theme, state: TagState) -> LabelBasicStyle {
        let mut text = LabelBasicStyle::from_state(theme, state.into());
        text.flow = Flow::Right;
        text.set_font_size(10.0);
        text
    }

    pub fn default_close(theme: Theme, state: TagState) -> SvgBasicStyle {
        SvgBasicStyle::from_state(theme, state.into())
    }

    pub fn default_icon(theme: Theme, state: TagState) -> SvgBasicStyle {
        SvgBasicStyle::from_state(theme, state.into())
    }
}

component_state! {
    TagState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    },
    _ => TagState::Basic
}

impl ComponentState for TagState {
    fn is_disabled(&self) -> bool {
        matches!(self, TagState::Disabled)
    }
}

impl From<TagState> for ViewState {
    fn from(value: TagState) -> Self {
        match value {
            TagState::Basic => ViewState::Basic,
            TagState::Hover => ViewState::Hover,
            TagState::Pressed => ViewState::Pressed,
            TagState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for TagState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => TagState::Basic,
            ViewState::Hover => TagState::Hover,
            ViewState::Pressed => TagState::Pressed,
            ViewState::Disabled => TagState::Disabled,
        }
    }
}

impl From<TagState> for SvgState {
    fn from(value: TagState) -> Self {
        match value {
            TagState::Basic => SvgState::Basic,
            TagState::Hover => SvgState::Hover,
            TagState::Pressed => SvgState::Pressed,
            TagState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<TagState> for LabelState {
    fn from(value: TagState) -> Self {
        match value {
            TagState::Basic | TagState::Hover | TagState::Pressed => LabelState::Basic,
            TagState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    TagPart {
        Icon => icon => ICON,
        Text => text => TEXT,
        Close => close => CLOSE,
        Container => container => CONTAINER
    }, TagState
}
