use makepad_widgets::*;

pub mod label;
pub mod view;
pub mod button;
pub mod traits;

live_design! {
    link luna_ui;

    // use link::luna_theme::*;
    use link::shaders::*;
    use link::luna_basic::*;
    use link::theme::*;

    pub LLabel = <LLabelBase>{
        draw_text: {
            text_style: <THEME_FONT_REGULAR>{}
        }
    }

    pub LView = <LViewBase>{}

    pub LButton = <LButtonBase>{
        slot: <LLabel> {
            text: "Button",
        },
    }
}

pub fn components_register(cx: &mut Cx) {
    label::live_design(cx);
    view::live_design(cx);
    button::live_design(cx);
}
