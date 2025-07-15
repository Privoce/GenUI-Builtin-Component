use crate::components::button::ButtonProp;
use crate::components::card::CardProp;
use crate::components::checkbox::CheckboxProp;
use crate::components::label::LabelProp;
use crate::components::radio::RadioProp;
use crate::components::view::ViewProp;
use crate::error::Error;
use crate::prop::manuel::{BUTTON, CARD, LABEL, RADIO, VIEW, CHECKBOX};
use crate::try_from_toml_item;

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf {
    pub label: LabelProp,
    pub view: ViewProp,
    pub button: ButtonProp,
    pub card: CardProp,
    pub radio: RadioProp,
    pub checkbox: CheckboxProp
}

try_from_toml_item! {
    ComponentsConf {
        label => LABEL, LabelProp::default(), |item| item.try_into(),
        view => VIEW, ViewProp::default(), |item| item.try_into(),
        button => BUTTON, ButtonProp::default(), |item| item.try_into(),
        card => CARD, CardProp::default(), |item| item.try_into(),
        radio => RADIO, RadioProp::default(), |item| item.try_into(),
        checkbox => CHECKBOX, CheckboxProp::default(), |item| item.try_into()
    }, "[components] should be a table"
}
