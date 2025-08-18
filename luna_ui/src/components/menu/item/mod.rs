mod prop;

use makepad_widgets::*;

use crate::{components::{label::GLabel, lifecycle::LifeCycle, svg::GSvg, view::GView}, prop::{ApplySlotMap, ApplyStateMap}, shader::draw_view::DrawView};

live_design!{
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GMenuItemBase = {{GMenuItem}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GMenuItem {
    #[live]
    pub prop: MenuItemProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_slot_map: ApplySlotMap<>
    // --- draw ----------------------
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    #[live]
    pub extra: GView,
    #[live]
    pub draw_item: DrawView,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: MenuItemState,
}