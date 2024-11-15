use makepad_widgets::Cx;

pub fn register(cx: &mut Cx){
    crate::components::colors::register::register(cx);
    crate::components::label::register::register(cx);
    crate::components::link::register::register(cx);
    crate::components::view::register::register(cx);
    crate::components::svg::register::register(cx);
    crate::components::icon::register::register(cx);
    crate::components::button::register::register(cx);
    crate::components::tag::register::register(cx);
    crate::components::image::register::register(cx);
    crate::components::breadcrumb::register::register(cx);
    crate::components::toggle::register::register(cx);
    crate::components::radio::register::register(cx);
    crate::components::checkbox::register::register(cx);
    crate::components::loading::register::register(cx);
    crate::components::divider::register::register(cx);
    crate::components::file_upload::register::register(cx);
    crate::components::progress::register::register(cx);
    crate::components::collapse::register::register(cx);
    crate::components::shader::register::register(cx);
    crate::components::input::register::register(cx);
    crate::components::popup::register::register(cx);
    crate::components::drop_down::register::register(cx);
    crate::components::table::register::register(cx);
    crate::components::tool_btn::register::register(cx);
    crate::components::window::register::register(cx);
    crate::components::select::register::register(cx);
    crate::components::tabbar::register::register(cx);
    crate::components::router::register::register(cx);
    crate::components::menu::register::register(cx);

    crate::components::tab::register::register(cx);
    crate::components::tab::live_design(cx);
}