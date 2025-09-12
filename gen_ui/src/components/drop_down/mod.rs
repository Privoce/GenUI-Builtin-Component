mod event;

pub use event::*;

use std::{cell::RefCell, rc::Rc};

use makepad_widgets::*;

use crate::{
    components::{popup::GPopupContainer, traits::PopupComponent, view::GView},
    prop::{CloseMode, PopupMode, Position, TriggerMode},
    visible,
};

live_design! {
    link genui_basic;

    pub GDropDownBase = {{GDropDown}} {
        style: {
            basic: {
                height: Fit,
                width: Fit,
                padding: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0},
                margin: {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0},
                spacing: 0.0,
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GDropDown {
    #[live]
    pub mode: PopupMode,
    #[live]
    pub close_mode: CloseMode,
    #[deref]
    pub deref_widget: GView,
    #[live]
    pub popup: Option<LivePtr>,
    #[live]
    pub position: Position,
    #[live]
    pub trigger_mode: TriggerMode,
    #[live]
    pub opened: bool,
    #[live(true)]
    pub visible: bool,
    #[live(4.0)]
    pub offset: f32,
    #[live]
    pub offset_x: f32,
    #[live]
    pub offset_y: f32,
    /// if proportion > 1.0, we think that is height/width (dep on position)(TODO: fix this)
    #[live(0.4)]
    pub proportion: f32,
    #[rust(true)]
    pub redraw_flag: bool,
}

#[derive(Default, Clone)]
pub struct PopupMenuGlobal {
    pub map: Rc<RefCell<ComponentMap<LivePtr, GPopupContainer>>>,
}

impl Widget for GDropDown {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        cx.add_nav_stop(self.area(), NavRole::DropDown, Margin::default());

        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.begin(cx);
            match self.mode {
                PopupMode::Popover | PopupMode::ToolTip => {
                    let area = self.area().rect(cx);
                    let angle_offset = self.position.angle_offset(area.size);
                    popup_menu.draw_popup(
                        cx,
                        scope,
                        Some(self.position),
                        angle_offset,
                        &mut self.redraw_flag,
                    );
                    let container = popup_menu.area().rect(cx);
                    let mut shift = match self.position {
                        Position::Bottom => DVec2 {
                            x: -container.size.x / 2.0 + area.size.x / 2.0,
                            y: area.size.y + self.offset as f64,
                        },
                        Position::BottomLeft => DVec2 {
                            x: 0.0,
                            y: area.size.y + self.offset as f64,
                        },
                        Position::BottomRight => DVec2 {
                            x: area.size.x - container.size.x,
                            y: area.size.y + self.offset as f64,
                        },
                        Position::Top => DVec2 {
                            x: -container.size.x / 2.0 + area.size.x / 2.0,
                            y: -self.offset as f64 - container.size.y,
                        },
                        Position::TopLeft => DVec2 {
                            x: 0.0,
                            y: -self.offset as f64 - container.size.y,
                        },
                        Position::TopRight => DVec2 {
                            x: area.size.x - container.size.x,
                            y: -self.offset as f64 - container.size.y,
                        },
                        Position::Left => DVec2 {
                            x: -self.offset as f64 - container.size.x,
                            y: area.size.y / 2.0 - container.size.y / 2.0,
                        },
                        Position::LeftTop => DVec2 {
                            x: -self.offset as f64 - container.size.x,
                            y: 0.0,
                        },
                        Position::LeftBottom => DVec2 {
                            x: -self.offset as f64 - container.size.x,
                            y: 0.0 - container.size.y + area.size.y,
                        },
                        Position::Right => DVec2 {
                            x: area.size.x + self.offset as f64,
                            y: area.size.y / 2.0 - container.size.y / 2.0,
                        },
                        Position::RightTop => DVec2 {
                            x: area.size.x + self.offset as f64,
                            y: 0.0,
                        },
                        Position::RightBottom => DVec2 {
                            x: area.size.x + self.offset as f64,
                            y: 0.0 - container.size.y + area.size.y,
                        },
                    };

                    shift.x += self.offset_x as f64;
                    shift.y += self.offset_y as f64;

                    popup_menu.end(cx, scope, self.area(), shift);
                }

                PopupMode::Dialog => {
                    popup_menu.draw_popup(cx, scope, None, 0.0, &mut false);
                    popup_menu.end(cx, scope, Area::Empty, DVec2::default());
                }
                PopupMode::Drawer => {
                    let _ = popup_menu.draw_container_drawer(
                        cx,
                        scope,
                        self.position,
                        self.proportion,
                        &mut self.redraw_flag,
                    );
                    popup_menu.end(cx, scope, Area::Empty, DVec2::default());
                }
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.opened && self.popup.is_some() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
            popup_menu.handle_event_with(cx, event, scope, self.area());
            if let Event::MouseDown(e) = event {
                match self.mode {
                    PopupMode::Popover | PopupMode::ToolTip => {
                        let is_in = popup_menu.menu_contains_pos(cx, e.abs);
                        self.close_inner(cx, DropDownToggleEvent::Other, is_in);
                    }

                    PopupMode::Dialog | PopupMode::Drawer => {
                        let is_in = popup_menu.container_contains_pos(cx, e.abs);
                        self.close_inner(cx, DropDownToggleEvent::Other, is_in);
                    }
                }
                return;
            }
        }

        match event.hits_with_sweep_area(cx, self.area(), self.area()) {
            // template remove -------------------------------------------------------------------
            // Hit::KeyFocus(_) => {
            //     // self.animator_play(cx, id!(focus.on));
            // }
            // Hit::KeyFocusLost(k_e) => {
            //     // self.toggle_inner(cx, GDropDownToggleKind::KetFocusLost(k_e.clone()), false);
            //     // self.animator_play(cx, id!(hover.off));
            //     // self.draw_view.redraw(cx);
            // }
            // template remove -------------------------------------------------------------------
            Hit::FingerDown(e) => {
                cx.set_key_focus(self.area());
                if self.trigger_mode.is_press() {
                    self.open_inner(cx, DropDownToggleEvent::Press(e));
                }
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(MouseCursor::Hand);
                if self.trigger_mode.is_hover() {
                    self.open_inner(cx, DropDownToggleEvent::Hover(e));
                }
            }
            Hit::FingerHoverOut(f) => {
                cx.set_cursor(MouseCursor::Default);
                if self.trigger_mode.is_hover() {
                    self.close_inner(cx, DropDownToggleEvent::Hover(f), false);
                }
            }
            Hit::FingerUp(e) => {
                if e.is_over && self.trigger_mode.is_click() {
                    self.open_inner(cx, DropDownToggleEvent::Click(e));
                } else {
                    // focus lost
                    self.close_inner(cx, DropDownToggleEvent::Other, false);
                }
            }
            _ => {}
        }
    }
}

impl WidgetNode for GDropDown {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        self.deref_widget.uid_to_widget(uid)
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.deref_widget.find_widgets(path, cached, results);
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.deref_widget.walk(cx)
    }

    fn area(&self) -> Area {
        self.deref_widget.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }

    visible!();
}

impl LiveHook for GDropDown {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
        if self.popup.is_none() || !apply.from.is_from_doc() || !self.visible {
            return;
        }
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut global_map = global.map.borrow_mut();
        global_map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));
        let popup = self.popup.unwrap();
        let popup = global_map.get_or_insert(cx, popup, |cx| {
            GPopupContainer::new_from_ptr(cx, Some(popup))
        });
        // self.close_mode = popup.close_mode;
        // self.mode = popup.mode;
        popup.popup.close_mode = self.close_mode;
    }
}

impl GDropDown {
    /// open the popup only inner control
    fn open_inner(&mut self, cx: &mut Cx, e_kind: DropDownToggleEvent) {
        if self.opened {
            return;
        }
        self.opened = true;
        self.redraw(cx);
        cx.sweep_lock(self.area());
        self.active_toggled(cx, e_kind);
    }
    /// close the popup only inner control
    fn close_inner(&mut self, cx: &mut Cx, e_kind: DropDownToggleEvent, is_in: bool) {
        // here is a quick return to optimize
        if !self.opened {
            return;
        }
        let mut flag = false;
        match self.close_mode {
            CloseMode::Out => {
                if !is_in {
                    flag = true;
                }
            }
            CloseMode::Virtual => {
                flag = false;
            }
        }
        if flag {
            self.opened = false;
            self.redraw(cx);
            cx.sweep_unlock(self.area());
            self.active_toggled(cx, e_kind);
        }
        self.redraw_flag = true;
    }
    fn active_toggled(&mut self, cx: &mut Cx, e_kind: DropDownToggleEvent) {
        cx.widget_action(
            self.widget_uid(),
            self.scope_path.as_ref().unwrap(),
            DropDownEvent::Changed(DropDownChanged {
                meta: e_kind,
                opened: self.opened,
            }),
        );
    }
}
