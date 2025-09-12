use toml_edit::Table;

use crate::components::{
    ButtonStyle, CardProp, CheckboxProp, CollapseProp, DividerProp, ImageProp, LabelProp, LinkProp,
    MenuItemProp, MenuProp, PopupContainerProp, PopupProp, RadioProp, SubMenuProp, SvgStyle,
    SwitchProp, TabbarItemProp, TabbarProp, TagProp, ViewStyle,
};
use crate::error::Error;
use crate::prop::manuel::{
    BUTTON, CARD, CHECKBOX, COLLAPSE, DIVIDER, IMAGE, LABEL, LINK, MENU, MENU_ITEM, POPUP,
    POPUP_CONTAINER, RADIO, SUB_MENU, SVG, SWITCH, TABBAR, TABBAR_ITEM, TAG, VIEW,
};
use crate::try_from_toml_item;

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf {
    pub label: LabelProp,
    pub view: ViewStyle,
    pub button: ButtonStyle,
    pub card: CardProp,
    pub radio: RadioProp,
    pub checkbox: CheckboxProp,
    pub switch: SwitchProp,
    pub divider: DividerProp,
    pub svg: SvgStyle,
    pub image: ImageProp,
    pub popup: PopupProp,
    pub popup_container: PopupContainerProp,
    pub tabbar: TabbarProp,
    pub tabbar_item: TabbarItemProp,
    pub tag: TagProp,
    pub link: LinkProp,
    pub menu_item: MenuItemProp,
    pub sub_menu: SubMenuProp,
    pub menu: MenuProp,
    pub collapse: CollapseProp,
}

try_from_toml_item! {
    ComponentsConf {
        label => LABEL, LabelProp::default(), |item| item.try_into(),
        view => VIEW, ViewStyle::default(), |item| item.try_into(),
        button => BUTTON, ButtonStyle::default(), |item| item.try_into(),
        card => CARD, CardProp::default(), |item| item.try_into(),
        radio => RADIO, RadioProp::default(), |item| item.try_into(),
        checkbox => CHECKBOX, CheckboxProp::default(), |item| item.try_into(),
        switch => SWITCH, SwitchProp::default(), |item| item.try_into(),
        divider => DIVIDER, DividerProp::default(), |item| item.try_into(),
        svg => SVG, SvgStyle::default(), |item| item.try_into(),
        image => IMAGE, ImageProp::default(), |item| item.try_into(),
        popup => POPUP, PopupProp::default(), |item| item.try_into(),
        popup_container => POPUP_CONTAINER, PopupContainerProp::default(), |item| item.try_into(),
        tabbar => TABBAR, TabbarProp::default(), |item| item.try_into(),
        tabbar_item => TABBAR_ITEM, TabbarItemProp::default(), |item| item.try_into(),
        tag => TAG, TagProp::default(), |item| item.try_into(),
        link => LINK, LinkProp::default(), |item| item.try_into(),
        menu_item => MENU_ITEM, MenuItemProp::default(), |item| item.try_into(),
        sub_menu => SUB_MENU, SubMenuProp::default(), |item| item.try_into(),
        menu => MENU, MenuProp::default(), |item| item.try_into(),
        collapse => COLLAPSE, CollapseProp::default(), |item| item.try_into()
    }, "[components] should be a table"
}

impl From<&ComponentsConf> for Table {
    fn from(value: &ComponentsConf) -> Self {
        let mut table = Table::new();
        table.insert(LABEL, (&value.label).into());
        table.insert(VIEW, (&value.view).into());
        table.insert(BUTTON, (&value.button).into());
        table.insert(CARD, (&value.card).into());
        table.insert(RADIO, (&value.radio).into());
        table.insert(CHECKBOX, (&value.checkbox).into());
        table.insert(SWITCH, (&value.switch).into());
        table.insert(DIVIDER, (&value.divider).into());
        table.insert(SVG, (&value.svg).into());
        table.insert(IMAGE, (&value.image).into());
        table.insert(POPUP, (&value.popup).into());
        table.insert(POPUP_CONTAINER, (&value.popup_container).into());
        table.insert(TABBAR, (&value.tabbar).into());
        table.insert(TABBAR_ITEM, (&value.tabbar_item).into());
        table.insert(TAG, (&value.tag).into());
        table.insert(LINK, (&value.link).into());
        table.insert(MENU_ITEM, (&value.menu_item).into());
        table.insert(SUB_MENU, (&value.sub_menu).into());
        table.insert(MENU, (&value.menu).into());
        table.insert(COLLAPSE, (&value.collapse).into());
        table
    }
}