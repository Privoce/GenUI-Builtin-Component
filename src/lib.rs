use makepad_widgets::Cx;

mod components;
pub mod error;
pub mod event;
pub mod shader;
pub mod themes;
pub mod utils;

pub use components::*;


pub fn live_design(cx: &mut Cx) {
    // components ---------------------------------------------
    crate::components::register::register(cx);
    // shader -------------------------------------------------
    crate::shader::icon_lib::live_design(cx);
    crate::shader::icon_lib::base::live_design(cx);
    crate::shader::icon_lib::arrow::live_design(cx);
    crate::shader::icon_lib::code::live_design(cx);
    crate::shader::icon_lib::emoji::live_design(cx);
    crate::shader::icon_lib::fs::live_design(cx);
    crate::shader::icon_lib::person::live_design(cx);
    crate::shader::icon_lib::relation::live_design(cx);
    crate::shader::icon_lib::state::live_design(cx);
    crate::shader::icon_lib::time::live_design(cx);
    crate::shader::icon_lib::tool::live_design(cx);
    crate::shader::icon_lib::ui::live_design(cx);
    crate::shader::draw_button::live_design(cx);
    crate::shader::draw_view::live_design(cx);
    crate::shader::draw_link::live_design(cx);
    crate::shader::draw_text::live_design(cx);
    crate::shader::draw_radio::live_design(cx);
    crate::shader::draw_check_box::live_design(cx);
    crate::shader::draw_svg::live_design(cx);
    crate::shader::draw_divider::live_design(cx);
    crate::shader::draw_toggle::live_design(cx);
    crate::shader::draw_progress::live_design(cx);
    crate::shader::draw_loading::live_design(cx);
    crate::shader::draw_icon_pixel::live_design(cx);
    crate::shader::draw_split::live_design(cx);
    crate::shader::draw_tab::live_design(cx);
    crate::shader::draw_tab_pane::live_design(cx);
    crate::shader::draw_popup::live_design(cx);
    crate::shader::draw_shader::live_design(cx);
    // export all the components
    crate::components::live_design(cx);
}
