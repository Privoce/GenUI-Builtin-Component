mod event;
mod item;
mod prop;
mod register;
mod sub;

pub use event::*;
pub use item::*;
pub use prop::*;
pub use register::register as menu_register;
pub use sub::*;

use makepad_widgets::*;

use crate::{
    active_event, area, area_ref,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, Style, SlotComponent, SlotStyle},
        view::{GView, ViewBasicStyle},
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
    themes::conf::Conf,
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
    pub style: MenuProp,
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
    // /// control child sub menu/ menu item need to sync the menu theme
    // #[live(true)]
    // pub through: bool, TODO!(sync child menu theme)
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
        let style = self.style.get(self.state);
        style.walk()
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
        let style = self.style.get(state);

        let _ = self.draw_menu.begin(
            cx,
            walk,
            Layout {
                clip_x: false,
                clip_y: false,
                padding: style.container.padding,
                align: style.container.align,
                flow: style.container.flow,
                spacing: style.container.spacing,
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
            // 在body的children中如果激活了某个菜单项，需要给出事件，并保证其他之前选中的菜单项去除激活态
            if !actions.is_empty() {
                let mut active = None;
                self.item_modes.clear();
                for (index, (_id, child)) in self.body.children.iter().enumerate() {
                    let index_chain = vec![index];
                    nested_action(
                        child,
                        cx,
                        &mut self.item_modes,
                        &mut active,
                        &index_chain,
                        &actions,
                    );
                }

                if let Some(e_type) = active {
                    let (meta, _value) = match e_type {
                        MenuActionType::SubMenu(SubMenuChanged { value, meta, .. }) => {
                            (meta, value)
                        }
                        MenuActionType::MenuItem(MenuItemClicked { meta, value, .. }) => {
                            self.active = Some(value.to_string());
                            (meta, value)
                        }
                    };

                    self.active_changed(cx, meta);
                    self.set_target_active(cx);
                }
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

    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        let live_props = ViewBasicStyle::live_props();
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
        // if active is some -> set active or find
        if let Some(active) = self.active.as_ref() {
            self.set_active(cx, Some(active.to_string()));
        } else {
            self.find_active();
        }
    }
}

impl Component for GMenu {
    type Error = Error;

    type State = MenuState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.menu;
        self.style = style.clone();
        self.header.style.basic = self.style.basic.header;
        self.body.style.basic = self.style.basic.body;
        self.footer.style.basic = self.style.basic.footer;
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let state = self.state;
        let style = self.style.get(state);
        self.draw_menu.merge(&style.container);
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
        self.style.sync_slot(&self.apply_slot_map);
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
    pub fn active_changed(&mut self, cx: &mut Cx, meta: Option<FingerUpEvent>) {
        if self.event_open {
            self.scope_path.as_ref().map(|path| {
                cx.widget_action(
                    self.widget_uid(),
                    path,
                    MenuEvent::Changed(MenuChanged {
                        meta,
                        active: self.active.clone(),
                    }),
                );
            });
        }
    }
    event_option! {
        hover_in: MenuEvent::HoverIn => MenuHoverIn,
        hover_out: MenuEvent::HoverOut => MenuHoverOut,
        changed: MenuEvent::Changed => MenuChanged
    }
    area! {
        area_header, header,
        area_body, body,
        area_footer, footer
    }
    /// 从body.children中查找激活的菜单项(MenuItem)
    /// 1. 在body的children中只能存在GSubMenu和GMenuItem，出现其他类型panic!
    /// 2. 激活的菜单项只能是GMenuItem
    /// 3. 如果没有激活的菜单项，返回None，也无需强制指定，将设置权交给使用者/GRouter
    /// 4. 如果有多个激活的菜单项，只会返回第一个
    /// 5. find时确定self.item_mode结构
    /// 6. 若使用者没有制定node的value，则按照索引进行指定，例如第3个subMenu中的第2个MenuItem，则value为"2_1" 2: 3的索引，1: 2的索引
    /// 7. 需要将menu的theme向下同步
    pub fn find_active(&mut self) {
        fn nested_find(
            child: &WidgetRef,
            item_modes: &mut Vec<MenuItemMode>,
            active: &mut Option<bool>,
            index_chain: &Vec<usize>,
        ) -> () {
            if let Some(mut child) = child.as_gsub_menu().borrow_mut() {
                if child.value.is_empty() {
                    child.generate_value(&index_chain);
                }
                let mut sub_menu_mode = vec![];
                // 递归查找子菜单项
                for (sub_index, (_id, sub_child)) in child.body.children.iter().enumerate() {
                    let mut index_chain = index_chain.clone();
                    index_chain.push(sub_index);
                    nested_find(sub_child, &mut sub_menu_mode, active, &index_chain);
                }
                item_modes.push(MenuItemMode::SubMenu {
                    active: child.active,
                    value: child.value.to_string(),
                    items: sub_menu_mode,
                });
            } else if let Some(mut child) = child.as_gmenu_item().borrow_mut() {
                if child.value.is_empty() {
                    child.generate_value(&index_chain);
                }
                item_modes.push(MenuItemMode::MenuItem {
                    value: child.value.to_string(),
                    active: child.active,
                });
                if child.active && active.is_none() {
                    active.replace(true);
                }
            } else {
                panic!("GMenu only allows GMenuItem or GSubMenu as child!");
            }
        }

        let mut active = None;
        self.item_modes.clear();
        if self.body.visible {
            for (index, (_id, child)) in self.body.children.iter().enumerate() {
                let index_chain = vec![index];
                nested_find(child, &mut self.item_modes, &mut active, &index_chain);
            }
        }
    }

    /// 设置激活状态
    /// 1. 如果active为None，表示不激活任何菜单项，需要将所有GMenuItem的active设置为false
    /// 2. 如果active为Some(value), 我们可以通过active知道是哪个菜单项被激活，不同的是SubMenu虽然也能被激活但只是展开状态，而MenuItem则是真正激活态
    pub fn set_active(&mut self, cx: &mut Cx, active: Option<String>) {
        self.item_modes.clear();
        // update children
        for (index, (_id, child)) in self.body.children.iter().enumerate() {
            let index_chain = vec![index];
            handle_nested(
                cx,
                child,
                &active,
                &index_chain,
                &mut self.item_modes,
                false,
            );
        }

        // cover active
        self.active = active;
    }
    // 在设置过self.active后，调用此函数，确保self.active的菜单项被激活
    pub fn set_target_active(&mut self, cx: &mut Cx) {
        for (index, (_id, child)) in self.body.children.iter().enumerate() {
            let index_chain = vec![index];
            // nested_set(child, cx, &self.active, &index_chain);
            handle_nested(cx, child, &self.active, &index_chain, &mut vec![], true);
        }
    }
}

impl GMenuRef {
    event_option_ref! {
        hover_in => MenuHoverIn,
        hover_out => MenuHoverOut,
        changed => MenuChanged
    }
    area_ref! {
        area_header,
        area_body,
        area_footer
    }
    getter_setter_ref! {}
}

fn handle_nested(
    cx: &mut Cx,
    child: &WidgetRef,
    active: &Option<String>,
    index_chain: &Vec<usize>,
    item_modes: &mut Vec<MenuItemMode>,
    is_action: bool,
) -> bool {
    if let Some(mut child) = child.as_gsub_menu().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }
        if !is_action {
            if let Some(active) = active {
                let active = child.value.eq(active);
                let _ = child.set_active(cx, active);
            }
        }
        let mut sub_menu_mode = vec![];
        let mut active_sub = false;
        for (sub_index, (_id, sub_child)) in child.body.children.iter().enumerate() {
            let mut index_chain = index_chain.clone();
            index_chain.push(sub_index);
            let is_active = handle_nested(
                cx,
                sub_child,
                active,
                &index_chain,
                &mut sub_menu_mode,
                is_action,
            );
            active_sub |= is_active;
        }
        child.active = active_sub;
        if !is_action {
            item_modes.push(MenuItemMode::SubMenu {
                active: child.active,
                value: child.value.to_string(),
                items: sub_menu_mode,
            });
        }
        return child.active;
    } else if let Some(mut child) = child.as_gmenu_item().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }
        if let Some(active) = active {
            let active = child.value.eq(active);
            let _ = child.set_active(cx, active);
        } else {
            let _ = child.set_active(cx, false);
        }
        if !is_action {
            item_modes.push(MenuItemMode::MenuItem {
                value: child.value.to_string(),
                active: child.active,
            });
        }
        return child.active;
    } else {
        panic!("GMenu only allows GMenuItem or GSubMenu as child!");
    }
}

fn nested_action(
    child: &WidgetRef,
    cx: &mut Cx,
    item_modes: &mut Vec<MenuItemMode>,
    active: &mut Option<MenuActionType>,
    index_chain: &Vec<usize>,
    actions: &Vec<Box<dyn ActionTrait>>,
) {
    if let Some(mut child) = child.as_gsub_menu().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }
        let mut sub_menu_mode = vec![];
        // 递归查找子菜单项
        for (sub_index, (_id, sub_child)) in child.body.children.iter().enumerate() {
            let mut index_chain = index_chain.clone();
            index_chain.push(sub_index);
            if let Some(e) = child.changed(actions) {
                active.replace(MenuActionType::SubMenu(e));
            } else {
                nested_action(
                    sub_child,
                    cx,
                    &mut sub_menu_mode,
                    active,
                    &index_chain,
                    actions,
                );
            }
        }
        item_modes.push(MenuItemMode::SubMenu {
            active: child.active,
            value: child.value.to_string(),
            items: sub_menu_mode,
        });
    } else if let Some(mut child) = child.as_gmenu_item().borrow_mut() {
        if child.value.is_empty() {
            child.generate_value(&index_chain);
        }
        item_modes.push(MenuItemMode::MenuItem {
            value: child.value.to_string(),
            active: child.active,
        });
        if active.is_none() {
            if let Some(e) = child.clicked(actions) {
                active.replace(MenuActionType::MenuItem(e));
            }
        }
    } else {
        panic!("GMenu only allows GMenuItem or GSubMenu as child!");
    }
}
