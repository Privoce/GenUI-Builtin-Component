use makepad_widgets::*;

use crate::{
    component, components::{button::LButton, card::LCard, label::LLabel, view::LView}
};

pub mod button;
pub mod card;
pub mod checkbox;
pub mod label;
pub mod radio;
pub mod switch;
pub mod tag;
pub mod view;

pub mod lifecycle;
pub mod traits;

use traits::Component;

live_design! {
    link luna_ui;

    // use link::luna_theme::*;
    use link::shaders::*;
    use link::luna_basic::*;
    use link::theme::*;

    pub LLabel = <LLabelBase>{}

    pub LView = <LViewBase>{
        animation_open: false,
        event_open: false,
    }

    pub LButton = <LButtonBase>{
        slot: <LLabel> {
            text: "Button",
        },
    }

    pub LCard = <LCardBase>{
        prop: {
            basic: {
                outer: {
                    height: 300.0,
                    width: 200.0,
                }
            }
        },
        animation_open: false,
        header: <LView> {
            <LLabel> {
                text: "Card Header"
            }
        }
        body: <LView> {
            <LLabel> {
                text: "Card Body"
            }
        }
        footer: <LView> {
            <LLabel> {
                text: "Card Footer"
            }
        }
    }
}

pub fn components_register(cx: &mut Cx) {
    label::live_design(cx);
    view::live_design(cx);
    button::live_design(cx);
    card::live_design(cx);
}

component! {
    Label => LLabel,
    View => LView,
    Button => LButton,
    Card => LCard
}
