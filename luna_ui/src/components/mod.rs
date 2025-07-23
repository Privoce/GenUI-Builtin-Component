use makepad_widgets::*;

use crate::{
    component, components::{button::GButton, card::GCard, label::GLabel, radio::GRadio, view::GView}
};

pub mod button;
pub mod card;
pub mod checkbox;
pub mod label;
pub mod radio;
pub mod switch;
pub mod tag;
pub mod view;
pub mod svg;
pub mod image;
pub mod divider;
pub mod link;
pub mod loading;
pub mod collapse;
pub mod select;
pub mod popup;
pub mod popover;
pub mod dialog;
pub mod tooltip;
pub mod drawer;

pub mod lifecycle;
pub mod traits;

use traits::Component;

live_design! {
    link luna_ui;

    // use link::luna_theme::*;
    use link::shaders::*;
    use link::genui_basic::*;
    use link::theme::*;

    pub GLabel = <GLabelBase>{}

    pub GView = <GViewBase>{
        animation_open: false,
        event_open: false,
    }

    pub GButton = <GButtonBase>{
        slot: <GLabel> {
            text: "Button",
        },
    }

    pub GCard = <GCardBase>{
        prop: {
            basic: {
                container: {
                    height: 300.0,
                    width: 200.0,
                }
            }
        },
        animation_open: false,
        header: <GView> {
            <GLabel> {
                text: "Card Header"
            }
        }
        body: <GView> {
            <GLabel> {
                text: "Card Body"
            }
        }
        footer: <GView> {
            <GLabel> {
                text: "Card Footer"
            }
        }
    }

    pub GRadio = <GRadioBase> {
        extra: <GView> {
            <GLabel> {
                text: "Extra",
            }
        }
    }
    pub GRadioGroup = <GRadioGroupBase> {}

    pub GCheckbox = <GCheckboxBase> {
        extra: <GView> {
            <GLabel> {
                text: "Extra",
            }
        }
    }

    pub GCheckboxGroup = <GCheckboxGroupBase> {}
    
    pub GSwitch = <GSwitchBase> {}

    pub GDivider = <GDividerBase> {}
}

pub fn components_register(cx: &mut Cx) {
    label::live_design(cx);
    view::live_design(cx);
    button::live_design(cx);
    card::live_design(cx);
    radio::live_design(cx);
    radio::group::live_design(cx);
    checkbox::live_design(cx);
    checkbox::group::live_design(cx);
    switch::live_design(cx);
    divider::live_design(cx);
}

component! {
    Label => GLabel,
    View => GView,
    Button => GButton,
    Card => GCard,
    Radio => GRadio
}
