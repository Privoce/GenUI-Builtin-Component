use makepad_widgets::*;

pub mod label;

live_design!{
    link luna_ui;

    use link::luna_theme::*;
    use link::shaders::*;
    use link::luna_basic::*;

    pub LLabel = <LabelBase>{}
}


pub fn components_register(cx: &mut Cx) {
    label::live_design(cx);
}