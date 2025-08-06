use crate::components::button::ButtonProp;
use crate::components::card::CardProp;
use crate::components::checkbox::CheckboxProp;
use crate::components::divider::DividerProp;
use crate::components::image::ImageProp;
use crate::components::label::LabelProp;
use crate::components::popup::container::PopupContainerProp;
use crate::components::popup::PopupProp;
use crate::components::radio::RadioProp;
use crate::components::svg::SvgProp;
use crate::components::switch::SwitchProp;
// use crate::components::tabbar::item::TabbarItemProp;
use crate::components::view::ViewProp;
use crate::error::Error;
use crate::prop::manuel::{
    BUTTON, CARD, CHECKBOX, DIVIDER, IMAGE, LABEL, POPUP, POPUP_CONTAINER, RADIO, SVG, SWITCH, TABBAR_ITEM, VIEW
};
use crate::try_from_toml_item;

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf {
    pub label: LabelProp,
    pub view: ViewProp,
    pub button: ButtonProp,
    pub card: CardProp,
    pub radio: RadioProp,
    pub checkbox: CheckboxProp,
    pub switch: SwitchProp,
    pub divider: DividerProp,
    pub svg: SvgProp,
    pub image: ImageProp,
    pub popup: PopupProp,
    pub popup_container: PopupContainerProp,
    // pub tabbar_item: TabbarItemProp,
}

try_from_toml_item! {
    ComponentsConf {
        label => LABEL, LabelProp::default(), |item| item.try_into(),
        view => VIEW, ViewProp::default(), |item| item.try_into(),
        button => BUTTON, ButtonProp::default(), |item| item.try_into(),
        card => CARD, CardProp::default(), |item| item.try_into(),
        radio => RADIO, RadioProp::default(), |item| item.try_into(),
        checkbox => CHECKBOX, CheckboxProp::default(), |item| item.try_into(),
        switch => SWITCH, SwitchProp::default(), |item| item.try_into(),
        divider => DIVIDER, DividerProp::default(), |item| item.try_into(),
        svg => SVG, SvgProp::default(), |item| item.try_into(),
        image => IMAGE, ImageProp::default(), |item| item.try_into(),
        popup => POPUP, PopupProp::default(), |item| item.try_into(),
        popup_container => POPUP_CONTAINER, PopupContainerProp::default(), |item| item.try_into()
        // tabbar_item => TABBAR_ITEM, TabbarItemProp::default(), |item| item.try_into()
    }, "[components] should be a table"
}
