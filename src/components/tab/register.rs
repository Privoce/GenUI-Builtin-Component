use makepad_widgets::Cx;

pub fn register(cx: &mut Cx){
    crate::components::tab::header::live_design(cx);
    crate::components::tab::button::live_design(cx);
    crate::components::tab::body::live_design(cx);
    crate::components::tab::pane::live_design(cx);
}