mod event;
pub mod item;
mod prop;
pub mod sub;

pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    active_event, area, area_ref,
    components::{
        lifecycle::LifeCycle,
        menu::{item::GMenuItemWidgetRefExt, sub::GSubMenuWidgetRefExt},
        traits::{BasicProp, Component, Prop, SlotComponent, SlotProp},
        view::{GView, ViewBasicProp},
    },
    error::Error,
    event_option, event_option_ref, getter_setter_ref, lifecycle,
    prop::{
        manuel::BASIC, ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, DeferWalks, MenuItemMode,
        SlotDrawer, ToStateMap,
    },
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GMenuBase = {{GMenu}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GMenu {
    #[live]
    pub prop: MenuProp,
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
    pub apply_slot_map: ApplySlotMap<MenuState, MenuPart>,
    // --- draw ----------------------
    #[live]
    pub draw_menu: DrawView,
    #[live]
    pub header: GView,
    #[live]
    pub body: GView,
    #[live]
    pub footer: GView,
    #[live]
    pub active: Option<String>,
    #[rust]
    pub item_modes: Vec<MenuItemMode>,
    #[live(true)]
    pub event_key: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: MenuState,
    #[live(true)]
    pub animation_spread: bool,
    #[rust]
    pub defer_walks: DeferWalks,
}

impl WidgetNode for GMenu {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for slot in [&self.header, &self.body, &self.footer] {
            for (_, child) in &slot.children {
                let x = child.uid_to_widget(uid);
                if !x.is_empty() {
                    return x;
                }
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for slot in [&self.header, &self.body, &self.footer] {
            for (_, child) in &slot.children {
                child.find_widgets(path, cached, results);
            }
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.state);
        prop.walk()
    }

    fn area(&self) -> Area {
        self.draw_menu.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_menu.redraw(cx);
        for (visible, slot) in [
            (self.header.visible, &mut self.header),
            (self.body.visible, &mut self.body),
            (self.footer.visible, &mut self.footer),
        ] {
            if visible {
                slot.redraw(cx);
            }
        }
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GMenu {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let state = self.state;
        let prop = self.prop.get(state);

        let _ = self.draw_menu.begin(
            cx,
            walk,
            Layout {
                clip_x: false,
                clip_y: false,
                padding: prop.container.padding,
                align: prop.container.align,
                flow: prop.container.flow,
                spacing: prop.container.spacing,
                ..Default::default()
            },
        );

        let _ = SlotDrawer::new(
            [
                (live_id!(header), (&mut self.header).into()),
                (live_id!(body), (&mut self.body).into()),
                (live_id!(footer), (&mut self.footer).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);

        self.draw_menu.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().menu = true;
        if self.header.visible {
            self.header.handle_event(cx, event, scope);
        }
        if self.body.visible {
            let actions = cx.capture_actions(|cx| self.body.handle_event(cx, event, scope));
            let mut update = None;
            for (index, ((id, child), item_mode)) in self
                .body
                .children
                .iter()
                .zip(self.item_modes.iter())
                .enumerate()
            {
                match item_mode {
                    MenuItemMode::SubMenu(_) => {
                        child.as_gsub_menu().borrow().map(|sub_menu| {
                            if let Some(e) = sub_menu.changed(&actions) {
                                update.replace((id.clone(), e.meta));
                                // if some one clicked sub menu, it just open the body
                            }
                        });
                    }
                    MenuItemMode::MenuItem(_) => {
                        child.as_gmenu_item().borrow().map(|item| {
                            if let Some(e) = item.clicked(&actions) {
                                if e.active {
                                    // means need to change self.active
                                    self.active.replace(e.value);
                                    // do fresh selected
                                    update.replace((id.clone(), e.meta));
                                }
                            }
                        });
                    }
                }
                if update.is_some() {
                    break;
                }
            }
            if let Some((id, e)) = update {
                self.fresh_selected(cx);

                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    MenuEvent::Changed(MenuChanged {
                        meta: e,
                        id,
                        value: self.active.clone()
                    }),
                );
            }
        }

        if self.footer.visible {
            self.footer.handle_event(cx, event, scope);
        }

        let area = self.area();
        let hit = event.hits(cx, area);
        self.handle_widget_event(cx, event, hit, area);
    }
}

impl LiveHook for GMenu {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicProp::live_props();
        self.set_apply_slot_map(
            nodes,
            index,
            [live_id!(basic)],
            [
                (MenuPart::Container, &live_props),
                (MenuPart::Header, &live_props),
                (MenuPart::Body, &live_props),
                (MenuPart::Footer, &live_props),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(MenuState::Basic, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GMenu {
    type Error = Error;

    type State = MenuState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.menu;
        self.prop = prop.clone();
        self.header.prop.basic = self.prop.basic.header;
        self.body.prop.basic = self.prop.basic.body;
        self.footer.prop.basic = self.prop.basic.footer;
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.state;
        let prop = self.prop.get(state);
        self.draw_menu.merge(&prop.container);
        Ok(())
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.header.switch_state(state.into());
        self.body.switch_state(state.into());
        self.footer.switch_state(state.into());
    }

    fn switch_state_with_animation(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();
        for (part, slot) in [
            (MenuPart::Header, &mut self.header),
            (MenuPart::Body, &mut self.body),
            (MenuPart::Footer, &mut self.footer),
        ] {
            crossed_map.remove(&part).map(|map| {
                slot.apply_state_map.merge(map.to_state());
            });

            slot.focus_sync();
        }

        // sync state if is not Basic
        self.prop.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
    }

    sync!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl SlotComponent<MenuState> for GMenu {
    type Part = MenuPart;
}

impl GMenu {
    active_event! {
        active_hover_in: MenuEvent::HoverIn |meta: FingerHoverEvent| => MenuHoverIn { meta },
        active_hover_out: MenuEvent::HoverOut |meta: FingerHoverEvent| => MenuHoverOut { meta }
    }
    event_option! {
        hover_in: MenuEvent::HoverIn => MenuHoverIn,
        hover_out: MenuEvent::HoverOut => MenuHoverOut
    }
    area! {
        area_header, header,
        area_body, body,
        area_footer, footer
    }
    pub fn fresh_selected(&mut self, cx: &mut Cx) {
        // let all children unselected
        for (_index, ((_, child), item_mode)) in self
            .body
            .children
            .iter()
            .zip(self.item_modes.iter())
            .enumerate()
        {
            match item_mode {
                MenuItemMode::SubMenu(_) => {
                    child.as_gsub_menu().borrow_mut().map(|mut sub_menu| {
                        sub_menu.clear_selected(cx);
                    });
                }
                MenuItemMode::MenuItem(_) => {
                    child.as_gmenu_item().borrow_mut().map(|mut item| {
                        item.clear_selected(cx);
                    });
                }
            }
        }
        // then if selected is not None, set the selected item
        if let Some(selected) = self.selected.as_ref() {
            MenuItemMode::find_node(&mut self.body.children, selected, &mut |item| {
                item.as_gmenu_item().borrow_mut().map(|mut item| {
                    item.selected = true;
                    item.render(cx);
                });
            });
        }
    }
    pub fn find_selected(&mut self) {
        for (_, child) in self.body.children.iter() {
            if let Some(child) = child.as_gmenu_item().borrow() {
                self.item_modes.push(MenuItemMode::MenuItem(child.selected));
            } else if let Some(child) = child.as_gsub_menu().borrow() {
                self.item_modes
                    .push(MenuItemMode::SubMenu(child.item_modes.clone()));
            } else {
                panic!("GMenu only allows GMenuItem or GSubMenu as child!");
            }
        }
        self.selected = MenuItemMode::selected(&self.item_modes);
    }

}

impl GMenuRef {
    event_option_ref! {
        hover_in => MenuHoverIn,
        hover_out => MenuHoverOut
    }
    area_ref! {
        area_header,
        area_body,
        area_footer
    }
    getter_setter_ref! {}
}
