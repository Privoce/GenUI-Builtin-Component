mod prop;

use makepad_widgets::*;
pub use prop::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        popup::{GPopup, PopupState},
        traits::{BasicStyle, PopupComponent, Style},
    }, error::Error, lifecycle, prop::{manuel::BASIC, ApplyStateMap, Position}, pure_after_apply, set_index, set_scope_path, shader::draw_view::DrawView, themes::conf::Conf
};

live_design! {
    link genui_basic;

    pub GPopupContainerBase = {{GPopupContainer}}{}
}

/// ## This is a container for the popup component.
#[derive(Live, LiveRegister)]
pub struct GPopupContainer {
    #[live]
    pub style: PopupContainerProp,
    #[live]
    pub popup: GPopup,
    #[live]
    pub draw_popup_container: DrawView,
    /// draw list is necessary!!!
    /// because we need to draw the popup on top of everything
    /// although the name of DrawList2d may let you think it's only for 2d list drawing
    /// actually it's for all the drawing that needs to be on top of everything!!!
    #[live]
    draw_list: DrawList2d,
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<PopupState>,
    #[rust]
    pub index: usize,
    #[rust(true)]
    pub sync: bool,
    #[rust]
    pub lifecycle: LifeCycle,
}

impl LiveHook for GPopupContainer {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &PopupContainerBasicStyle::live_props(),
            [live_id!(basic)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(PopupState::Basic, applys);
                }
                _ => {}
            },
        );
    }
}

impl PopupComponent for GPopupContainer {
    type Error = Error;

    type State = PopupState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.popup_container;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let style = self.style.get(self.current_state());
        self.draw_popup_container.merge(&style.into());
        Ok(())
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        self.style.sync(&self.apply_state_map);
    }

    fn current_state(&self) -> Self::State {
        PopupState::Basic
    }

    fn begin(&mut self, cx: &mut Cx2d) -> () {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        let style = self.style.get(self.current_state());
        self.draw_popup_container
            .begin(cx, style.walk(), style.layout());
    }

    fn end(&mut self, cx: &mut Cx2d, _scope: &mut Scope, shift_area: Area, shift: DVec2) -> () {
        self.draw_popup_container.end(cx);
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
    }

    fn redraw(&mut self, cx: &mut Cx) -> () {
        self.draw_list.redraw(cx);
        self.popup.redraw(cx);
    }

    fn draw_popup(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Option<Position>,
        angle_offset: f32,
        redraw: &mut bool,
    ) -> () {
        self.popup
            .draw_popup(cx, scope, position, angle_offset, redraw);
    }

    set_index!();
    lifecycle!();
    set_scope_path!();
}

impl GPopupContainer {
    pub fn area(&self) -> Area {
        self.popup.area
    }
    pub fn draw_container_drawer(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Position,
        proportion: f32,
        redraw: &mut bool,
    ) -> () {
        self.popup
            .draw_container_drawer(cx, scope, position, proportion, redraw);
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        self.popup.handle_event_with(cx, event, scope, sweep_area);
    }

    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.popup.menu_contains_pos(cx, pos)
    }
    pub fn container_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.popup.container_contains_pos(cx, pos)
    }
}
